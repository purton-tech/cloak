import { Vault, Cipher, ByteData, AESKey, ECDHKeyPair, ECDHPublicKey } from '../cryptography/vault'

class AddMember extends HTMLElement {

    constructor() {
        super()

        const successButton = this.querySelector('button.success')

        if(successButton) {

            successButton.addEventListener('click', event => {
                event.preventDefault()
                this.updateUsers()
                return false
            })
        } else {
            console.error('Could not fund required elements')
        }
    }

    private async updateUsers() {

        const userSelection = this.querySelector('#user-selection')
        const wrappedKeyInput = this.querySelector('#wrapped-vault-key')
        const ecdhPublicKeyInput = this.querySelector('#ecdh-public-key')

        if(userSelection instanceof HTMLSelectElement 
            && wrappedKeyInput instanceof HTMLInputElement
            && ecdhPublicKeyInput instanceof HTMLInputElement) {
                
            const htmlOption = userSelection.item(userSelection.selectedIndex)
            if(htmlOption instanceof HTMLOptionElement) {
                const selectedECDHPubKey = htmlOption.getAttribute("data-ecdh-pub-key")
                if(selectedECDHPubKey) {
                    const ecdhPublicKey = await ECDHPublicKey.import(ByteData.fromB64(selectedECDHPubKey))
                    const vaultKey = await this.decryptSymmetricVaultKey()
                    const epherealKeyPair = await ECDHKeyPair.fromRandom()
                    const aesKey = await epherealKeyPair.privateKey.deriveAESKey(ecdhPublicKey)
                    const wrappedVaultKey = await aesKey.wrap(vaultKey)
                    const ecdhPublicKeyData = await epherealKeyPair.publicKey.export()
        
                    wrappedKeyInput.value = wrappedVaultKey.string
                    ecdhPublicKeyInput.value = await ecdhPublicKeyData.b64
        
                    const form = document.getElementById("add-team-member")
        
                    if(form instanceof HTMLFormElement) {
                        this.parseEnvironments()
                        form.submit()
                    }
                }
            }
        }
    }

    private async parseEnvironments() {
        var ids = ''
        this.querySelectorAll("input[type='checkbox']:checked").forEach((item) => {
            if(item instanceof HTMLInputElement) {
                ids += item.value + ','
            }
        })
        const envHiddenField = this.querySelector('#environments')
        if(envHiddenField instanceof HTMLInputElement) {
            envHiddenField.value = ids
        }
    }

    private async decryptSymmetricVaultKey(): Promise<AESKey> {
        const ecdhPublicKeyInput = this.querySelector('#user-vault-ecdh-public-key') as HTMLInputElement
        const encryptedVaultKeyInput = this.querySelector('#encrypted-vault-key') as HTMLInputElement
        const vaultKeyCipher = Cipher.fromString(encryptedVaultKeyInput.value)
        const ecdhPublicKey = await ECDHPublicKey.import(ByteData.fromB64(ecdhPublicKeyInput.value))
        return await Vault.decryptVaultKey(vaultKeyCipher, ecdhPublicKey)
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('add-member', AddMember);
    }
})