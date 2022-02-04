import { DB } from "./db"
import { ByteData } from "./byte_data"
import { AESKey } from "./aes_key"

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

        const echdPrivateKey = await DB.getKeyFromIndexDB(UNPROTECTED_ECDH_PRIVATE_KEY)
        const echdPublicKey = await DB.getKeyFromIndexDB(ECDH_PUBLIC_KEY)
        return new this(new ECDHPublicKey(echdPublicKey), new ECDHPrivateKey(echdPrivateKey))
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

    async deriveSecretFromPublicKey(publicKey: ECDHPublicKey) : Promise<AESKey> {
        const aesKey = await window.crypto.subtle.deriveKey(
            {
                name: "ECDH",
                public: publicKey.publicKey
            },
            this.privateKey.privateKey,
            AES_OPTIONS,
            true,
            ["encrypt", "decrypt"]
        )
        return new AESKey(aesKey)
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
}

export class ECDHPrivateKey {
    public privateKey: CryptoKey

    constructor(privateKey: CryptoKey) {
        this.privateKey = privateKey
    }
}