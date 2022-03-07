import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { Vault, Cipher, ByteData, AESKey, ECDHKeyPair, ECDHPublicKey } from '../../asset-pipeline/cryptography/vault'
import { VaultClient } from '../../asset-pipeline/api.client';
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
import { CreateSecretsRequest, Secret, ServiceAccount, ServiceAccountSecrets } from '../../asset-pipeline/api';


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

    constructor() {
        super()

        // Attach to the button
        const newVaultButton = document.getElementById('new-secret')
        newVaultButton.addEventListener('click', async event => {
            this.open = true
        })

        // So you want to create a secret
        const createSecretButton = this.querySelector('#create-secret')
        createSecretButton.addEventListener('click', async event => {
            event.preventDefault()
            this.createSecret()
        })
    }

    async createSecret() {

        this.secretNameInput = document.getElementById('secret-name') as HTMLInputElement
        this.secretValueInput = document.getElementById('secret-value') as HTMLTextAreaElement
        this.blindIndexInput = document.getElementById('name-blind-index') as HTMLInputElement
        this.secretForm = document.getElementById('add-secret-form') as HTMLFormElement
        this.vaultIdInput = document.getElementById('vault-id') as HTMLInputElement

        const vaultId = parseInt(this.vaultIdInput.value)

        if (this.secretForm.checkValidity()) {
            try {
                const plaintextName = this.secretNameInput.value
                const plaintextValue = this.secretValueInput.value

                const vaultKey = await this.decryptSymmetricVaultKey()

                const cipherName = await vaultKey.encrypt(ByteData.fromText(plaintextName))
                const cipherValue = await vaultKey.encrypt(ByteData.fromText(plaintextValue))
                const nameBlindIndex = await Vault.blindIndex(plaintextName, vaultId)

                await this.sendSecretsToServiceAccounts(vaultId, 
                    plaintextName, plaintextValue, cipherName, cipherValue, nameBlindIndex)

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
        nameBlindIndex : ByteData) {

        // Call back to the server and get the vault details including the
        // connected service accounts
        const call = this.getVaultClient().getVault({
                vaultId: vaultId
            }, this.getRpcOptions()
        )

        const vault = await call.response
        const createServiceRequest = await this.deriveServiceAccountSecrets(
            vault.serviceAccounts, 
            plaintextName, plaintextValue, nameBlindIndex.b64)
        
        const createCall = this.getVaultClient().createSecrets(createServiceRequest, this.getRpcOptions())
        await createCall.response
        await this.submitForm(cipherName, cipherValue, nameBlindIndex)
    }

    async deriveServiceAccountSecrets(serviceAccounts: ServiceAccount[],
        plaintextName: string, plaintextValue: string, blindIndex: string)  : Promise<CreateSecretsRequest> {

        const etherealKeyPair = await ECDHKeyPair.fromRandom()
        const etherealPublicKeyData = await etherealKeyPair.publicKey.export()
        const etherealPublicKeyBase64 = etherealPublicKeyData.b64

        const createSecretsRequest : CreateSecretsRequest = {
            accountSecrets: []
        }

        for(var index = 0; index < serviceAccounts.length; index ++) {
            const serviceAccount = serviceAccounts[index]
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

customElements.define('new-secret', NewSecret);