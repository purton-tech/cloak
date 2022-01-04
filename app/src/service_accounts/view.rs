markup::define! {
    ViewServiceAccount<'a>(service_account: &'a crate::models::ServiceAccount) {
        sl_drawer[label=format!("View {}", service_account.name), id=format!("view-service-account-{}", service_account.id)] {
            table.m_table {
                tr {
                    td { "Name" }
                    td { {service_account.name} }
                }
                tr {
                    td { "ECDH Public Key" }
                    td { {service_account.ecdh_public_key} }
                }
                tr {
                    td { "ECDH Private Key" }
                    td { {service_account.encrypted_ecdh_private_key} }
                }
            }
        }
        script {
            {markup::raw(format!("document.getElementById('service-account-row-{}').addEventListener('click',
                () => document.getElementById('view-service-account-{}').show());",
                service_account.id, service_account.id))}
        }
    }
}
