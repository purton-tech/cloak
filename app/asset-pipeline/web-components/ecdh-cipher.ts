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

        ECDHKeyPair.fromBarricade().then(keypair => {
            ECDHPublicKey.import(ecdhPublicKey).then(ecdhPublicKey => {
                keypair.privateKey.unwrapMessage(cipher, ecdhPublicKey).then(plaintext => {
                    this.innerText = plaintext.toText()
                    console.log('here')
                })
            })
        })
    }
}

customElements.define('ecdh-cipher', ECDHCipher);