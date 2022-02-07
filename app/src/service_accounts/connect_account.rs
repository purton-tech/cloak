use crate::models;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ConnectServiceAccount {
    pub vault_id: u32,
    pub service_account_id: u32,
}

markup::define! {
    ConnectServiceAccountDrawer<'a>(
        service_account: &'a crate::models::service_account::ServiceAccount,
        vaults: &'a Vec<models::vault::Vault>) {

        connect_account[label=format!("View {}", service_account.name),
            "service-account-id"=format!("{}", service_account.id)] {

            template[slot="body"] {
                fieldset {
                    label[for="secret"] { "ECDH Public Key" }
                    select[id=format!("vault-select-{}", service_account.id)] {
                        option { {"Select..."} }
                        @for vault in *vaults {
                            option[value=vault.id] { {vault.name} }
                        }

                        input[id=format!("service-account-public-key-{}", service_account.id), type="hidden",
                            value=service_account.ecdh_public_key.clone(),
                            name="public_key"] {}
                    }
                    span.a_help_text { "The key for this service account" }
                }
            }

            template[slot="footer"] {
                button.a_button.auto.success[id = format!("connect-to-vault-{}",
                    service_account.id)] { "Connect to Vault" }
            }
        }
        // This is the form that gets submitted for connecting to a vault.
        // The secrets were already retrieved and re-encrypted by the Typescript.
        // Here we attach the account and redirect.
        form.m_form[
            method = "post", action=super::CONNECT,
            id=format!("service-account-form-{}", service_account.id)] {
                input[type="hidden", name="service_account_id",
                    value=format!("{}", service_account.id)] {}
                input[type="hidden", name="vault_id",
                    id=format!("service-account-form-vault-id-{}", service_account.id)] {}
        }
    }
}
