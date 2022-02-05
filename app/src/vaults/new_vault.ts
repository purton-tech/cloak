import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { ByteData, Vault , ECDH_OPTIONS, Cipher, ECDHKeyPair, AESKey } from '../../asset-pipeline/cryptography/vault'


class NewVault extends SideDrawer {

    constructor() {
        super()

        let newVaultButton = document.getElementById('new-vault')
        document.getElementById('new-vault').addEventListener('click', async event => {
            let element = newVaultButton.previousElementSibling.firstChild
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
                    wrappedKeyField instanceof HTMLTextAreaElement) {
                    
                    const throwawayKeyPairExport = await publicKey.export()
                    publicKeyField.value = throwawayKeyPairExport.b64
                    wrappedKeyField.innerText = wrappedKey.string
                }
            }
        })
    }
}

customElements.define('new-vault', NewVault);