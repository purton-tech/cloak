import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="delete-member-"]').forEach(async (row) => {
    const id = parseInt(row.id.split('-')[2])

    // Detect when a user clicks a row
    row.addEventListener('click', () => {
        const drawer = document.getElementById('delete-member-drawer-' + id)
        if (drawer instanceof SideDrawer) {
            drawer.open = true
        }
    })
})