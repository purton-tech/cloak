import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault, Cipher } from '../../asset-pipeline/cryptography/vault'

let newAccountButton = document.getElementById('new-account')

if (newAccountButton) {
    newAccountButton.addEventListener('click', async event => {

        let element = newAccountButton.previousElementSibling.firstChild
        if (element instanceof SideDrawer) {
            element.open = true

            // We create a key pair wrapped with the users key.
            // Only the user that creates a service account can view the keypair
            const keyPairDH = await Vault.generateUserWrappedECDHKeyPair();
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