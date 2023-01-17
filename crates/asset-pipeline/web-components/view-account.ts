import { SideDrawer } from './side-drawer'
import { AESKey, Cipher, ByteData } from '../cryptography/vault'

class ViewAccount extends SideDrawer {

    constructor() {
        super()

        const serviceAccountIDHiddenField = document.getElementById('service-account-id')

        if(serviceAccountIDHiddenField instanceof HTMLInputElement) {
            const serviceAccountId = parseInt(serviceAccountIDHiddenField.value)
        
            // Decrypt the ECDH private key
            try {
                const inputs = this.querySelectorAll(
                    '#wrapped-ecdh-private-key-' + serviceAccountId)
                
                for(var input of inputs) {
                    if (input instanceof HTMLAnchorElement) {
                        const key = input.getAttribute('data-key')
                        if(key != null) {
                            const cipher = Cipher.fromString(key)
                            AESKey.fromBarricade().then(usersAesKey => {
                                usersAesKey.decrypt(cipher).then(decryptedKey => {
                                    if (input instanceof HTMLAnchorElement) {
                                        input.href = 'data:text/plain;charset=utf-8,' 
                                            + encodeURIComponent(decryptedKey.toPem('PRIVATE'))
                                    }
                                })
                            })
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
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('view-account', ViewAccount);
    }
})