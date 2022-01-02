import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import { Vault } from '../../asset-pipeline/vault'

let newVaultButton = document.getElementById('new-vault')

if(newVaultButton) {
    newVaultButton.addEventListener('click', async event => {
        let element = newVaultButton.previousElementSibling.firstChild
        if (element instanceof SlDrawer) {
            element.show()
    
            document.getElementById('new-vault-key').innerText = (await Vault.newWrappedKey()).string
        }
    })
}