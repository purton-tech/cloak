import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { ECDHKeyPair, AESKey } from '../../asset-pipeline/cryptography/vault'


class NewVault extends SideDrawer {

    constructor() {
        super()

        let newVaultButton = document.getElementById('new-vault')
        newVaultButton.addEventListener('click', async event => {
            let element = document.getElementById('new-vault-drawer')
            if (element instanceof SideDrawer) {
                element.open = true
    
                let aliceECDHKeyPair = await ECDHKeyPair.fromBarricade();
                const aesVaultKey = await AESKey.fromRandom()

                const { wrappedKey, publicKey } = await aliceECDHKeyPair.publicKey.wrapKey(aesVaultKey)

                // As a check try to unwrap it. This will blow up if the logic doesn't work
                await aliceECDHKeyPair.privateKey.unwrapKey(wrappedKey, publicKey)

                const wrappedKeyField = this.querySelector('[id="new-vault-key"]')
    
                const publicKeyField = this.querySelector('[id="public-key"]')

    
                if(publicKeyField instanceof HTMLInputElement &&
                    wrappedKeyField instanceof HTMLInputElement) {
                    
                    const throwawayKeyPairExport = await publicKey.export()
                    publicKeyField.value = throwawayKeyPairExport.b64
                    wrappedKeyField.value = wrappedKey.string
                }
            }
        })
    }
}

customElements.define('new-vault', NewVault);