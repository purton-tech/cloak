import { DB } from "./db"
import { ByteData } from "./byte_data"

export const ECDSA_OPTIONS = {
    name: "ECDSA",
    namedCurve: "P-256"
};

const UNPROTECTED_ECDSA_PRIVATE_KEY = 'unprotected_ecdsa_private_key'
const ECDSA_PUBLIC_KEY = 'ecdsa_public_key'

export class ECDSAKeyPair {

    public privateKey: ECDSAPrivateKey
    public publicKey: ECDSAPublicKey

    static async fromBarricade() : Promise<ECDSAKeyPair> {

        const ecdsaPrivateKey = await DB.getKeyFromIndexDB(UNPROTECTED_ECDSA_PRIVATE_KEY)
        const ecdsaPublicKey = await DB.getKeyFromIndexDB(ECDSA_PUBLIC_KEY)
        return new this(new ECDSAPublicKey(ecdsaPublicKey), new ECDSAPrivateKey(ecdsaPrivateKey))
    }

    constructor(publicKey: ECDSAPublicKey, privateKey: ECDSAPrivateKey) {
        this.privateKey = privateKey
        this.publicKey = publicKey
    }
}

export class ECDSAPublicKey {
    public publicKey: CryptoKey

    constructor(publicKey: CryptoKey) {
        this.publicKey = publicKey
    }

    async export() : Promise<ByteData> {
        return new ByteData(await self.crypto.subtle.exportKey('spki', this.publicKey))
    }

    static async import(spkiKey: ByteData) : Promise<ECDSAPublicKey> {

        const key = await self.crypto.subtle.importKey('spki', spkiKey.arr.buffer,
            ECDSA_OPTIONS, true, [])

        return new this(key)
    }
}

export class ECDSASignature {

    private signature: ByteData

    
    // Copied from https://stackoverflow.com/questions/39554165/ecdsa-signatures-between-node-js-and-webcrypto-appear-to-be-incompatible
    // It generated hex, we could write a more efficient one that jsut works with bytes.
    private toDER(): ByteData {

        // Extract r & s and format it in ASN1 format.
        var signHex = Array.prototype.map.call(this.signature.arr, function (x) {
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
    private lengthOfHex(hex) {
        return ('00' + (hex.length / 2).toString(16)).slice(-2).toString();
    }
}

export class ECDSAPrivateKey {
    public privateKey: CryptoKey

    constructor(privateKey: CryptoKey) {
        this.privateKey = privateKey
    }
}