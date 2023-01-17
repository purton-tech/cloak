import { AESKey, ECDHKeyPair } from '../cryptography/vault'

/**
 * Create a key pair encrypted with the users key, the keys are stored in hidden
 * fields which can be used in a form and sent to the server
 * 
 * <ecdh-keygen  public="public_key", private="encrypted_vault_key"><ecdh-cipher>
 * 
 * Public and private are the names that will be used for the fields.
 * 
 * 1. Generate a random AES key.
 * 2. Wrap it using the user public ECDH key.
 */
export class EcdhNonShareableKeygen extends HTMLElement {

    constructor() {
        super()
        const privateName = this.attributes.getNamedItem('private')?.value;
        const publicName = this.attributes.getNamedItem('public')?.value;

        ECDHKeyPair.fromRandom().then(ecdhKeyPair => {
            AESKey.fromBarricade().then(userAesKey => {
                ecdhKeyPair.privateKey.export().then(exportedECDHPrivateKey => {
                    userAesKey.encrypt(exportedECDHPrivateKey).then(wrappedECDHPrivateKey => {
                        ecdhKeyPair.publicKey.export().then(publicKey => {

                            const template = document.createElement('template');
                            template.innerHTML = `
                            <input type="hidden" name="${privateName}" value="${wrappedECDHPrivateKey.string}">
                            <input type="hidden" name="${publicName}" value="${publicKey.b64}">
                            `
                            const templateNode = template.cloneNode(true)
                            if (templateNode instanceof HTMLTemplateElement) {
                                const templateDocument = templateNode.content
                                this.appendChild(templateDocument)
                            }
                        })
                    })
                })
            })
        })
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('ecdh-non-shareable-keygen', EcdhNonShareableKeygen)
    }
})