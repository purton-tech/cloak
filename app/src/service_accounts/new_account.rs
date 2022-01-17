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
pub struct NewServiceAccount {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_private_key: String,
}

pub async fn new(
    authentication: Authentication,
    Form(new_service_account): Form<NewServiceAccount>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "
            INSERT INTO 
                service_accounts (user_id, name, ecdh_public_key, encrypted_ecdh_private_key)
            VALUES($1, $2, $3, $4) 
        ",
        authentication.user_id as i32,
        new_service_account.name,
        new_service_account.public_key,
        new_service_account.encrypted_private_key
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    ServiceAccountForm {

        form.m_form[id="add-secret-form", method = "post", action=super::NEW] {
            sl_drawer[label="Add Service Accounts", class="add-secret"] {
                p {
                    "To allow applications to access secrets without human intervention,
                    We support service accounts. A service account is a non-human account 
                    that is tied to one or more vaults."
                }

                fieldset {
                    label[for="name"] { "Name" }
                    input[id="secret-name", type="text", required="", name="name"] {}

                    label[for="secret"] { "ECDH Public Key" }
                    input[id="public-key", type="text", required="", name="public_key"] {}

                    label[for="kry"] { "Wrapped ECDH Private Key" }
                    textarea[rows="8", required="", readonly="", name="encrypted_private_key", id="private-key"] {}
                    span.a_help_text { "The key for this service account" }

                }

                button.a_button.auto.success[slot="footer", type = "submit"] { "Create Service Account" }
            }
        }

    }
}
