const template = document.createElement('template');

template.innerHTML = `
<div class="snackbar-container">
    <p>This is the message</p>
</div>
`

export class SnackBar extends HTMLElement {

    constructor() {
        super()
        const templateNode = template.cloneNode(true)

        if(templateNode instanceof HTMLTemplateElement) {
            const templateDocument = templateNode.content
            this.appendChild(templateDocument)
        }
    }
}

customElements.define('snack-bar', SnackBar);