import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'

const connectButton = document.getElementById('connect-to-vault')

// document.getElementById('service-account-row-{}').addEventListener('click',
//() => document.getElementById('view-service-account-{}').show());
document.querySelectorAll('[id^="service-account-row-"]').forEach((row) => {
    console.log(row.id)
    row.addEventListener('click', () => {
        const drawer = document.getElementById('view-' + row.id)
        if(drawer instanceof SlDrawer) {
            drawer.show()
        }
    })
})