const template = document.createElement('template');

template.innerHTML = `
<div class="snackbar-container">
</div>
`

const COOKIE_NAME = 'flash_aargh'

export class SnackBar extends HTMLElement {

    constructor() {
        super()
        const templateNode = template.cloneNode(true)

        const message = this.getCookie(COOKIE_NAME)
        if (templateNode instanceof HTMLTemplateElement && message != null) {
            const templateDocument = templateNode.content
            this.appendChild(templateDocument)

            const div = this.querySelector('.snackbar-container')
            if(div instanceof HTMLDivElement) {
                const p = document.createElement('p')
                const text = document.createTextNode(message);
                p.appendChild(text)
                div.appendChild(p)
                this.deleteCookie(COOKIE_NAME)
            }

            setInterval(() => {
                if (div instanceof HTMLDivElement) {
                    div.classList.add('close')
                }
            }, 4000);
        }

    }

    getCookie(name: string): string {
        const nameLenPlus = (name.length + 1);
        return document.cookie
            .split(';')
            .map(c => c.trim())
            .filter(cookie => {
                return cookie.substring(0, nameLenPlus) === `${name}=`;
            })
            .map(cookie => {
                return decodeURIComponent(cookie.substring(nameLenPlus));
            })[0] || null;
    }

    deleteCookie(name: string) {
        document.cookie = name+'=; Max-Age=-99999999;';  
    }
}

customElements.define('snack-bar', SnackBar);