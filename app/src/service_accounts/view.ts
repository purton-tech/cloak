import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import * as grpcWeb from 'grpc-web';
import { VaultClient } from '../../asset-pipeline/ApiServiceClientPb';
import { GetVaultRequest, GetVaultResponse } from '../../asset-pipeline/api_pb';
import { Vault, Cipher } from '../../asset-pipeline/vault'


async function handleConnect(rowId: number) {

    const vaultSelect = document.getElementById('vault-select-' + rowId)

    if (vaultSelect instanceof HTMLSelectElement && vaultSelect.selectedIndex != 0) {
        const vaultClient = new VaultClient(window.location.protocol
            + '//' + window.location.host, null, null);

        const request = new GetVaultRequest();
        request.setVaultId(parseInt(vaultSelect.options[vaultSelect.selectedIndex].value))

        // Call back to the server
        const call = vaultClient.getVault(request,

            // Important, Envoy will pick this up then authorise our request
            { 'authentication-type': 'cookie' },

            async (err: grpcWeb.RpcError, vault: GetVaultResponse) => {
                if (err) {
                    console.log('Error code: ' + err.code + ' "' + err.message + '"');
                } else {
                    const ecdhKey = document.getElementById('service-account-key-' + rowId)
                    if(ecdhKey instanceof HTMLInputElement) {
                        const cipher = Cipher.fromString(ecdhKey.value)
                        await decryptSecrets(vault, cipher)
                    }
                }
            }
        )
    }
}

async function decryptSecrets(vault: GetVaultResponse, encryptedECDHPrivateKey: Cipher) {

    // Decrypt the vault key.
    const vaultCipher = Cipher.fromString(vault.getEncryptedVaultKey())
    const vaultKey = await Vault.unwrapKey(vaultCipher)

    // Decrypt the ECDH key
    const ECDHPrivateKey = await Vault.unwrapECDHKey(encryptedECDHPrivateKey)
    console.log(ECDHPrivateKey)

    const dec = new TextDecoder(); // always utf-8

    // Process the secrets convert encrypt them with the 
    vault.getSecretsList().forEach(async secret => {
        const cipherName = Cipher.fromString(secret.getEncryptedName())
        console.log(cipherName)
        const plaintextName = await Vault.aesDecrypt(cipherName, vaultKey)
        console.log(dec.decode(plaintextName.arr))
    })
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