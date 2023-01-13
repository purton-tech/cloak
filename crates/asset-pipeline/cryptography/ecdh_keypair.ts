import { DB } from "./db"
import { ByteData } from "./byte_data"
import { AESKey } from "./aes_key"
import { Cipher } from "./cipher"

export const ECDH_OPTIONS = {
    name: "ECDH",
    namedCurve: "P-256"
};

const AES_OPTIONS = {
    name: "AES-GCM",
    length: 256
};

const UNPROTECTED_ECDH_PRIVATE_KEY = 'unprotected_ecdh_private_key'
const ECDH_PUBLIC_KEY = 'ecdh_public_key'

export class ECDHKeyPair {

    public privateKey: ECDHPrivateKey
    public publicKey: ECDHPublicKey

    static async fromBarricade() : Promise<ECDHKeyPair> {

        const ecdhPrivateKey = await DB.getKeyFromIndexDB(UNPROTECTED_ECDH_PRIVATE_KEY)
        const ecdhPublicKey = await DB.getKeyFromIndexDB(ECDH_PUBLIC_KEY)
        return new this(new ECDHPublicKey(ecdhPublicKey), new ECDHPrivateKey(ecdhPrivateKey))
    }

    // Create a new random key pair
    static async fromRandom() : Promise<ECDHKeyPair> {
        const keyPair = await self.crypto.subtle.generateKey(ECDH_OPTIONS, true, ['deriveKey', 'deriveBits'])
        return new this(new ECDHPublicKey(keyPair.publicKey), new ECDHPrivateKey(keyPair.privateKey))
    }

    constructor(publicKey: ECDHPublicKey, privateKey: ECDHPrivateKey) {
        this.privateKey = privateKey
        this.publicKey = publicKey
    }
}

export class ECDHPublicKey {
    public publicKey: CryptoKey

    constructor(publicKey: CryptoKey) {
        this.publicKey = publicKey
    }

    async export() : Promise<ByteData> {
        return new ByteData(await self.crypto.subtle.exportKey('spki', this.publicKey))
    }

    // Encrypt a key that only the private key that corresponds to this public key
    // we be able to decrypt
    async wrapKey(key: AESKey) : Promise<{ wrappedKey: Cipher, publicKey: ECDHPublicKey }> {
        let ephemeralKeyPair = await ECDHKeyPair.fromRandom();
        const derivedAESKey = await ephemeralKeyPair.privateKey.deriveAESKey(this)
        const wrappedKey: Cipher = await derivedAESKey.wrap(key)

        return { wrappedKey: wrappedKey, publicKey: ephemeralKeyPair.publicKey }
    }
    
    // Encrypt a key that only the private key that corresponds to this public key
    // we be able to decrypt
    async wrapMessage(message: ByteData) : Promise<{ wrappedMessage: Cipher, publicKey: ECDHPublicKey }> {
        let ephemeralKeyPair = await ECDHKeyPair.fromRandom();
        const derivedAESKey = await ephemeralKeyPair.privateKey.deriveAESKey(this)
        const wrappedMessage: Cipher = await derivedAESKey.encrypt(message)

        return { wrappedMessage: wrappedMessage, publicKey: ephemeralKeyPair.publicKey }
    }

    static async import(spkiKey: ByteData) : Promise<ECDHPublicKey> {

        const key = await self.crypto.subtle.importKey('spki', spkiKey.arr.buffer,
            ECDH_OPTIONS, true, [])

        return new this(key)
    }
}

export class ECDHPrivateKey {
    public privateKey: CryptoKey

    constructor(privateKey: CryptoKey) {
        this.privateKey = privateKey
    }

    // Unwrap a key that was encrypted using an ECDH key agreement with a public key
    async unwrapKey(cipher: Cipher, publicKey: ECDHPublicKey) : Promise<AESKey> {
        const derivedAESKey = await this.deriveAESKey(publicKey)
        return await derivedAESKey.unwrap(cipher)
    }

    // Unwrap a key that was encrypted using an ECDH key agreement with a public key
    async unwrapMessage(cipher: Cipher, publicKey: ECDHPublicKey) : Promise<ByteData> {
        const derivedAESKey = await this.deriveAESKey(publicKey)
        return await derivedAESKey.decrypt(cipher)
    }

    async export() : Promise<ByteData> {
        return new ByteData(await self.crypto.subtle.exportKey('pkcs8', this.privateKey))
    }

    async deriveAESKey(publicKey: ECDHPublicKey) : Promise<AESKey> {
        const aesKey = await window.crypto.subtle.deriveKey(
            {
                name: "ECDH",
                public: publicKey.publicKey
            },
            this.privateKey,
            AES_OPTIONS,
            true,
            ["encrypt", "decrypt"]
        )
        return new AESKey(aesKey)
    }
}