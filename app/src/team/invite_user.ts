import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault, ByteData } from '../../asset-pipeline/vault'

class InviteUser extends SideDrawer {

    private emailInput : HTMLInputElement
    private inviteText : HTMLTextAreaElement

    constructor() {
        super()

        this.emailInput = this.querySelector("input[type='email']")
        this.inviteText = this.querySelector("textarea[name='invite']")

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
                this.open = true
            })
        }
    }

    private generateInvite() {
        this.emailInput.reportValidity()
        if(this.emailInput.validity.valid == true) {

            const email = encodeURIComponent(this.emailInput.value.toLowerCase())
            const date = new Date().getTime()
            const url = `${location.protocol}//${location.host}/invite/${email}/${date}/`

            const data = ByteData.fromText(url)
            const sigPromise = Vault.sign(data)
            sigPromise.then(data => {
                this.inviteText.value = url + data.b64
            })
        }

    }
}

customElements.define('invite-user', InviteUser);