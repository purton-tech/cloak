import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { AESKey, Cipher, ByteData } from '../../asset-pipeline/cryptography/vault'

class ViewAccount extends SideDrawer {

    constructor() {
        super()

        const serviceAccountId = parseInt(this.getAttribute('service-account-id'))
    
        let viewAccountLink = document.getElementById('service-account-view-' + serviceAccountId)
        viewAccountLink.addEventListener('click', async event => {

            // Decrypt the ECDH private key
            try {
                const input = this.querySelector('#wrapped-ecdh-private-key-' + serviceAccountId)
                if (input instanceof HTMLTextAreaElement) {
                    const cipher = Cipher.fromString(input.value)
                    const usersAesKey = await AESKey.fromBarricade()
                    const decryptedKey = await usersAesKey.decrypt(cipher)
                    input.value = decryptedKey.toPem('PRIVATE')
                }
                const pubKey = this.querySelector('#ecdh-public-key-' + serviceAccountId)
                if (pubKey instanceof HTMLTextAreaElement) {
                    const data = ByteData.fromB64(pubKey.value)
                    pubKey.value = data.toPem('PUBLIC')
                }
            } catch (e) {
                console.log(e)
            }

            this.open = true
        })
    }
}

customElements.define('view-account', ViewAccount);