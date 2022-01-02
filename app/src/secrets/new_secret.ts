import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import { Vault, Cipher } from '../../asset-pipeline/vault'

let newSecretButton = document.getElementById('new-secret')

if (newSecretButton) {
    newSecretButton.addEventListener('click', async event => {

        let element = newSecretButton.previousElementSibling.firstChild
        if (element instanceof SlDrawer) {
            element.show()
        }
    })
}

let createSecretButton = document.getElementById('create-secret')

if (createSecretButton) {
    createSecretButton.addEventListener('click', async event => {
        event.preventDefault()

        const secretNameInput = document.getElementById('secret-name')
        const secretValueInput = document.getElementById('secret-value')
        const secretForm = document.getElementById('add-secret-form')
        const vaultKeyInput = document.getElementById('vault-key')

        if (secretNameInput instanceof HTMLInputElement &&
            secretValueInput instanceof HTMLInputElement &&
            vaultKeyInput instanceof HTMLInputElement &&
            secretForm instanceof HTMLFormElement) {
            const enc = new TextEncoder(); // always utf-8

            console.log(secretForm.checkValidity())
            if (secretForm.checkValidity()) {
                try {
                    const vaultCipher = Cipher.fromString(vaultKeyInput.value)
                    const vaultKey = await Vault.unwrapKey(vaultCipher)

                    const cipher = await Vault.aesEncrypt(enc.encode(secretNameInput.value), vaultKey)
                    secretNameInput.value = cipher.string
                    const cipher2 = await Vault.aesEncrypt(enc.encode(secretValueInput.value), vaultKey)
                    secretValueInput.value = cipher2.string
                    secretForm.submit()
                } catch (err) {
                    if (err instanceof Error) {
                        console.log(err.message)
                    }
                }
            }
        }
    })
}