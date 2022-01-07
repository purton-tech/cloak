import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import * as grpcWeb from 'grpc-web';
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import { GetVaultRequest, GetVaultResponse, CreateServiceAccountRequest, CreateServiceAccountResponse } from '../../asset-pipeline/api_pb';
import { Vault, Cipher, ByteData } from '../../asset-pipeline/vault'

async function handleConnect(rowId: number) {

    const vaultSelect = document.getElementById('vault-select-' + rowId)

    if (vaultSelect instanceof HTMLSelectElement && vaultSelect.selectedIndex != 0) {
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
                    const ecdhKey = document.getElementById('service-account-key-' + rowId)
                    if (ecdhKey instanceof HTMLInputElement) {
                        const cipher = Cipher.fromString(ecdhKey.value)
                        await transferSecretsToServiceAccount(vault, cipher, vaultId, vaultClient)
                    }
                }
            }
        )
    }
}

async function transferSecretsToServiceAccount(vault: GetVaultResponse, 
    encryptedECDHPrivateKey: Cipher, vaultId: number, vaultClient: VaultClient) {

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

    console.log(aesKeyAgreement)

    // Process the secrets - re-encrypt them with the agreement key.
    const secretList = vault.getSecretsList()
    for await (var secret of secretList) {
        const cipherName = Cipher.fromString(secret.getEncryptedName())
        const plaintextName: ByteData = await Vault.aesDecrypt(cipherName, vaultKey)
        console.log(dec.decode(plaintextName.arr))
        const newEncryptedName = await Vault.aesEncrypt(plaintextName.arr, aesKeyAgreement)
        secret.setEncryptedName(newEncryptedName.string)
        const cipherValue = Cipher.fromString(secret.getEncryptedSecretValue())
        const plaintextValue: ByteData = await Vault.aesDecrypt(cipherValue, vaultKey)
        const newEncryptedValue = await Vault.aesEncrypt(plaintextValue.arr, aesKeyAgreement)
        secret.setEncryptedName(newEncryptedValue.string)
    }

    // Send the enc rypted payload back to the server
    const request = new CreateServiceAccountRequest()
    request.setVaultId(vaultId)
    request.setSecretsList(secretList)
    
    const call = vaultClient.createServiceAccount(request,

        // Important, Envoy will pick this up then authorise our request
        { 'authentication-type': 'cookie' },

        async (err: grpcWeb.RpcError, serviceAccount: CreateServiceAccountResponse) => {
            if (err) {
                console.log('Error code: ' + err.code + ' "' + err.message + '"');
            } else {
                console.log('sent')
            }
        }
    )
}

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="service-account-row-"]').forEach((row) => {

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

            console.log('clicked')

            await handleConnect(serviceAccountId)
        })
    }
})