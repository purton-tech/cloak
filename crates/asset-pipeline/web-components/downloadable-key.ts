import { Cipher } from '../cryptography/cipher'
import { AESKey } from '../cryptography/vault'

// <downloadable-key wrapped-ecdh-key=""><downloadable-key>
// Unwraps the key into a downloable link
export class DownloadableKey extends HTMLElement {

    constructor() {
        super()
        const wrappedKey = this.attributes.getNamedItem('wrapped-ecdh-private-key');
        try {
            if (wrappedKey != null) {
                const cipher = Cipher.fromString(wrappedKey.value)
                AESKey.fromBarricade().then(usersAesKey => {
                    usersAesKey.decrypt(cipher).then(decryptedKey => {
                        console.log(decryptedKey)
                        const href = 'data:text/plain;charset=utf-8,' 
                            + encodeURIComponent(decryptedKey.toPem('PRIVATE'))
                        
                        const template = document.createElement('template');
                        template.innerHTML = `
                        <a href="${href}" download="cloak.pem">cloak.pem</a>
                        `
                        const templateNode = template.cloneNode(true)
                        if (templateNode instanceof HTMLTemplateElement) {
                            const templateDocument = templateNode.content
                            this.appendChild(templateDocument)
                        }
                    })
                })
            }

        } catch(e) {
            console.error(e)
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('downloadable-key', DownloadableKey);
    }
})