import { SideDrawer } from '../../asset-pipeline/side-drawer'


class InviteUser extends SideDrawer {
    constructor() {
        super()
    }
}

customElements.define('invite-user', InviteUser);

let newSecretButton = document.getElementById('invite-user')
if (newSecretButton) {
    newSecretButton.addEventListener('click', async event => {

        let element = newSecretButton.previousElementSibling.firstChild
        if (element instanceof InviteUser) {
            console.log('here')
            element.open = true
        }
    })
}