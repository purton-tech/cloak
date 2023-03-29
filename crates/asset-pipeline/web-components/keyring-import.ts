import { Cipher } from '../cryptography/cipher'
import { AESKey } from '../cryptography/vault'

// Show the user the cloka key in a way that they can import
// it into the cloak command line.
export class KeyringImport extends HTMLElement {

    constructor() {
        super()
        const wrappedKey = this.attributes.getNamedItem('wrapped-ecdh-private-key');
        const accountName = this.attributes.getNamedItem('account-name');
        try {
            if (wrappedKey != null && accountName != null) {
                const cipher = Cipher.fromString(wrappedKey.value)
                AESKey.fromBarricade().then(usersAesKey => {
                    usersAesKey.decrypt(cipher).then(decryptedKey => {

                        let keyImport = decryptedKey.toPem('PRIVATE')
                        keyImport = keyImport.replace('-----BEGIN PRIVATE KEY-----', '')
                        keyImport = keyImport.replace('-----END PRIVATE KEY-----', '')
                        keyImport = keyImport.replace(/\s/g, '')

                        console.log(keyImport)

                        let accountNameFormatted = accountName.value.replace(' ', '-')
                        accountNameFormatted = accountNameFormatted.toLocaleLowerCase()
                        
                        const template = document.createElement('template');
                        template.innerHTML = `
                        <textarea class='mb-2 width-full' rows='10'>cloak import ${accountNameFormatted} ${keyImport}</textarea>
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
        customElements.define('keyring-import', KeyringImport);
    }
})