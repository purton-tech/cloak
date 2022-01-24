import { SideDrawer } from '../../asset-pipeline/side-drawer'

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="delete-account-controller-"]').forEach(async (row) => {
    const serviceAccountId = parseInt(row.id.split('-')[3])

    // Detect when a user clicks a row
    row.addEventListener('click', () => {
        const drawer = document.getElementById('delete-account-drawer-' + serviceAccountId)
        if (drawer instanceof SideDrawer) {
            drawer.open = true
        }
    })
})