import { SideDrawer } from '../../asset-pipeline/side-drawer'
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

        const enc = new TextEncoder(); // always utf-8
        const vaultId = parseInt(this.vaultIdInput.value)

        if (this.secretForm.checkValidity()) {
            try {
                const plaintextName = this.secretNameInput.value
                const plaintextValue = this.secretValueInput.value

                const vaultKey = await this.decryptSymmetricVaultKey()

                const cipherName = await vaultKey.encrypt(new ByteData(enc.encode(plaintextName)))
                const cipherValue = await vaultKey.encrypt(new ByteData(enc.encode(plaintextValue)))
                const nameBlindIndex = await Vault.blindIndex(plaintextName, vaultId)

                await this.sendSecretsToServiceAccounts()

                await this.submitForm(cipherName, cipherValue, nameBlindIndex)

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

    async sendSecretsToServiceAccounts() {
        // For each service account public key
        // generate a temporary ECDH keypair
        // derive the secret between the keypair and the service account
        // Send to the server.
    }

    // Generate an agreement with the current user
    async decryptSymmetricVaultKey(): Promise<AESKey> {
        const ecdhPublicKeyInput = this.querySelector('#user-vault-ecdh-public-key') as HTMLInputElement
        const encryptedVaultKeyInput = this.querySelector('#encrypted-vault-key') as HTMLInputElement
        const vaultKeyCipher = Cipher.fromString(encryptedVaultKeyInput.value)

        console.log('here1')

        const ecdhPublicKey = await ECDHPublicKey.import(ByteData.fromB64(ecdhPublicKeyInput.value))

        console.log((await ecdhPublicKey.export()).b64)
        const aliceECDHKeyPair = await ECDHKeyPair.fromBarricade()
        console.log((await aliceECDHKeyPair.publicKey.export()).b64)
        console.log(vaultKeyCipher)

        return await aliceECDHKeyPair.privateKey.unwrapKey(vaultKeyCipher, ecdhPublicKey)
    }
}

customElements.define('new-secret', NewSecret);

async function encryptSecretToConnectedServiceAccounts(vaultKey: CryptoKey, vaultId: number,
    secretName: string, secretValue: string, secretForm : HTMLFormElement,
    secretNameInput : HTMLInputElement, secretValueInput : HTMLInputElement,
    blindIndexInput : HTMLInputElement,
    nameCipher: Cipher, valueCipher: Cipher) {

    const vaultClient = new VaultClient(window.location.protocol
        + '//' + window.location.host, null, null);

    const request = new GetVaultRequest();
    request.setVaultId(vaultId)

    // Call back to the server and get the vault details including the
    // connected service accounts
    vaultClient.getVault(request,

        // Important, Envoy will pick this up then authorise our request
        { 'authentication-type': 'cookie' },

        async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
            if (err) {
                console.log('Error code: ' + err.code + ' "' + err.message + '"');
            } else {
                const cipher = Cipher.fromString(vault.getEncryptedVaultPrivateEcdhKey())
                const vaultECDHPrivateKey = await Vault.unwrapECDHKeyPair(cipher, vaultKey)
                const nameBlindIndex = await Vault.blindIndex(secretName, vaultId)

                const createServiceRequest = await deriveServiceAccountSecrets(
                    vault.getServiceAccountsList(), vaultECDHPrivateKey, 
                    secretName, secretValue, nameBlindIndex.b64)

                vaultClient.createSecrets(createServiceRequest,

                    // Important, Envoy will pick this up then authorise our request
                    { 'authentication-type': 'cookie' },
                    
                    async (err: grpcWeb.RpcError, vault: CreateSecretsResponse) => {
                        if (err) {
                            console.log('Error code: ' + err.code + ' "' + err.message + '"');
                        } else {

                            secretNameInput.value = nameCipher.string
                            secretValueInput.value = valueCipher.string
                            blindIndexInput.value = nameBlindIndex.b64
                            secretForm.submit()
                        }
                    }
                )
            }
        }
    )
}

async function deriveServiceAccountSecrets(serviceAccounts: ServiceAccount[], vaultECDHPrivateKey: CryptoKey,
    plaintextName: string, plaintextValue: string, blindIndex: string)  : Promise<CreateSecretsRequest> {

    const createSecretsRequest = new CreateSecretsRequest()

    for(var index = 0; index < serviceAccounts.length; index ++) {
        const serviceAccount = serviceAccounts[index]
        // Get a key agreement between the service account ECDH private key and the vault ECDH public key.
        const serviceAccountECDHPublicKeyData = 
            ByteData.fromB64(serviceAccount.getPublicEcdhKey())
        const serviceAccountECDHPublicKey: CryptoKey = 
            await Vault.importPublicECDHKey(serviceAccountECDHPublicKeyData)
        const aesKeyAgreement: CryptoKey = 
            await Vault.deriveSecretKey(vaultECDHPrivateKey, serviceAccountECDHPublicKey)
    
        // Associated Data
        const associatedData = new Uint8Array(4)
        const view = new DataView(associatedData.buffer)
        view.setUint32(0, serviceAccount.getServiceAccountId(), true /* littleEndian */);
    
        const enc = new TextEncoder(); // always utf-8
        const newEncryptedName = await Vault.aeadEncrypt(enc.encode(plaintextName), 
            associatedData, aesKeyAgreement)
        const newEncryptedValue = await Vault.aeadEncrypt(enc.encode(plaintextValue), 
            associatedData, aesKeyAgreement)
    
        const secret = new Secret()
        secret.setEncryptedSecretValue(newEncryptedValue.string)
        secret.setNameBlindIndex(blindIndex)
        secret.setEncryptedName(newEncryptedName.string)
    
        const serviceAccountSecrets = new ServiceAccountSecrets()
        serviceAccountSecrets.setServiceAccountId(serviceAccount.getServiceAccountId())
        serviceAccountSecrets.addSecrets(secret)

        createSecretsRequest.addAccountSecrets(serviceAccountSecrets)

        console.log(createSecretsRequest.getAccountSecretsList().length)
    }

    return createSecretsRequest
}