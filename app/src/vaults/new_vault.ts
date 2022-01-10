import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import { Vault } from '../../asset-pipeline/vault'

let newVaultButton = document.getElementById('new-vault')

if(newVaultButton) {
    newVaultButton.addEventListener('click', async event => {
        let element = newVaultButton.previousElementSibling.firstChild
        if (element instanceof SlDrawer) {
            element.show()
    
            document.getElementById('new-vault-key').innerText = (await Vault.newWrappedKey()).string

            const keyPairDH = await Vault.generateWrappedECDHKeyPair();
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