import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { ECDHKeyPair, AESKey } from '../../asset-pipeline/cryptography/vault'


class NewAccount extends SideDrawer {

    constructor() {
        super()

        let newAccountButton = document.getElementById('new-account')
        if(newAccountButton != null) {
            newAccountButton.addEventListener('click', async event => {
    
                this.open = true
    
                // We create a key pair wrapped with the users key.
                // Only the user that creates a service account can view the keypair
                const ecdhKeyPair = await ECDHKeyPair.fromRandom()
    
                const userAesKey = await AESKey.fromBarricade()
                const exportedECDHPrivateKey = await ecdhKeyPair.privateKey.export()
                const wrappedECDHPrivateKey = await userAesKey.encrypt(exportedECDHPrivateKey)
    
                const publicKeyField = this.querySelector('#public-key')
                const privateKeyField = this.querySelector('#private-key')
    
                if(publicKeyField instanceof HTMLInputElement &&
                    privateKeyField instanceof HTMLInputElement) {
                    publicKeyField.value = (await ecdhKeyPair.publicKey.export()).b64
                    privateKeyField.value = wrappedECDHPrivateKey.string
                }
            })
        }
    }
}

customElements.define('new-account', NewAccount);