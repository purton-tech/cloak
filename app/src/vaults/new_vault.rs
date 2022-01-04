use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_vault_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_private_key: String,
}

pub async fn new(
    authentication: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    let vault = sqlx::query!(
        "
            INSERT INTO 
                vaults (user_id, name, ecdh_public_key, encrypted_ecdh_private_key)
            VALUES($1, $2, $3, $4) 
            RETURNING id
        ",
        authentication.user_id as i32,
        new_vault.name,
        new_vault.public_key,
        new_vault.encrypted_private_key
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    sqlx::query!(
        "
            INSERT INTO 
                users_vaults (user_id, vault_id, encrypted_vault_key)
            VALUES($1, $2, $3) 
        ",
        authentication.user_id as i32,
        vault.id,
        new_vault.encrypted_vault_key
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    VaultForm {

        form.m_form[style="margin-top: 2em", method = "post", action=super::NEW] {
            sl_drawer[label="Add Vault"] {
                p {
                    "Vaults keep related secrets together.
                    For example you could have a vault called My Project with all
                    the secrets related to your project."
                }

                fieldset {
                    label[for="name"] { "Name *" }
                    input[type="text", required="", name="name"] {}
                    span.a_help_text { "Give your vault a name" }

                    label[for="kry"] { "Wrapped AES Key" }
                    textarea[rows="4", required="", readonly="", name="encrypted_vault_key", id="new-vault-key"] {}
                    span.a_help_text { "The key for this vault" }

                    label[for="secret"] { "ECDH Public Key" }
                    input[id="public-key", type="text", required="", name="public_key"] {}

                    label[for="kry"] { "Wrapped ECDH Private Key" }
                    textarea[rows="8", required="", readonly="", name="encrypted_private_key", id="private-key"] {}
                    span.a_help_text { "The key for this service account" }
                }

                button.a_button.auto.success[slot="footer", type = "submit"] { "Create Vault" }
            }
        }

    }
}
