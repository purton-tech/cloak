import { openDB } from 'idb';

const DB_NAME = 'keyval'

export class DB {
    public static async getKeyFromIndexDB(name: string) : Promise<CryptoKey> {
        const db = await openDB('Vault', 1, {
            upgrade(db) {
                db.createObjectStore("keyval");
            },
        })
        const key = await db.get(DB_NAME, name) as CryptoKey
        db.close()
        return key
    }

    public static async storeKeyInIndexDB(name: string, key: CryptoKey) {
        const db = await openDB('Vault', 1, {
            upgrade(db) {
                db.createObjectStore("keyval");
            },
        })
        await db.put(DB_NAME, key, name) 
        db.close()
    }
}