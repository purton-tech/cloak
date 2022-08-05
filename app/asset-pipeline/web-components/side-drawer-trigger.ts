import { SideDrawer } from './side-drawer'

// We can create a trigger to open drawers
document.querySelectorAll('[data-drawer-target]').forEach(async (row) => {
    // Detect when a user clicks a row
    row.addEventListener('click', (event) => {

        event.stopImmediatePropagation()
        const target = row.getAttribute('data-drawer-target')
        if(target != null) {
            const drawer = document.getElementById(target)
            console.log(drawer)
            if (drawer instanceof SideDrawer) {
                drawer.open = true
            }
        }
    })
})