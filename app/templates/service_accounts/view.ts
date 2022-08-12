import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { AESKey, Cipher, ByteData } from '../../asset-pipeline/cryptography/vault'

class ViewAccount extends SideDrawer {

    constructor() {
        super()

        const serviceAccountAttr = this.getAttribute('service-account-id')

        if(serviceAccountAttr != null) {
            const serviceAccountId = parseInt(serviceAccountAttr)
        
            let viewAccountLink = document.getElementById('service-account-view-' + serviceAccountId)
            if(viewAccountLink != null) {
                viewAccountLink.addEventListener('click', async event => {
        
                    // Decrypt the ECDH private key
                    try {
                        const inputs = this.querySelectorAll(
                            '#wrapped-ecdh-private-key-' + serviceAccountId)
                        
                        for(var input of inputs) {
                            if (input instanceof HTMLTextAreaElement) {
                                const cipher = Cipher.fromString(input.value)
                                const usersAesKey = await AESKey.fromBarricade()
                                const decryptedKey = await usersAesKey.decrypt(cipher)
                                input.value = decryptedKey.toPem('PRIVATE')
                            } else if (input instanceof HTMLAnchorElement) {
                                const key = input.getAttribute('data-key')
                                if(key != null) {
                                    const cipher = Cipher.fromString(key)
                                    const usersAesKey = await AESKey.fromBarricade()
                                    const decryptedKey = await usersAesKey.decrypt(cipher)
                                    input.href = 'data:text/plain;charset=utf-8,' 
                                        + encodeURIComponent(decryptedKey.toPem('PRIVATE'))
                                }
                            }
                        }
                        const pubKeys = this.querySelectorAll(
                            '#ecdh-public-key-' + serviceAccountId)
                        for(var input of pubKeys) {
                            if (input instanceof HTMLTextAreaElement) {
                                const data = ByteData.fromB64(input.value)
                                input.value = data.toPem('PUBLIC')
                            }
                        }
                    } catch (e) {
                        console.log(e)
                    }
        
                    this.open = true
                })
            }
        }
    }
}

customElements.define('view-account', ViewAccount);