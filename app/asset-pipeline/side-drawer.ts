const template = document.createElement('template');

template.innerHTML = `
<div class="drawer drawer--end drawer--fixed drawer--has-footer" part="base">
    <div part="overlay" class="drawer__overlay" tabindex="-1">
    </div>
    <div part="panel" class="drawer__panel" role="dialog" aria-modal="true"  tabindex="0">
        <div class="drawer__body">
        </div>
        <footer part="footer" class="drawer__footer">
            <slot name="footer"></slot>
        </footer>
    </div>
</div>
`

export class SideDrawer extends HTMLElement {

    constructor() {
        super();
        const body = this.querySelector("template[slot='body']").cloneNode(true)
        const templateNode = template.cloneNode(true)

        if(templateNode instanceof HTMLTemplateElement && body instanceof HTMLTemplateElement) {

            const drawerBody = templateNode.content.querySelector(".drawer__body")
            drawerBody.appendChild(body.content)
    
            this.appendChild(templateNode.content)

        }
    }

    static get observedAttributes() {
        return ['open'];
    }

    get open(): Boolean {
        return Boolean(this.getAttribute('open'))
    }

    set open(value: Boolean) {
        this.setAttribute('open', value.toString())
    }

    attributeChangedCallback(name: string, oldVal: string, newVal: string) {
        if (oldVal !== newVal) {
            switch (name) {
                case 'open':
                    this.open = new Boolean(newVal);
                    if(this.open) {
                        this.querySelector('.drawer').classList.remove('drawer--open')
                        this.querySelector('.drawer').classList.add('drawer--open')
                    } else {
                        this.querySelector('.drawer').classList.remove('drawer--open')
                    }
                    break;
            }
        }
    }
}

customElements.define('side-drawer', SideDrawer);