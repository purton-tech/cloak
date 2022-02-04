import { DB } from "./db"
import { ByteData } from "./byte_data"
import { Cipher } from "./cipher"

const AES_OPTIONS = {
    name: "AES-GCM",
    length: 256
};

const UNPROTECTED_SYMMETRIC_KEY = 'unprotected_symmetric_key'

export class AESKey {
    public privateKey: CryptoKey

    constructor(privateKey: CryptoKey) {
        this.privateKey = privateKey
    }

    static async fromRandom() : Promise<AESKey> {
        const newAesKey = await self.crypto.subtle.generateKey(
            AES_OPTIONS,
            true,
            ['decrypt', 'encrypt'])
        return new this(newAesKey)
    }

    // Encrypt the given payload
    async encrypt(data: ByteData) : Promise<Cipher> {
        const encOptions = {
            name: 'AES-GCM',
            iv: new Uint8Array(16)
        };
        self.crypto.getRandomValues(encOptions.iv);
        const ivData = new ByteData(encOptions.iv.buffer);
        const cipher = new ByteData(
            await self.crypto.subtle.encrypt(encOptions, this.privateKey, 
                data.arr))

        return new Cipher(ivData, cipher)
    }

    // Encrypt this key with another key.
    async wrap(key: AESKey) : Promise<Cipher> {
        const symKeyData = new ByteData(await self.crypto.subtle.exportKey('raw', this.privateKey))
        return key.encrypt(symKeyData)
    }
}