import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { Vault, Cipher, ByteData, AESKey, ECDHKeyPair, ECDHPublicKey } from '../../asset-pipeline/cryptography/vault'

class AddMember extends SideDrawer {

    constructor() {
        super()

        this.querySelector('button.danger').addEventListener('click', event => {
            event.preventDefault()
            this.open = false
            return false
        })

        this.querySelector('button.success').addEventListener('click', event => {
            event.preventDefault()
            this.updateUsers()
            return false
        })

        // Initiate the button that opens this drawer
        let newSecretButton = document.getElementById('add-member')
        if (newSecretButton) {
            newSecretButton.addEventListener('click', async event => {
                this.open = true
            })
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

customElements.define('add-member', AddMember);