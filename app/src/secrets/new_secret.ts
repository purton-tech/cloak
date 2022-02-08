import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { Vault, Cipher, ByteData, AESKey, ECDHKeyPair, ECDHPublicKey } from '../../asset-pipeline/cryptography/vault'
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import * as grpcWeb from 'grpc-web';
import { GetVaultRequest, GetVaultResponse, CreateSecretsRequest, CreateSecretsResponse, Secret, ServiceAccount, ServiceAccountSecrets } from '../../asset-pipeline/api_pb';


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
    private secretValueInput: HTMLInputElement
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
        this.secretValueInput = document.getElementById('secret-value') as HTMLInputElement
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

        const request = new GetVaultRequest();
        request.setVaultId(vaultId)
        // Call back to the server and get the vault details including the
        // connected service accounts
        this.getVaultClient().getVault(request,
    
            // Important, Envoy will pick this up then authorise our request
            { 'authentication-type': 'cookie' },
    
            async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
                if (err) {
                    console.log('Error code: ' + err.code + ' "' + err.message + '"');
                } else {
    
                    const createServiceRequest = await this.deriveServiceAccountSecrets(
                        vault.getServiceAccountsList(), 
                        plaintextName, plaintextValue, nameBlindIndex.b64)
    
                    this.getVaultClient().createSecrets(createServiceRequest,
    
                        // Important, Envoy will pick this up then authorise our request
                        { 'authentication-type': 'cookie' },
                        
                        async (err: grpcWeb.RpcError, vault: CreateSecretsResponse) => {
                            if (err) {
                                console.log('Error code: ' + err.code + ' "' + err.message + '"');
                            } else {
                                await this.submitForm(cipherName, cipherValue, nameBlindIndex)
                            }
                        }
                    )
                }
            }
        )
    }


    async deriveServiceAccountSecrets(serviceAccounts: ServiceAccount[],
        plaintextName: string, plaintextValue: string, blindIndex: string)  : Promise<CreateSecretsRequest> {

        const createSecretsRequest = new CreateSecretsRequest()

        const etherealKeyPair = await ECDHKeyPair.fromRandom()
        const etherealPublicKeyData = await etherealKeyPair.publicKey.export()
        const etherealPublicKeyBase64 = etherealPublicKeyData.b64

        for(var index = 0; index < serviceAccounts.length; index ++) {
            const serviceAccount = serviceAccounts[index]
            // Get a key agreement between the service account ECDH private key and the vault ECDH public key.
            const serviceAccountECDHPublicKeyData = 
                ByteData.fromB64(serviceAccount.getPublicEcdhKey())
            const serviceAccountECDHPublicKey: ECDHPublicKey = 
                await ECDHPublicKey.import(serviceAccountECDHPublicKeyData)
            const aesKeyAgreement: AESKey = 
                await etherealKeyPair.privateKey.deriveAESKey(serviceAccountECDHPublicKey)
        
            // Associated Data
            const associatedData = new ByteData(new Uint8Array(4))
            const view = new DataView(associatedData.arr.buffer)
            view.setUint32(0, serviceAccount.getServiceAccountId(), true /* littleEndian */);
        
            const newEncryptedName = await aesKeyAgreement.aeadEncrypt(ByteData.fromText(plaintextName), 
                associatedData)
            const newEncryptedValue = await aesKeyAgreement.aeadEncrypt(ByteData.fromText(plaintextValue), 
                associatedData)
        
            const secret = new Secret()
            secret.setEncryptedSecretValue(newEncryptedValue.string)
            secret.setNameBlindIndex(blindIndex)
            secret.setEncryptedName(newEncryptedName.string)
        
            const serviceAccountSecrets = new ServiceAccountSecrets()
            serviceAccountSecrets.setServiceAccountId(serviceAccount.getServiceAccountId())
            serviceAccountSecrets.addSecrets(secret)
            serviceAccountSecrets.setPublicEcdhKey(etherealPublicKeyBase64)

            createSecretsRequest.addAccountSecrets(serviceAccountSecrets)

            console.log(createSecretsRequest.getAccountSecretsList().length)
        }

        return createSecretsRequest
    }

    private getVaultClient() : VaultClient {
        return new VaultClient(window.location.protocol
            + '//' + window.location.host, null, null)
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