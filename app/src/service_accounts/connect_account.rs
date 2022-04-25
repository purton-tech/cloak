use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct ConnectServiceAccount {
    pub vault_id: i32,
    pub service_account_id: i32,
}

pub async fn connect(
    current_user: Authentication,
    Form(connect_form): Form<ConnectServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::service_accounts::connect(
        &client,
        &connect_form.vault_id,
        &connect_form.service_account_id,
        &(current_user.user_id as i32),
    )
    .await?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    ConnectServiceAccountDrawer<'a>(
        service_account: &'a queries::service_accounts::GetAll,
        vaults: &'a Vec<queries::vaults::GetAll>) {

        connect_account[label=format!("View {}", service_account.account_name),
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
