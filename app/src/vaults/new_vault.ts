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
    
                // Geneate a new ECDH key pair
                //const ecdhKeyPair = await generateECDHKeyPair()
                let throwawayKeyPair = await ECDHKeyPair.fromRandom();
                let aliceECDHKeyPair = await ECDHKeyPair.fromBarricade();

                const derivedAesKey = await 
                    throwawayKeyPair.deriveSecretFromPublicKey(aliceECDHKeyPair.publicKey)

                // Create a new Symmteric key and wrap it with the derived key
                const aesVaultKey = await AESKey.fromRandom()
                const aesVaultKeyWrapped: Cipher = await aesVaultKey.wrap(derivedAesKey)

                const wrappedKeyField = this.querySelector('[id="new-vault-key"]')
    
                const publicKeyField = this.querySelector('[id="public-key"]')

    
                if(publicKeyField instanceof HTMLInputElement &&
                    wrappedKeyField instanceof HTMLTextAreaElement) {
                    
                    const throwawayKeyPairExport = 
                        await throwawayKeyPair.publicKey.export()
                    publicKeyField.value = throwawayKeyPairExport.b64
                    wrappedKeyField.innerText = aesVaultKeyWrapped.string
                }
            }
        })
    }
}

customElements.define('new-vault', NewVault);