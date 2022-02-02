import { SideDrawer } from '../../asset-pipeline/side-drawer'

const openers = document.querySelectorAll(".open-members-drawer");

openers.forEach(function(opener) {
    opener.addEventListener('click', event => {
        event.stopPropagation()
        let element = opener.previousElementSibling.firstChild
        if(element instanceof SideDrawer) {
            element.open = true
        }
        return false
    })
});

class MembersDrawer extends SideDrawer {

    constructor() {
        super()

        this.querySelector("button[id='add-user-button']").addEventListener('click', event => {
            event.preventDefault()
            console.log('click')
            return false
        })
    }
}

customElements.define('members-drawer', MembersDrawer);