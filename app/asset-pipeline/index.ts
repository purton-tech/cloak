import './index.scss'
import '@shoelace-style/shoelace/dist/themes/light.css';
import '@shoelace-style/shoelace/dist/components/tab-panel/tab-panel.js';
import '@shoelace-style/shoelace/dist/components/tab-group/tab-group.js';
import '@shoelace-style/shoelace/dist/components/tab/tab.js';
import '@shoelace-style/shoelace/dist/components/drawer/drawer.js';

import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'

document.querySelectorAll('.drawer-opener').forEach(item => {
    item.addEventListener('click', event => {
        let element = item.previousElementSibling.firstChild
        if(element instanceof SlDrawer) {
            element.show()  
        }  
    })
})