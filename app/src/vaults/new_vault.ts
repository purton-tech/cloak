import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'

let newVaultButton = document.getElementById('new-vault')

newVaultButton.addEventListener('click', event => {
    let element = newVaultButton.previousElementSibling.firstChild
    if (element instanceof SlDrawer) {
        element.show()

        document.getElementById('new-vault-key').innerText = "here"
    }
})