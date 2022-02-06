import { openDB } from 'idb';
export { Cipher } from './cipher'
export { ByteData } from './byte_data'
export { ECDHKeyPair, ECDHPublicKey, ECDHPrivateKey } from './ecdh_keypair'
export { ECDSAKeyPair, ECDSAPublicKey, ECDSAPrivateKey, ECDSASignature } from './ecdsa_keypair'
export { AESKey } from './aes_key'
import { ByteData } from './byte_data'

// All client side cryptography comes through this class.
export class Vault {



    public static async blindIndex(text: string, id: number): Promise<ByteData> {
        let enc = new TextEncoder();
        const data = enc.encode(text + ':' + id)
        const hash: ArrayBuffer = await crypto.subtle.digest('SHA-256', data)
        return new ByteData(hash.slice(0, 8))
    }

}
