use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ConnectServiceAccount {
    pub vault_id: u32,
    pub service_account_id: u32,
}

pub async fn connect(
    _authentication: Authentication,
    Form(vault): Form<ConnectServiceAccount>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "
            UPDATE service_accounts 
            SET 
                vault_id = $1
            WHERE 
                id = $2
        ",
        vault.vault_id as i32,
        vault.service_account_id as i32
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    ViewServiceAccount<'a>(
        service_account: &'a crate::models::service_account::ServiceAccount,
        vaults: &'a Vec<models::vault::Vault>) {

        form.m_form {
            side_drawer[label=format!("View {}", service_account.name),
                id=format!("view-service-account-row-{}", service_account.id)] {

                @if service_account.vault_id.is_some() {
                    template[slot="body"] {

                        fieldset {
                            label[for="public_key"] { "ECDH Public Key" }
                            textarea[
                                id=format!("ecdh-public-key-{}", service_account.id),
                                rows="6", type="text",
                                name="public_key"] {
                                    {service_account.ecdh_public_key.clone()}
                                }

                            label[for="encrypted_private_key"] { "ECDH Private Key" }
                            textarea[rows="8", required="", readonly="",
                                name="encrypted_private_key",
                                id=format!("wrapped-ecdh-private-key-{}", service_account.id)] {
                                {service_account.encrypted_ecdh_private_key}
                            }
                            span.a_help_text { "The key for this service account" }
                        }
                    }

                    template[slot="footer"] {}
                } else {

                    template[slot="body"] {
                        fieldset {
                            label[for="secret"] { "ECDH Public Key" }
                            select[id=format!("vault-select-{}", service_account.id)] {
                                option { {"Select..."} }
                                @for vault in *vaults {
                                    option[value=vault.id] { {vault.name} }
                                }

                                input[id=format!("service-account-key-{}", service_account.id), type="hidden",
                                    value=service_account.encrypted_ecdh_private_key.clone(),
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
