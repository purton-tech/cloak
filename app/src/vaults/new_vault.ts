import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault } from '../../asset-pipeline/vault'

let newVaultButton = document.getElementById('new-vault')

if(newVaultButton) {
    newVaultButton.addEventListener('click', async event => {
        let element = newVaultButton.previousElementSibling.firstChild
        if (element instanceof SideDrawer) {
            element.open = true

            // Alice wants to create a vault that Bob can access.
            // Alice creates a new Symmetric vault key.
            // Alice Creates new Asymmetric Keys for the vault
            // Encrypt the Vault key, with an ECDH agreement between the Alice and the Vault as she currently has the Vault p
            // Encrypt the Asymmetric Private key with the users master key

            // For a user to recontrstruct a key
            // 
    
            // Get a completely new AES key and wrap it with the users master key
            let wrappedKey = await Vault.newWrappedKey()
            document.getElementById('new-vault-key').innerText = wrappedKey.string

            // Unwrap it again. Just in case.
            let vaultKey = await Vault.unwrapKey(wrappedKey)

            const keyPairDH = await Vault.generateWrappedECDHKeyPair(vaultKey);
            const publicKeyField = document.getElementById('public-key')
            const privateKeyField = document.getElementById('private-key')

            if(publicKeyField instanceof HTMLInputElement &&
                privateKeyField instanceof HTMLTextAreaElement) {
                publicKeyField.value = keyPairDH.publicKey.b64
                privateKeyField.innerText = keyPairDH.privateKey.string
            }
        }
    })
}