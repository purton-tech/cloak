import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'

class InviteUser extends SideDrawer {

    constructor() {
        super()

        const cancelButton = this.querySelector('button.danger')

        if(cancelButton) {
            cancelButton.addEventListener('click', event => {
                event.preventDefault()
                this.open = false
                return false
            })
        }

        // Initiate the button that opens this drawer
        let newSecretButton = document.getElementById('invite-user')
        if (newSecretButton) {
            newSecretButton.addEventListener('click', async event => {
                this.open = true
            })
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('invite-user', InviteUser);
    }
})