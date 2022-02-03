import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault, ByteData } from '../../asset-pipeline/vault'

class AddMember extends SideDrawer {

    constructor() {
        super()

        // Initiate the button that opens this drawer
        let newSecretButton = document.getElementById('add-member')
        if (newSecretButton) {
            newSecretButton.addEventListener('click', async event => {
                this.open = true
            })
        }
    }
}

customElements.define('add-member', AddMember);