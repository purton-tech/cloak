import { ECDHKeyPair, ECDHPublicKey } from '../cryptography/ecdh_keypair'
import { Cipher } from '../cryptography/cipher'
import { ByteData } from '../cryptography/byte_data'

// <ecdh-cipher cipher="", ecdh-public-key=""><ecdh-cipher>
// Create a key agreement betweent he users ECDH key and the public key
// Use the result to decrpyt the cipher
export class ECDHCipher extends HTMLElement {

    constructor() {
        super()
        const cipher = Cipher.fromString(this.attributes.getNamedItem('cipher').value)
        const ecdhPublicKey = ByteData.fromB64(
            this.attributes.getNamedItem('ecdh-public-key').value)
        const wrappedAesKey = Cipher.fromString(
            this.attributes.getNamedItem('wrapped-aes-key').value)

        // With the users ECDH key create a key form a key agreement
        // This key can then unwrap the wrapped key.
        // The unwrapped key can decrypt the cipher
        ECDHKeyPair.fromBarricade().then(keypair => {
            ECDHPublicKey.import(ecdhPublicKey).then(ecdhPublicKey => {
                keypair.privateKey.unwrapKey(wrappedAesKey, ecdhPublicKey).then(vaultKey => {
                    vaultKey.decrypt(cipher).then(plaintext => {
                        this.innerText = plaintext.toText()
                    })
                })
            })
        })
    }
}

customElements.define('ecdh-cipher', ECDHCipher);