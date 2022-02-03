import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault } from '../../asset-pipeline/vault'

class NewVault extends SideDrawer {

    constructor() {
        super()

        let newVaultButton = document.getElementById('new-vault')
        newVaultButton.addEventListener('click', async event => {
            let element = newVaultButton.previousElementSibling.firstChild
            if (element instanceof SideDrawer) {
                element.open = true
    
                // Geneate a new ECDH key pair
                // Make a key agreement and get a wrapped AES key
                // Store wrapped key and public ECDH key.
                const aesKey = await Vault.createKey()
                const recipientPublicKey = await Vault.getECDHPublicKey()
                const { wrappedAesKey, publicKey } = await Vault.wrapKeyForRecipient(aesKey, recipientPublicKey)
        
                const wrappedKeyField = this.querySelector('[id="new-vault-key"]')
    
                const publicKeyField = this.querySelector('[id="public-key"]')
    
                if(publicKeyField instanceof HTMLInputElement &&
                    wrappedKeyField instanceof HTMLTextAreaElement) {
                    publicKeyField.value = publicKey.b64
                    wrappedKeyField.innerText = wrappedAesKey.string
                }
            }
        })
    }
}

customElements.define('new-vault', NewVault);