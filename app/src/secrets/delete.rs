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
pub struct DeleteSecret {
    pub secret_id: u32,
}

pub async fn delete(
    Path(vault_id): Path<u32>,
    authentication: Authentication,
    Form(delete_secret): Form<DeleteSecret>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    models::secret::Secret::delete(&pool, delete_secret.secret_id, &authentication).await?;

    Ok(Redirect::to(super::secret_route(vault_id as i32).parse()?))
}

markup::define! {
    DeleteSecretForm<'a>(
        secret_id: u32,
        vault_id: u32,
        secret_name: String,
        user_vault: &'a models::user_vault::UserVault) {

        form.m_form[method="post", action=super::delete_route(*vault_id)] {
            side_drawer[label="Delete Secret?",
                id=format!("delete-secret-drawer-{}", secret_id)] {

                template[slot="body"] {
                    p {
                        "Are you sure you want to delete the secret "
                        ecdh_cipher[cipher=secret_name.clone(),
                            "wrapped-aes-key"=user_vault.encrypted_vault_key.clone(),
                            "ecdh-public-key"=user_vault.ecdh_public_key.clone()] {}
                    }
                    input[type="hidden", name="secret_id", value=secret_id.to_string()] {}
                }
                template[slot="footer"] {
                    button.a_button.auto.danger[type = "submit"] { "Delete Secret" }
                }
            }
        }

    }
}
