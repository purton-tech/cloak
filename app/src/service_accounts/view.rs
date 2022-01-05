use crate::models;

markup::define! {
    ViewServiceAccount<'a>(
        service_account: &'a crate::models::ServiceAccount,
        vaults: &'a Vec<models::Vault>) {

        form.m_form {
            sl_drawer[label=format!("View {}", service_account.name),
                id=format!("view-service-account-row-{}", service_account.id)] {

                @if service_account.vault_id.is_some() {

                    fieldset {
                        label[for="secret"] { "ECDH Public Key" }
                        input[id="public-key", type="text",
                            value=service_account.ecdh_public_key.clone(),
                            name="public_key"] {}

                        label[for="kry"] { "Wrapped ECDH Private Key" }
                        textarea[rows="8", required="", readonly="",
                            name="encrypted_private_key", id="private-key"] {
                            {service_account.encrypted_ecdh_private_key}
                        }
                        span.a_help_text { "The key for this service account" }
                    }
                } else {

                    fieldset {
                        label[for="secret"] { "ECDH Public Key" }
                        select {
                            option { {"Select..."} }
                            @for vault in *vaults {
                                option { {vault.name} }
                            }
                        }
                        span.a_help_text { "The key for this service account" }
                    }
                    button.a_button.auto.success[slot="footer", id = "connect-to-vault"] { "Connect to Vault" }
                }
            }
        }
    }
}
