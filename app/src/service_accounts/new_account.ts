import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import { Vault, Cipher } from '../../asset-pipeline/vault'

let newAccountButton = document.getElementById('new-account')

if (newAccountButton) {
    newAccountButton.addEventListener('click', async event => {

        let element = newAccountButton.previousElementSibling.firstChild
        if (element instanceof SlDrawer) {
            element.show()
        }
    })
}