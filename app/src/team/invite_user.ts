import { SideDrawer } from '../../asset-pipeline/side-drawer'

class InviteUser extends SideDrawer {

    private emailInput : HTMLInputElement

    constructor() {
        super()

        this.emailInput = this.querySelector("input[type='email']")

        this.querySelector('button.danger').addEventListener('click', event => {
            event.preventDefault()
            this.open = false
            return false
        })

        this.querySelector('button.success').addEventListener('click', event => {
            event.preventDefault()
            this.generateInvite()
            return false
        })

        // Initiate the button that opens this drawer
        let newSecretButton = document.getElementById('invite-user')
        if (newSecretButton) {
            newSecretButton.addEventListener('click', async event => {

                let element = newSecretButton.previousElementSibling.firstChild
                if (element instanceof InviteUser) {
                    element.open = true
                }
            })
        }
    }

    private generateInvite() {

    }
}

customElements.define('invite-user', InviteUser);