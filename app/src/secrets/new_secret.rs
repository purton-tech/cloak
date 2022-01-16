use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewSecret {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub secret: String,
}

pub async fn new(
    Path(id): Path<i32>,
    _authentication: Authentication,
    Form(new_secret): Form<NewSecret>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    dbg!(&new_secret);
    sqlx::query!(
        "
            INSERT INTO 
                secrets (vault_id, name, secret)
            VALUES($1, $2, $3) 
        ",
        id,
        new_secret.name,
        new_secret.secret,
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::secret_route(id).parse().unwrap()))
}

markup::define! {
    NewSecretPage<'a>(user_vault: &'a models::UserVault) {

        form.m_form[id="add-secret-form", style="margin-top: 2em", method = "post",
            action=super::new_route(user_vault.vault_id)] {
            sl_drawer[label="Add Secret", class="add-secret"] {
                p {
                    "Folders keep related secrets together.
                    For example you could have a folder called Database with all
                    the secrets related to database access."
                }

                fieldset {
                    label[for="name"] { "Name" }
                    input[id="secret-name", type="text", required="", name="name"] {}

                    label[for="secret"] { "Secret" }
                    input[id="secret-value", type="text", required="", name="secret"] {}

                }

                // Store the encrypted vault key here, then we can use it in the client to
                // encrypt the secrets we create.
                input[type="hidden",
                    id="vault-key",
                    value=user_vault.encrypted_vault_key.clone()] {}
                input[type="hidden",
                    id="vault-id",
                    value=user_vault.vault_id] {}

                button.a_button.auto.success[slot="footer", id="create-secret"] { "Create Secret" }
            }
        }
    }
}