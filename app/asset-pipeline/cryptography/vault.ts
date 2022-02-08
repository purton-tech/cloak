export { Cipher } from './cipher'
export { ByteData } from './byte_data'
export { ECDHKeyPair, ECDHPublicKey, ECDHPrivateKey } from './ecdh_keypair'
export { ECDSAKeyPair, ECDSAPublicKey, ECDSAPrivateKey, ECDSASignature } from './ecdsa_keypair'
export { AESKey } from './aes_key'
import { AESKey } from './aes_key'
import { ByteData } from './byte_data'
import { Cipher } from './cipher'
import { ECDHKeyPair, ECDHPublicKey, ECDHPrivateKey } from './ecdh_keypair'

// All client side cryptography comes through this class.
export class Vault {
    public static async blindIndex(text: string, id: number): Promise<ByteData> {
        let enc = new TextEncoder();
        const data = enc.encode(text + ':' + id)
        const hash: ArrayBuffer = await crypto.subtle.digest('SHA-256', data)
        return new ByteData(hash.slice(0, 8))
    }

    // 1. Get a key agreement between the user and the ECDH public key.
    // 2. The key agreement can decrypt the vault the cipher
    // 3. The cipher is then used to decrypt the wrapped vault key
    public static async decryptVaultKey(wrappedVaultKey: Cipher, ecdhPublicKey: ECDHPublicKey) : Promise<AESKey> {
        const userECDHKeyPair = await ECDHKeyPair.fromBarricade()
        const vaultKey = await userECDHKeyPair.privateKey.unwrapKey(wrappedVaultKey, ecdhPublicKey)
        return vaultKey
    }
}
