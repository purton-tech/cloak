import { Vault, Cipher } from '../../asset-pipeline/cryptography/vault'

const component = document.getElementById('secrets-table-controller');

if(component instanceof HTMLDivElement) {

    const wrappedVaultKeyInput = component.querySelector("#wrapped-vault-key")

    if(wrappedVaultKeyInput instanceof HTMLInputElement) {
        const wrappedVaultKeyCipher = Cipher.fromString(wrappedVaultKeyInput.value)
        const vaultKeyPromise = async () => {
            return await Vault.unwrapKey(wrappedVaultKeyCipher)
        }

        component.querySelectorAll('.secrets_table .cipher').forEach(async (span) => {

            const vaultKey = await vaultKeyPromise()

            if(span instanceof HTMLSpanElement) {
                const spanCipher = Cipher.fromString(span.innerText)
                const decryptedByteData = await Vault.aesDecrypt(spanCipher, vaultKey)

                const dec = new TextDecoder(); // always utf-8

                span.innerText = dec.decode(decryptedByteData.arr)
            }
        })
    }
}