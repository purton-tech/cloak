import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { Vault, ECDHPublicKey, ECDHKeyPair, AESKey, Cipher, ByteData } from '../../asset-pipeline/cryptography/vault'
import { VaultClient } from '../../asset-pipeline/api.client';
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
import { GetVaultResponse, Secret } from '../../asset-pipeline/api';


class ConnectAccount extends SideDrawer {

    constructor() {
        super()

        const serviceAccountAttr = this.getAttribute('service-account-id')

        if(serviceAccountAttr) {
            const serviceAccountId = parseInt(serviceAccountAttr)
            let connectButton = this.querySelector('#connect-to-vault-' + serviceAccountId)
            let newAccountButton = document.getElementById('service-account-row-' + serviceAccountId)

            if(connectButton && newAccountButton) {
    
                newAccountButton.addEventListener('click', async event => {
                    this.open = true
                })
        
                connectButton.addEventListener('click', async event => {
                    await this.handleConnect(serviceAccountId)
                })
            } else {
                console.error('Could not fund required elements')
            }
        } else {
            console.error('Could not fund required elements')
        }
    }

    async handleConnect(serviceAccountId: number) {
        const vaultSelect = document.getElementById('vault-select-' + serviceAccountId)
        const ecdhKey = document.getElementById('service-account-public-key-' + serviceAccountId)


        if (vaultSelect instanceof HTMLSelectElement && vaultSelect.selectedIndex != 0
            && ecdhKey instanceof HTMLInputElement) {

            const vaultClient = this.getVaultClient()

            const selectValue = vaultSelect.options[vaultSelect.selectedIndex].value
            const vaultId = parseInt(selectValue.split(':')[0])
            const environmentId = parseInt(selectValue.split(':')[1])

            const serviceAccountECDHPublicKey = await ECDHPublicKey.import(ByteData.fromB64(ecdhKey.value))

            // Call back to the server
            const call = vaultClient.getVault(
                {
                    vaultId: vaultId
                },
                this.getRpcOptions()
            )
            const vault: GetVaultResponse = await call.response

            await this.transferSecretsToServiceAccount(vault, serviceAccountId,
                vaultId, environmentId, serviceAccountECDHPublicKey)
        } else {
            console.log("Didn't find needed element")
            console.log('vault select = ' + vaultSelect)
            console.log('ecdhKey  = ' + ecdhKey)
        }
    }

    async transferSecretsToServiceAccount(vault: GetVaultResponse, serviceAccountId: number,
        vaultId: number, environmentId: number, serviceAccountECDHPublicKey: ECDHPublicKey) {

        const wrappedVaultKey = Cipher.fromString(vault.userVaultEncryptedVaultKey)
        const ecdhUserPublicKey = await ECDHPublicKey.import(ByteData.fromB64(vault.userVaultPublicEcdhKey))
        const vaultKey = await Vault.decryptVaultKey(wrappedVaultKey, ecdhUserPublicKey)

        //
        const etherealKeyPair = await ECDHKeyPair.fromRandom()
        const aesKey = await etherealKeyPair.privateKey.deriveAESKey(serviceAccountECDHPublicKey)

        const rencryptedSecrets = await this.decryptAndRencryptSecrets(vault,
            vaultKey, aesKey, serviceAccountId, environmentId)

        // Send the encrypted payload back to the server
        const publicKeyExport = await etherealKeyPair.publicKey.export()

        const connectForm = document.getElementById('service-account-form-' + serviceAccountId)
        const connectFormVaultId = document.getElementById('service-account-form-vault-id-' + serviceAccountId)
        const connectFormEnvironmentId = document.getElementById('service-account-form-environment-id-' + serviceAccountId)

        const vaultClient = this.getVaultClient()

        if (
            connectForm instanceof HTMLFormElement &&
            connectFormVaultId instanceof HTMLInputElement &&
            connectFormEnvironmentId instanceof HTMLInputElement
        ) {

            const call = vaultClient.createSecrets({
                accountSecrets: [
                    {
                        publicEcdhKey: publicKeyExport.b64,
                        serviceAccountId: serviceAccountId,
                        secrets: rencryptedSecrets
                    }
                ]
            }, this.getRpcOptions())
            const response = await call.response

            // Assuming that all worked, connect the account to the vault
            connectFormVaultId.value = '' + vaultId
            connectFormEnvironmentId.value = '' + environmentId
            connectForm.submit()
        }
    }

    async decryptAndRencryptSecrets(vault: GetVaultResponse, vaultKey: AESKey,
        agreementKey: AESKey, serviceAccountId: number, environmentId : number): Promise<Secret[]> {

        const associatedData = this.getAssociatedData(serviceAccountId)

        var secretList: Array<Secret> = []

        // Process the secrets - re-encrypt them with the agreement key.
        for await (var secret of vault.secrets) {
            if(secret.environmentId == environmentId) {
                const cipherName = Cipher.fromString(secret.encryptedName)
                const plaintextName: ByteData = await vaultKey.decrypt(cipherName)
                const newEncryptedName = await agreementKey.aeadEncrypt(plaintextName, associatedData)
    
                secret.encryptedName = newEncryptedName.string
                secret.environmentId = environmentId
                const cipherValue = Cipher.fromString(secret.encryptedSecretValue)
                const plaintextValue: ByteData = await vaultKey.decrypt(cipherValue)
    
                const newEncryptedValue = await agreementKey.aeadEncrypt(plaintextValue, associatedData)
                secret.encryptedSecretValue = newEncryptedValue.string
    
                secretList.push(secret)
            }            
        }
        return secretList
    }

    private getAssociatedData(serviceAccountId: number): ByteData {
        const associatedData = new Uint8Array(4)
        const view = new DataView(associatedData.buffer)
        view.setUint32(0, serviceAccountId, true /* littleEndian */);
        return new ByteData(associatedData)
    }

    private getVaultClient(): VaultClient {
        let transport = new GrpcWebFetchTransport({
            baseUrl: window.location.protocol + '//' + window.location.host
        });
        return new VaultClient(transport)
    }

    private getRpcOptions(): RpcOptions {
        const meta = {}
        meta['authentication-type'] = 'cookie';

        let options: RpcOptions = {
            meta: meta
        }
        return options
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('connect-account', ConnectAccount);
    }
})