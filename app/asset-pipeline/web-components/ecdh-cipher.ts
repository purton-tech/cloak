import { ECDHKeyPair, ECDHPublicKey } from '../cryptography/ecdh_keypair'
import { Cipher } from '../cryptography/cipher'
import { ByteData } from '../cryptography/byte_data'
import { Vault } from '../cryptography/vault'

// <ecdh-cipher cipher="", ecdh-public-key="", wrapped-aes-key=""><ecdh-cipher>
// Create a key agreement betweent he users ECDH key and the public key
// Use the result to decrpyt the cipher
export class ECDHCipher extends HTMLElement {

    constructor() {
        super()
        const cipherEle = this.attributes.getNamedItem('cipher');
        const pkEle = this.attributes.getNamedItem('ecdh-public-key');
        const wrappedEle = this.attributes.getNamedItem('wrapped-aes-key');
        if (cipherEle != null && pkEle != null && wrappedEle != null) {
            const cipher = Cipher.fromString(cipherEle.value)
            const ecdhPublicKey = ByteData.fromB64(pkEle.value)
            const wrappedAesKey = Cipher.fromString(wrappedEle.value)
    
            // With the users ECDH key create a key form a key agreement
            // This key can then unwrap the wrapped key.
            // The unwrapped key can decrypt the cipher
            ECDHPublicKey.import(ecdhPublicKey).then(ecdhPublicKey => {
                Vault.decryptVaultKey(wrappedAesKey, ecdhPublicKey).then(vaultKey => {
                    vaultKey.decrypt(cipher).then(plaintext => {
                        this.innerText = plaintext.toText()
                    })
                })
            })
        } else {
            console.error('Coulkd not find the HTML elements needed')
        }
    }
}

customElements.define('ecdh-cipher', ECDHCipher);