import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import { Vault, Cipher, ByteData } from '../../asset-pipeline/vault'
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import * as grpcWeb from 'grpc-web';
import { GetVaultRequest, GetVaultResponse, CreateSecretsRequest, CreateSecretsResponse, Secret, ServiceAccount } from '../../asset-pipeline/api_pb';

let newSecretButton = document.getElementById('new-secret')

if (newSecretButton) {
    newSecretButton.addEventListener('click', async event => {

        let element = newSecretButton.previousElementSibling.firstChild
        if (element instanceof SlDrawer) {
            element.show()
        }
    })
}

let createSecretButton = document.getElementById('create-secret')

if (createSecretButton) {
    createSecretButton.addEventListener('click', async event => {
        event.preventDefault()

        const secretNameInput = document.getElementById('secret-name')
        const secretValueInput = document.getElementById('secret-value')
        const secretForm = document.getElementById('add-secret-form')
        const vaultKeyInput = document.getElementById('vault-key')
        const vaultIdInput = document.getElementById('vault-id')

        if (secretNameInput instanceof HTMLInputElement &&
            secretValueInput instanceof HTMLInputElement &&
            vaultKeyInput instanceof HTMLInputElement &&
            vaultIdInput instanceof HTMLInputElement &&
            secretForm instanceof HTMLFormElement) {
            const enc = new TextEncoder(); // always utf-8
            const vaultId = parseInt(vaultIdInput.value)

            console.log(secretForm.checkValidity())
            if (secretForm.checkValidity()) {
                try {
                    const vaultCipher = Cipher.fromString(vaultKeyInput.value)
                    const vaultKey = await Vault.unwrapKey(vaultCipher)

                    const cipher = await Vault.aesEncrypt(
                        enc.encode(secretNameInput.value), 
                        vaultKey)

                    const cipher2 = await Vault.aesEncrypt(
                        enc.encode(secretValueInput.value), 
                        vaultKey)

                    secretNameInput.value = cipher.string
                    secretValueInput.value = cipher2.string

                    await encryptSecretToConnectedServiceAccounts(
                        vaultKey, vaultId,
                        secretNameInput.value, secretValueInput.value)

                    secretForm.submit()
                } catch (err) {
                    if (err instanceof Error) {
                        console.log(err.message)
                    }
                }
            }
        }
    })
}

async function encryptSecretToConnectedServiceAccounts(vaultKey: CryptoKey, vaultId: number,
    secretName: string, secretValue: string) {

    const vaultClient = new VaultClient(window.location.protocol
        + '//' + window.location.host, null, null);

    const request = new GetVaultRequest();
    request.setVaultId(vaultId)

    // Call back to the server and get the vault details including the
    // connected service accounts
    const call = vaultClient.getVault(request,

        // Important, Envoy will pick this up then authorise our request
        { 'authentication-type': 'cookie' },

        async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
            if (err) {
                console.log('Error code: ' + err.code + ' "' + err.message + '"');
            } else {
                // For loops are async aware and will wait for all promises to complete
                for (let index = 0; index < vault.getServiceAccountsList().length; index++) {

                    const cipher = Cipher.fromString(vault.getEncryptedVaultPrivateEcdhKey())
                    const vaultECDHPrivateKey = await Vault.unwrapKey(cipher)
                    await addSecretForServiceAccount(
                        vault.getServiceAccountsList()[index], vaultECDHPrivateKey, 
                        vaultClient, secretName, secretValue)
                }
            }
        }
    )
}

async function addSecretForServiceAccount(serviceAccount: ServiceAccount, vaultECDHPrivateKey: CryptoKey,
    vaultClient: VaultClient, 
    plaintextName: string, plaintextValue: string) {

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
    secret.setEncryptedName(newEncryptedName.string)

    // Send the encrypted payload back to the server
    const request = new CreateSecretsRequest()
    request.setServiceAccountId(serviceAccount.getServiceAccountId())
    request.addSecrets(secret)

    const call = vaultClient.createSecrets(request,

        // Important, Envoy will pick this up then authorise our request
        { 'authentication-type': 'cookie' },

        async (err: grpcWeb.RpcError, serviceAccount: CreateSecretsResponse) => {
            if (err) {
                console.log('Error code: ' + err.code + ' "' + err.message + '"');
            } else {
            }
        }
    )
}