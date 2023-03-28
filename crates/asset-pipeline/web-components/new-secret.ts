import { Vault, Cipher, ByteData, AESKey, ECDHKeyPair, ECDHPublicKey } from '../cryptography/vault'
import { VaultClient } from '../api.client';
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
import { CreateSecretsRequest, Secret, ServiceAccount, ServiceAccountSecrets } from '../api';
import { SideDrawer } from './side-drawer';

/**
 * Alice creates a new secret for a vault.
 * 
 * - We retrieve the User Vault. 
 * - Alices creates an ECDH agreement between her private key and the User Vault public key.
 * - Alice can now decrypt the Vault encryption key using the derived key.
 * - Alice can use the Vault encryption key to encrypt the secrets.
 */
class NewSecret extends SideDrawer {

    private secretNameInput: HTMLInputElement
    private secretValueInput: HTMLTextAreaElement
    private blindIndexInput: HTMLInputElement
    private secretForm: HTMLFormElement
    private vaultIdInput: HTMLInputElement
    private environmentIdSelect: HTMLSelectElement

    constructor() {
        super()

        const createSecretButton = this.querySelector('#create-secret')

        if(createSecretButton) {
    
            // So you want to create a secret
            createSecretButton.addEventListener('click', async event => {
                event.preventDefault()
                this.createSecret()
            })
        } else {
            console.error('Could not find required elements')
        }
    }

    async createSecret() {

        this.secretNameInput = document.getElementById('secret-name') as HTMLInputElement
        this.secretValueInput = document.getElementById('secret-value') as HTMLTextAreaElement
        this.blindIndexInput = document.getElementById('name-blind-index') as HTMLInputElement
        this.secretForm = document.getElementById('add-secret-form') as HTMLFormElement
        this.vaultIdInput = document.getElementById('vault-id') as HTMLInputElement
        this.environmentIdSelect = document.getElementById('environment_id') as HTMLSelectElement

        const vaultId = parseInt(this.vaultIdInput.value)

        if (this.secretForm.reportValidity()) {
            try {
                const plaintextName = this.secretNameInput.value
                const plaintextValue = this.secretValueInput.value

                const vaultKey = await this.decryptSymmetricVaultKey()

                const cipherName = await vaultKey.encrypt(ByteData.fromText(plaintextName))
                const cipherValue = await vaultKey.encrypt(ByteData.fromText(plaintextValue))
                const nameBlindIndex = await Vault.blindIndex(plaintextName, vaultId)

                const environmentId = parseInt(this.environmentIdSelect.value)

                await this.sendSecretsToServiceAccounts(vaultId, 
                    plaintextName, plaintextValue, cipherName, cipherValue, 
                    nameBlindIndex, environmentId)

            } catch (err) {
                if (err instanceof Error) {
                    console.log(err.message)
                }
            }
        }
    }

    async submitForm(nameCipher: Cipher, valueCipher: Cipher, nameBlindIndex: ByteData) {
        this.secretNameInput.value = nameCipher.string
        this.secretValueInput.value = valueCipher.string
        this.blindIndexInput.value = nameBlindIndex.b64
        this.secretForm.submit()
    }

    async sendSecretsToServiceAccounts(vaultId : number, plaintextName : string, 
        plaintextValue : string, cipherName : Cipher, cipherValue : Cipher,
        nameBlindIndex : ByteData, environmentId : number) {

        // Call back to the server and get the vault details including the
        // connected service accounts
        const call = this.getVaultClient().getVault({
                vaultId: vaultId
            }, this.getRpcOptions()
        )

        const vault = await call.response
        const createServiceRequest = await this.deriveServiceAccountSecrets(
            vault.serviceAccounts, 
            plaintextName, plaintextValue, nameBlindIndex.b64, environmentId)
            
        const createCall = this.getVaultClient().createSecrets(createServiceRequest, this.getRpcOptions())
        await createCall.response
        await this.submitForm(cipherName, cipherValue, nameBlindIndex)
    }

    async deriveServiceAccountSecrets(serviceAccounts: ServiceAccount[],
        plaintextName: string, plaintextValue: string, 
        blindIndex: string, environmentId : number)  : Promise<CreateSecretsRequest> {

        const etherealKeyPair = await ECDHKeyPair.fromRandom()
        const etherealPublicKeyData = await etherealKeyPair.publicKey.export()
        const etherealPublicKeyBase64 = etherealPublicKeyData.b64

        const createSecretsRequest : CreateSecretsRequest = {
            accountSecrets: []
        }

        for(var index = 0; index < serviceAccounts.length; index ++) {
            const serviceAccount = serviceAccounts[index]

            // Only send the secret if the service account environment corresponds to the
            // environment this new secret is being added to.
            if(environmentId == serviceAccount.environmentId) {
                // Get a key agreement between the service account ECDH private key and the vault ECDH public key.
                const serviceAccountECDHPublicKeyData = 
                    ByteData.fromB64(serviceAccount.publicEcdhKey)
                const serviceAccountECDHPublicKey: ECDHPublicKey = 
                    await ECDHPublicKey.import(serviceAccountECDHPublicKeyData)
                const aesKeyAgreement: AESKey = 
                    await etherealKeyPair.privateKey.deriveAESKey(serviceAccountECDHPublicKey)
            
                // Associated Data
                const associatedData = new ByteData(new Uint8Array(4))
                const view = new DataView(associatedData.arr.buffer)
                view.setUint32(0, serviceAccount.serviceAccountId, true /* littleEndian */);
            
                const newEncryptedName = await aesKeyAgreement.aeadEncrypt(ByteData.fromText(plaintextName), 
                    associatedData)
                const newEncryptedValue = await aesKeyAgreement.aeadEncrypt(ByteData.fromText(plaintextValue), 
                    associatedData)
            
                const secret : Secret = {
                    encryptedName: newEncryptedName.string,
                    encryptedSecretValue: newEncryptedValue.string,
                    environmentId: environmentId,
                    nameBlindIndex: blindIndex
                }
            
                const serviceAccountSecrets : ServiceAccountSecrets = {
                    serviceAccountId: serviceAccount.serviceAccountId,
                    secrets: [
                        secret
                    ],
                    publicEcdhKey: etherealPublicKeyBase64
                }
    
                createSecretsRequest.accountSecrets.push(serviceAccountSecrets)
            }
        }

        console.log(createSecretsRequest.accountSecrets.length)

        return createSecretsRequest
    }

    private getVaultClient(): VaultClient {
        let transport = new GrpcWebFetchTransport({
            baseUrl: window.location.protocol + '//' + window.location.host
        });
        return new VaultClient(transport)
    }

    private getRpcOptions() : RpcOptions {
        const meta = {}
        meta['authentication-type'] = 'cookie';

        let options: RpcOptions = {
            meta: meta
        }
        return options
    }

    private async decryptSymmetricVaultKey(): Promise<AESKey> {
        const ecdhPublicKeyInput = this.querySelector('#user-vault-ecdh-public-key') as HTMLInputElement
        const encryptedVaultKeyInput = this.querySelector('#encrypted-vault-key') as HTMLInputElement
        const vaultKeyCipher = Cipher.fromString(encryptedVaultKeyInput.value)
        const ecdhPublicKey = await ECDHPublicKey.import(ByteData.fromB64(ecdhPublicKeyInput.value))
        return await Vault.decryptVaultKey(vaultKeyCipher, ecdhPublicKey)
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('new-secret', NewSecret);
    }
})