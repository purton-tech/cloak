import { SideDrawer } from '../../asset-pipeline/web-components/side-drawer'

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {// Configure all the drawers for each service account.
        document.querySelectorAll('[id^="delete-vault-"]').forEach(async (row) => {
            const vaultId = parseInt(row.id.split('-')[2])
        
            // Detect when a user clicks a row
            row.addEventListener('click', (event) => {
                event.stopImmediatePropagation()
                const drawer = document.getElementById('delete-vault-drawer-' + vaultId)
                if (drawer instanceof SideDrawer) {
                    drawer.open = true
                }
            })
        })
    }
})