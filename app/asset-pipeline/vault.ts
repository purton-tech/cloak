import { openDB } from 'idb';

const AES_OPTIONS = {
    name: "AES-GCM",
    length: 256
};

const ECDH_OPTIONS = {
    name: "ECDH",
    namedCurve: "P-384"
};

export class Vault {

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
    
    public static async generateWrappedECDHKeyPair() {
    
        try {
            const keyPair = await self.crypto.subtle.generateKey(ECDH_OPTIONS, true, ['deriveKey', 'deriveBits']);
            const publicKey = new ByteData(await self.crypto.subtle.exportKey('spki', keyPair.publicKey));
            const privateKey = new ByteData(await self.crypto.subtle.exportKey('pkcs8', keyPair.privateKey));
            const protectedPrivateKey = await this.encrypt(privateKey.arr);
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
        const key = await db.get('keyval', 'unprotected_symmetric_key') as CryptoKey
        db.close()
    
        return await this.aesEncrypt(data, key)
    }

    public static async decrypt(cipher: Cipher): Promise<ByteData> {
    
        const db = await this.openIndexedDB()
        const key = await db.get('keyval', 'unprotected_symmetric_key') as CryptoKey
        db.close()
    
        return await this.aesDecrypt(cipher, key)
    }
    
    public static async aesEncrypt(data: Uint8Array, key: CryptoKey) {
    
        const encOptions = {
            name: 'AES-GCM',
            iv: new Uint8Array(16)
        };
        self.crypto.getRandomValues(encOptions.iv);
        const ivData = new ByteData(encOptions.iv.buffer);
        const cipher = new ByteData(await self.crypto.subtle.encrypt(encOptions, key, data))
    
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
        var signHex = Array.prototype.map.call(signature.arr, function (x) { return ('00' + x.toString(16)).slice(-2); }).join(''),
            r = signHex.substring(0, 96),
            s = signHex.substring(96),
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

    static fromString(string: string) : Cipher {
        const iv = ByteData.fromB64(string.split('|')[0])
        const ct = ByteData.fromB64(string.split('|')[1])
        return new this(iv, ct)
    }
}