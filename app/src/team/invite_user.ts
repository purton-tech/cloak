import { SideDrawer } from '../../asset-pipeline/side-drawer'
import { Vault, ByteData } from '../../asset-pipeline/vault'

class InviteUser extends SideDrawer {

    private emailInput : HTMLInputElement
    private inviteText : HTMLTextAreaElement
    private organisationId: number

    constructor() {
        super()

        this.emailInput = this.querySelector("input[type='email']")
        this.inviteText = this.querySelector("textarea[name='invite']")
        this.organisationId = parseInt(this.getAttribute("organisation"))

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

            const email = this.emailInput.value.toLowerCase()
            const date = new Date().getTime()
            const urlToSign = this.generateUrl(email, date)
            const data = ByteData.fromText(urlToSign)
            const sigPromise = Vault.sign(data)
            const urlToSend = this.generateUrl(encodeURIComponent(email), date)
            sigPromise.then(data => {
                this.inviteText.value = urlToSend + '&sig=' + encodeURIComponent(data.b64)
            })
        }

    }

    private generateUrl(email : String, time : number) {
        return `${location.protocol}//${location.host}/app/team/invite/${this.organisationId}?email=${email}&time=${time}`
    }
}

customElements.define('invite-user', InviteUser);