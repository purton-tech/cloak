import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault } from '../../asset-pipeline/vault'

let newVaultButton = document.getElementById('new-vault')

if(newVaultButton) {
    newVaultButton.addEventListener('click', async event => {
        let element = newVaultButton.previousElementSibling.firstChild
        console.log(element)
        if (element instanceof SideDrawer) {
            element.open = true
    
            let wrappedKey = await Vault.newWrappedKey()
            document.getElementById('new-vault-key').innerText = wrappedKey.string

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