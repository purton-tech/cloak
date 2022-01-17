import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import * as grpcWeb from 'grpc-web';
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import { GetVaultRequest, GetVaultResponse, CreateSecretsRequest, ServiceAccountSecrets,CreateSecretsResponse } from '../../asset-pipeline/api_pb';
import { Vault, Cipher, ByteData } from '../../asset-pipeline/vault'

async function handleConnect(serviceAccountId: number) {

    const vaultSelect = document.getElementById('vault-select-' + serviceAccountId)
    const ecdhKey = document.getElementById('service-account-key-' + serviceAccountId)

    if (vaultSelect instanceof HTMLSelectElement && vaultSelect.selectedIndex != 0
        && ecdhKey instanceof HTMLInputElement) {

        const vaultClient = new VaultClient(window.location.protocol
            + '//' + window.location.host, null, null);

        const request = new GetVaultRequest();
        const vaultId = parseInt(vaultSelect.options[vaultSelect.selectedIndex].value)
        request.setVaultId(vaultId)

        // Call back to the server
        const call = vaultClient.getVault(request,

            // Important, Envoy will pick this up then authorise our request
            { 'authentication-type': 'cookie' },

            async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
                if (err) {
                    console.log('Error code: ' + err.code + ' "' + err.message + '"');
                } else {
                    const cipher = Cipher.fromString(ecdhKey.value)
                    await transferSecretsToServiceAccount(vault, 
                        cipher, serviceAccountId, vaultClient, vaultId)
                }
            }
        )
    }
}

async function transferSecretsToServiceAccount(vault: GetVaultResponse, 
    encryptedECDHPrivateKey: Cipher, serviceAccountId: number, 
    vaultClient: VaultClient, vaultId: number) {

    // Decrypt the vault key.
    const vaultCipher = Cipher.fromString(vault.getEncryptedVaultKey())
    const vaultKey = await Vault.unwrapKey(vaultCipher)

    // Decrypt the ECDH key
    const ECDHPrivateKey = await Vault.unwrapECDHKey(encryptedECDHPrivateKey)

    const dec = new TextDecoder(); // always utf-8

    // Get a key agreement between the service account ECDH private key and the vault ECDH public key.
    const vaultECDHPublicKeyData = ByteData.fromB64(vault.getVaultPublicEcdhKey())
    const vaultECDHPublicKey: CryptoKey = await Vault.importPublicECDHKey(vaultECDHPublicKeyData)
    const aesKeyAgreement: CryptoKey = await Vault.deriveSecretKey(ECDHPrivateKey, vaultECDHPublicKey)

    // Associated Data
    const associatedData = new Uint8Array(4)
    const view = new DataView(associatedData.buffer)
    view.setUint32(0, serviceAccountId, true /* littleEndian */);

    // Process the secrets - re-encrypt them with the agreement key.
    const secretList = vault.getSecretsList()
    for await (var secret of secretList) {
        const cipherName = Cipher.fromString(secret.getEncryptedName())
        const plaintextName: ByteData = await Vault.aesDecrypt(cipherName, vaultKey)
        console.log(dec.decode(plaintextName.arr))
        const newEncryptedName = await Vault.aeadEncrypt(plaintextName.arr, 
            associatedData, aesKeyAgreement)

        secret.setEncryptedName(newEncryptedName.string)
        const cipherValue = Cipher.fromString(secret.getEncryptedSecretValue())
        const plaintextValue: ByteData = await Vault.aesDecrypt(cipherValue, vaultKey)

        const newEncryptedValue = await Vault.aeadEncrypt(plaintextValue.arr, 
            associatedData, aesKeyAgreement)
        secret.setEncryptedSecretValue(newEncryptedValue.string)
    }

    // Send the encrypted payload back to the server
    const request = new CreateSecretsRequest()
    const serviceAccountSecrets = new ServiceAccountSecrets()
    serviceAccountSecrets.setServiceAccountId(serviceAccountId)
    serviceAccountSecrets.setSecretsList(secretList)
    request.addAccountSecrets(serviceAccountSecrets)

    const connectForm = document.getElementById('service-account-form-' + serviceAccountId)
    const connectFormVaultId = document.getElementById('service-account-form-vault-id-' + serviceAccountId)

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

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="service-account-row-"]').forEach(async (row) => {

    const serviceAccountId = parseInt(row.id.split('-')[3])

    // Detect when a user clicks a row
    row.addEventListener('click', () => {
        const drawer = document.getElementById('view-service-account-row-' + serviceAccountId)
        if (drawer instanceof SlDrawer) {
            drawer.show()
        }
    })

    // The user wants to connect a service account to a vault
    const connectButton = document.getElementById('connect-to-vault-' + serviceAccountId)
    if (connectButton instanceof HTMLButtonElement) {
        connectButton.addEventListener('click', async event => {
            event.preventDefault()

            await handleConnect(serviceAccountId)
        })
    }

    // Decrypt the ECDH private key
    try {
        const input = document.getElementById('wrapped-ecdh-private-key-' + serviceAccountId)
        if (input instanceof HTMLTextAreaElement) {
            const cipher = Cipher.fromString(input.innerText)
            const decryptedKey = await Vault.decrypt(cipher)
            input.value = decryptedKey.toPem('PRIVATE')
        }
        const pubKey = document.getElementById('ecdh-public-key-' + serviceAccountId)
        if (pubKey instanceof HTMLTextAreaElement) {
            const data = ByteData.fromB64(pubKey.innerText)
            pubKey.value = data.toPem('PUBLIC')
        }
    } catch (e) {
        console.log(e)
    }
})