import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'
import { ByteData, ECDSAKeyPair } from '../../asset-pipeline/cryptography/vault'

class InviteUser extends SideDrawer {

    private emailInput : HTMLInputElement
    private inviteText : HTMLInputElement
    private inviteForm : HTMLFormElement
    private organisationId: number
    private userId: number

    constructor() {
        super()

        this.emailInput = this.querySelector("input[type='email']")
        this.inviteText = this.querySelector("input#invite")
        this.inviteForm = this.querySelector("form#create-invite-form")
        this.organisationId = parseInt(this.getAttribute("organisation"))
        this.userId = parseInt(this.getAttribute("user"))

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

    private async generateInvite() {
        this.emailInput.reportValidity()
        if(this.emailInput.validity.valid == true) {

            const email = this.emailInput.value.toLowerCase()
            const date = new Date().getTime()
            const urlToSign = this.generateUrl(email, date)
            const data = ByteData.fromText(urlToSign)

            const aliceECDSAKeyPair = await ECDSAKeyPair.fromBarricade()
            
            const sigPromise = aliceECDSAKeyPair.privateKey.sign(data)
            const urlToSend = this.generateUrl(encodeURIComponent(email), date)
            sigPromise.then(signature => {
                this.inviteText.value = urlToSend + '&sig=' + encodeURIComponent(signature.toDER().b64)
                this.inviteForm.submit()
            })
        }

    }

    private generateUrl(email : String, time : number) {
        return `${location.protocol}//${location.host}/app/team/invite/`
            + `${this.organisationId}?id=${this.userId}`
            + `&email=${email}&time=${time}`
    }
}

customElements.define('invite-user', InviteUser);