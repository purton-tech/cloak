import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault, ByteData } from '../../asset-pipeline/cryptography/vault'

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

    private updateUsers() {

        let users = []

        this.querySelectorAll('input[type="checkbox"]:checked').forEach(element => {
            if(element instanceof HTMLInputElement) {
                users.push(element.value)
            }
        })

        // For each user we are adding, we need to encrypt the vault key on their behalf.


        alert(users)
    }
}

customElements.define('add-member', AddMember);