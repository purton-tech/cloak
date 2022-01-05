import SlDrawer from '@shoelace-style/shoelace/dist/components/drawer/drawer.js'
import * as grpcWeb from 'grpc-web';
import {VaultClient} from '../../asset-pipeline/ApiServiceClientPb';
import {ListSecretsRequest, ListSecretsResponse} from '../../asset-pipeline/api_pb';

// The user wants to connect a service account to a vault
const connectButton = document.getElementById('connect-to-vault')
if(connectButton instanceof HTMLButtonElement) {
    connectButton.addEventListener('click', async event => {
        event.preventDefault()
        alert('here')
    })
}

// Configure all the drawers for each service account.
document.querySelectorAll('[id^="service-account-row-"]').forEach((row) => {
    row.addEventListener('click', () => {
        const drawer = document.getElementById('view-' + row.id)
        if(drawer instanceof SlDrawer) {
            drawer.show()
        }
    })
})