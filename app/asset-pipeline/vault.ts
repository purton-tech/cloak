import { openDB } from 'idb';

const AES_OPTIONS = {
    name: "AES-GCM",
    length: 256
};

const ECDH_OPTIONS = {
    name: "ECDH",
    namedCurve: "P-256"
};

const UNPROTECTED_SYMMETRIC_KEY = 'unprotected_symmetric_key'
const UNPROTECTED_ECDSA_PRIVATE_KEY = 'unprotected_ecdsa_private_key'
const ECDH_PUBLIC_KEY = 'ecdh_public_key'
const DB_NAME = 'keyval'

// All client side cryptography comes through this class.
export class Vault {

    public static async createKey(): Promise<CryptoKey> {

        const newAesKey = await self.crypto.subtle.generateKey(
            AES_OPTIONS,
            true,
            ['decrypt', 'encrypt'])
        return newAesKey
    }

    public static async getECDHPublicKey(): Promise<CryptoKey> {
        const db = await this.openIndexedDB()
        const key = await db.get(DB_NAME, ECDH_PUBLIC_KEY) as CryptoKey
        db.close()

        return key
    }

    public static async wrapKeyForRecipient(key: CryptoKey, publicKey: CryptoKey): 
        Promise<{ wrappedAesKey: Cipher, publicKey: ByteData }> {
        const keyPair = await self.crypto.subtle.generateKey(ECDH_OPTIONS, true, ['deriveKey', 'deriveBits'])

        const aesKey = await this.deriveSecretKey(keyPair.privateKey, publicKey)

        const symKeyData = new ByteData(await self.crypto.subtle.exportKey('raw', aesKey))
        const protectedSymKey = await this.encrypt(symKeyData.arr);
        const publicKeyData = new ByteData(await self.crypto.subtle.exportKey('spki', keyPair.publicKey));
        return { wrappedAesKey: protectedSymKey, publicKey: publicKeyData }
    }

    // Use the ECDSA private key to sign data.
    // We can verify signature with openssl as we generate in DER format.
    // openssl dgst -SHA384 -verify key.pem -signature signature.bin proto.bin
    public static async sign(bytesToSign: ByteData): Promise<ByteData> {

        const db = await this.openIndexedDB()
        const ecdsaKey = await db.get(DB_NAME, UNPROTECTED_ECDSA_PRIVATE_KEY) as CryptoKey
        db.close()

        let signature = await window.crypto.subtle.sign(
            {
                name: "ECDSA",
                hash: { name: "SHA-256" },
            },
            ecdsaKey,
            bytesToSign.arr
        );

        return this.toDER(new ByteData(signature))
    }


    public static async blindIndex(text: string, id: number): Promise<ByteData> {
        let enc = new TextEncoder();
        const data = enc.encode(text + ':' + id)
        const hash: ArrayBuffer = await crypto.subtle.digest('SHA-256', data)
        return new ByteData(hash.slice(0, 8))
    }

    public static async newWrappedKey(): Promise<Cipher> {

        const newAesKey = await self.crypto.subtle.generateKey(
            AES_OPTIONS,
            true,
            ['decrypt', 'encrypt'])
        const symKeyData = new ByteData(await self.crypto.subtle.exportKey('raw', newAesKey))
        const protectedSymKey = await this.encrypt(symKeyData.arr);

        return protectedSymKey
    }

    public static async unwrapKey(cipher: Cipher): Promise<CryptoKey> {

        const byteData = await this.decrypt(cipher)

        return await self.crypto.subtle.importKey(
            'raw', byteData.arr.buffer, AES_OPTIONS, false, ['decrypt', 'encrypt']);

    }

    public static async unwrapECDHKey(cipher: Cipher): Promise<CryptoKey> {

        const byteData = await this.decrypt(cipher)

        return await self.crypto.subtle.importKey(
            'pkcs8', byteData.arr.buffer, ECDH_OPTIONS, false, ['deriveKey', 'deriveBits']);

    }

    public static async unwrapECDHKeyPair(cipher: Cipher, key: CryptoKey): Promise<CryptoKey> {

        const byteData = await this.aesDecrypt(cipher, key)

        return await self.crypto.subtle.importKey(
            'pkcs8', byteData.arr.buffer, ECDH_OPTIONS, false, ['deriveKey', 'deriveBits']);

    }

    public static async deriveSecretKey(privateKey: CryptoKey, publicKey: CryptoKey): Promise<CryptoKey> {
        return window.crypto.subtle.deriveKey(
            {
                name: "ECDH",
                public: publicKey
            },
            privateKey,
            AES_OPTIONS,
            true,
            ["encrypt", "decrypt"]
        );
    }

    public static async importPublicECDHKey(key: ByteData): Promise<CryptoKey> {
        return await self.crypto.subtle.importKey('spki', key.arr.buffer,
            ECDH_OPTIONS, false, [])
    }

    public static async generateUserWrappedECDHKeyPair() {

        const db = await this.openIndexedDB()
        const key = await db.get(DB_NAME, UNPROTECTED_SYMMETRIC_KEY) as CryptoKey
        db.close()

        return await this.generateWrappedECDHKeyPair(key)
    }

    public static async generateWrappedECDHKeyPair(key: CryptoKey) {

        try {
            const keyPair = await self.crypto.subtle.generateKey(ECDH_OPTIONS, true, ['deriveKey', 'deriveBits']);
            const publicKey = new ByteData(await self.crypto.subtle.exportKey('spki', keyPair.publicKey));
            const privateKey = new ByteData(await self.crypto.subtle.exportKey('pkcs8', keyPair.privateKey));
            const protectedPrivateKey = await this.aesEncrypt(privateKey.arr, key);
            return {
                publicKey: publicKey,
                privateKey: protectedPrivateKey
            };
        } catch (err) {
            console.error(err);
        }
    }

    public static async encrypt(data: Uint8Array): Promise<Cipher> {

        const db = await this.openIndexedDB()
        const key = await db.get(DB_NAME, UNPROTECTED_SYMMETRIC_KEY) as CryptoKey
        db.close()

        return await this.aesEncrypt(data, key)
    }

    public static async decrypt(cipher: Cipher): Promise<ByteData> {

        const db = await this.openIndexedDB()
        const key = await db.get(DB_NAME, UNPROTECTED_SYMMETRIC_KEY) as CryptoKey
        db.close()

        return await this.aesDecrypt(cipher, key)
    }

    public static async aesEncrypt(data: Uint8Array, key: CryptoKey): Promise<Cipher> {

        const encOptions = {
            name: 'AES-GCM',
            iv: new Uint8Array(16)
        };
        self.crypto.getRandomValues(encOptions.iv);
        const ivData = new ByteData(encOptions.iv.buffer);
        const cipher = new ByteData(await self.crypto.subtle.encrypt(encOptions, key, data))

        return new Cipher(ivData, cipher)
    }

    public static async aeadEncrypt(plaintext: Uint8Array,
        data: Uint8Array, key: CryptoKey): Promise<Cipher> {

        const encOptions = {
            name: 'AES-GCM',
            iv: new Uint8Array(12),
            additionalData: data
        };
        self.crypto.getRandomValues(encOptions.iv);
        const ivData = new ByteData(encOptions.iv.buffer);
        const cipher = new ByteData(await self.crypto.subtle.encrypt(encOptions, key, plaintext))

        return new Cipher(ivData, cipher)
    }

    public static async aesDecrypt(cipher: Cipher, key: CryptoKey): Promise<ByteData> {

        const decOptions = {
            name: 'AES-GCM',
            iv: cipher.iv.arr.buffer
        };

        return new ByteData(await self.crypto.subtle.decrypt(decOptions, key, cipher.ct.arr.buffer));
    }

    private static async openIndexedDB() {
        return await openDB('Vault', 1, {
            upgrade(db) {
                db.createObjectStore("keyval");
            },
        });
    }
    // Copied from https://stackoverflow.com/questions/39554165/ecdsa-signatures-between-node-js-and-webcrypto-appear-to-be-incompatible
    // It generated hex, we could write a more efficient one that jsut works with bytes.
    private static toDER(signature: ByteData): ByteData {

        // Extract r & s and format it in ASN1 format.
        var signHex = Array.prototype.map.call(signature.arr, function (x) {
            return ('00' + x.toString(16)).slice(-2);
        }).join(''),
            r = signHex.substring(0, signHex.length / 2),
            s = signHex.substring(signHex.length / 2),
            rPre = true,
            sPre = true;

        while (r.indexOf('00') === 0) {
            r = r.substring(2);
            rPre = false;
        }

        if (rPre && parseInt(r.substring(0, 2), 16) > 127) {
            r = '00' + r;
        }

        while (s.indexOf('00') === 0) {
            s = s.substring(2);
            sPre = false;
        }

        if (sPre && parseInt(s.substring(0, 2), 16) > 127) {
            s = '00' + s;
        }

        const payload = '02' + this.lengthOfHex(r) + r + '02' + this.lengthOfHex(s) + s
        const der = '30' + this.lengthOfHex(payload) + payload

        return ByteData.fromHex(der)
    }

    // Auxs
    private static lengthOfHex(hex) {
        return ('00' + (hex.length / 2).toString(16)).slice(-2).toString();
    }

}


export class ByteData {

    arr: Uint8Array
    b64: string
    hex: string

    constructor(buf) {
        if (!arguments.length) {
            this.arr = null;
            this.b64 = null;
            return;
        }

        this.arr = new Uint8Array(buf);
        this.b64 = this.toB64(buf);
        this.hex = this.toHex(buf);
    }

    toB64(buf) {
        let binary = '';
        const bytes = new Uint8Array(buf);
        for (let i = 0; i < bytes.byteLength; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return btoa(binary);
    }

    addNewLines(str: string) {
        var finalString = '';
        while (str.length > 0) {
            finalString += str.substring(0, 64) + '\n';
            str = str.substring(64);
        }

        return finalString;
    }

    toPem(type: string) {
        var b64WithLines = this.addNewLines(this.b64);
        var pem = "-----BEGIN " + type + " KEY-----\n" + b64WithLines + "-----END " + type + " KEY-----";

        return pem;
    }

    toHex(bytes: Uint8Array) {
        for (var hex = [], i = 0; i < bytes.length; i++) {
            var current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
            hex.push((current >>> 4).toString(16));
            hex.push((current & 0xF).toString(16));
        }
        return hex.join("");
    }

    static fromB64(base64: string): ByteData {
        var binary_string = atob(base64);
        var len = binary_string.length;
        var bytes = new Uint8Array(len);
        for (var i = 0; i < len; i++) {
            bytes[i] = binary_string.charCodeAt(i);
        }
        return new this(bytes)
    }

    static fromHex(hex: string) {
        for (var bytes = [], c = 0; c < hex.length; c += 2)
            bytes.push(parseInt(hex.substr(c, 2), 16));
        return new this(bytes)
    }

    static fromText(text: string) {
        const enc = new TextEncoder()
        return new this(enc.encode(text))
    }
}

export class Cipher {

    iv: ByteData
    ct: ByteData
    string: string

    constructor(iv: ByteData, ct: ByteData) {
        if (!arguments.length) {
            this.iv = null;
            this.ct = null;
            this.string = null;
            return;
        }

        this.iv = iv;
        this.ct = ct;
        this.string = iv.b64 + '|' + ct.b64;
    }

    static fromString(string: string): Cipher {
        const iv = ByteData.fromB64(string.split('|')[0])
        const ct = ByteData.fromB64(string.split('|')[1])
        return new this(iv, ct)
    }
}