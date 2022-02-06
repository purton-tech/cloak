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

    // Encrypt the given payload
    async decrypt(cipher: Cipher) : Promise<ByteData> {
        const decOptions = {
            name: 'AES-GCM',
            iv: cipher.iv.arr.buffer
        };
        return new ByteData(await self.crypto.subtle.decrypt(decOptions, this.privateKey, cipher.ct.arr.buffer));
    }

    // Encrypt the given payload
    async aeadEncrypt(plaintext: ByteData, data: ByteData) : Promise<Cipher> {

        const encOptions = {
            name: 'AES-GCM',
            iv: new Uint8Array(12),
            additionalData: data
        };
        self.crypto.getRandomValues(encOptions.iv);
        const ivData = new ByteData(encOptions.iv.buffer);
        const cipher = new ByteData(
            await self.crypto.subtle.encrypt(encOptions, this.privateKey, plaintext.arr))

        return new Cipher(ivData, cipher)
    }

    // Encrypt the given payload
    async aeadDecrypt(cipher: Cipher, data: ByteData) : Promise<ByteData> {
        const decOptions = {
            name: 'AES-GCM',
            iv: cipher.iv.arr.buffer,
            additionalData: data
        };
        return new ByteData(await self.crypto.subtle.decrypt(
            decOptions, this.privateKey, cipher.ct.arr.buffer));
    }

    // Wrap an AES key
    async wrap(key: AESKey) : Promise<Cipher> {
        const symKeyData = new ByteData(await self.crypto.subtle.exportKey('raw', key.privateKey))
        return this.encrypt(symKeyData)
    }

    // Unwrap an AES key
    async unwrap(cipher: Cipher) : Promise<AESKey> {
        const symKeyData = await this.decrypt(cipher)

        const key =  await self.crypto.subtle.importKey(
            'raw', symKeyData.arr.buffer, AES_OPTIONS, true, ['decrypt', 'encrypt']);
        return new AESKey(key)
    }

    async export() : Promise<ByteData> {
        return new ByteData(await self.crypto.subtle.exportKey('raw', this.privateKey))
    }
}