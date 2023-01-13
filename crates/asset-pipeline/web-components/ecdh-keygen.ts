import { AESKey, ECDHKeyPair } from '../cryptography/vault'

// Create a key pair encrypted with the users key, the keys are stored in hidden
// fields which can be used in a form and sent to the server
//
// <ecdh-keygen  public="public_key", private="encrypted_vault_key"><ecdh-cipher>
// Create a key agreement betweent he users ECDH key and the public key
// Use the result to decrpyt the cipher
export class EcdhKeygen extends HTMLElement {

    constructor() {
        super()
        const privateName = this.attributes.getNamedItem('private')?.value;
        const publicName = this.attributes.getNamedItem('public')?.value;

        ECDHKeyPair.fromBarricade().then(ecdhKeyPair => {
            AESKey.fromRandom().then(aesKey => {
                ecdhKeyPair.publicKey.wrapKey(aesKey).then(result => {
                    result.publicKey.export().then(throwawayKeyPairExport => {
                        const template = document.createElement('template');
                        template.innerHTML = `
                        <input type="hidden" name="${privateName}" value="${result.wrappedKey.string}">
                        <input type="hidden" name="${publicName}" value="${throwawayKeyPairExport.b64}">
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
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('ecdh-keygen', EcdhKeygen)
    }
})