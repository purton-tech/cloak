import { SideDrawer } from '../../asset-pipeline/side-drawer'

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="delete-secret-controller-"]').forEach(async (row) => {
    const secretId = parseInt(row.id.split('-')[3])

    // Detect when a user clicks a row
    row.addEventListener('click', () => {
        const drawer = document.getElementById('delete-secret-drawer-' + secretId)
        if (drawer instanceof SideDrawer) {
            drawer.open= true
        }
    })
})