import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'

class InviteUser extends SideDrawer {

    constructor() {
        super()

        this.querySelector('button.danger').addEventListener('click', event => {
            event.preventDefault()
            this.open = false
            return false
        })

        // Initiate the button that opens this drawer
        let newSecretButton = document.getElementById('invite-user')
        if (newSecretButton) {
            newSecretButton.addEventListener('click', async event => {
                this.open = true
            })
        }
    }
}

customElements.define('invite-user', InviteUser);