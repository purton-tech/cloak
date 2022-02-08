import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { Vault, ECDHPublicKey, ECDHKeyPair, AESKey, Cipher, ByteData } from '../../asset-pipeline/cryptography/vault'
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import * as grpcWeb from 'grpc-web';
import { GetVaultRequest, GetVaultResponse, Secret, ServiceAccountSecrets, CreateSecretsRequest, CreateSecretsResponse } from '../../asset-pipeline/api_pb';


class ConnectAccount extends SideDrawer {

    constructor() {
        super()

        const serviceAccountId = parseInt(this.getAttribute('service-account-id'))

        let newAccountButton = document.getElementById('service-account-row-' + serviceAccountId)
        newAccountButton.addEventListener('click', async event => {
            this.open = true
        })

        let connectButton = this.querySelector('#connect-to-vault-' + serviceAccountId)
        connectButton.addEventListener('click', async event => {
            await this.handleConnect(serviceAccountId)
        })
    }

    async handleConnect(serviceAccountId: number) {
        const vaultSelect = document.getElementById('vault-select-' + serviceAccountId)
        const ecdhKey = document.getElementById('service-account-public-key-' + serviceAccountId)
    
        if (vaultSelect instanceof HTMLSelectElement && vaultSelect.selectedIndex != 0
            && ecdhKey instanceof HTMLInputElement) {
    
            const vaultClient = this.getVaultClient() 
    
            const request = new GetVaultRequest();
            const vaultId = parseInt(vaultSelect.options[vaultSelect.selectedIndex].value)
            request.setVaultId(vaultId)

            const serviceAccountECDHPublicKey = await ECDHPublicKey.import(ByteData.fromB64(ecdhKey.value))
    
            // Call back to the server
            vaultClient.getVault(request,
    
                // Important, Envoy will pick this up then authorise our request
                { 'authentication-type': 'cookie' },
    
                async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
                    if (err) {
                        console.log('Error code: ' + err.code + ' "' + err.message + '"');
                    } else {
                        await this.transferSecretsToServiceAccount(vault, serviceAccountId, 
                            vaultId, serviceAccountECDHPublicKey)
                    }
                }
            )
        }
    }

    async transferSecretsToServiceAccount(vault: GetVaultResponse, serviceAccountId: number, 
        vaultId: number, serviceAccountECDHPublicKey : ECDHPublicKey) {
        console.log(vault)

        const wrappedVaultKey = Cipher.fromString(vault.getUserVaultEncryptedVaultKey())
        const ecdhUserPublicKey = await ECDHPublicKey.import(ByteData.fromB64(vault.getUserVaultPublicEcdhKey()))
        const vaultKey = await Vault.decryptVaultKey(wrappedVaultKey, ecdhUserPublicKey)

        //
        const etherealKeyPair = await ECDHKeyPair.fromRandom()
        const aesKey = await etherealKeyPair.privateKey.deriveAESKey(serviceAccountECDHPublicKey)

        const rencryptedSecrets = await this.decryptAndRencryptSecrets(vault, 
            vaultKey, aesKey, serviceAccountId)

        // Send the encrypted payload back to the server
        const request = new CreateSecretsRequest()
        const serviceAccountSecrets = new ServiceAccountSecrets()
        serviceAccountSecrets.setServiceAccountId(serviceAccountId)
        serviceAccountSecrets.setSecretsList(rencryptedSecrets)
        const publicKeyExport = await etherealKeyPair.publicKey.export()
        serviceAccountSecrets.setPublicEcdhKey(publicKeyExport.b64)
        request.addAccountSecrets(serviceAccountSecrets)

        const connectForm = document.getElementById('service-account-form-' + serviceAccountId)
        const connectFormVaultId = document.getElementById('service-account-form-vault-id-' + serviceAccountId)
    
        const vaultClient = this.getVaultClient() 

        if (connectForm instanceof HTMLFormElement && connectFormVaultId instanceof HTMLInputElement) {
        
            const call = vaultClient.createSecrets(request,

                // Important, Envoy will pick this up then authorise our request
                { 'authentication-type': 'cookie' },

                async (err: grpcWeb.RpcError, serviceAccount: CreateSecretsResponse) => {
                    if (err) {
                        console.log('Error code: ' + err.code + ' "' + err.message + '"');
                    } else {
                        // Assuming that all worked, connect the account to the vault
                        connectFormVaultId.value = '' + vaultId
                        connectForm.submit()
                    }
                }
            )
        }
    }

    async decryptAndRencryptSecrets(vault: GetVaultResponse, vaultKey: AESKey, 
        agreementKey: AESKey, serviceAccountId: number) : Promise<Secret[]> {

        const associatedData = this.getAssociatedData(serviceAccountId)

        // Process the secrets - re-encrypt them with the agreement key.
        const secretList = vault.getSecretsList()
        for await (var secret of secretList) {
            const cipherName = Cipher.fromString(secret.getEncryptedName())
            const plaintextName: ByteData = await vaultKey.decrypt(cipherName)
            const newEncryptedName = await agreementKey.aeadEncrypt(plaintextName, associatedData)

            secret.setEncryptedName(newEncryptedName.string)
            const cipherValue = Cipher.fromString(secret.getEncryptedSecretValue())
            const plaintextValue: ByteData = await vaultKey.decrypt(cipherValue)

            const newEncryptedValue = await agreementKey.aeadEncrypt(plaintextValue, associatedData)
            secret.setEncryptedSecretValue(newEncryptedValue.string)
        }

        return secretList
    }

    private getAssociatedData(serviceAccountId: number) : ByteData {
        const associatedData = new Uint8Array(4)
        const view = new DataView(associatedData.buffer)
        view.setUint32(0, serviceAccountId, true /* littleEndian */);
        return new ByteData(associatedData)
    }

    private getVaultClient() : VaultClient {
        return new VaultClient(window.location.protocol
            + '//' + window.location.host, null, null)
    }
}

customElements.define('connect-account', ConnectAccount);