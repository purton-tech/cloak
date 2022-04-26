use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteSecret {
    pub secret_id: i32,
}

pub async fn delete(
    Path(vault_id): Path<u32>,
    current_user: Authentication,
    Form(delete_secret): Form<DeleteSecret>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let secret = queries::secrets::get(
        &client,
        &delete_secret.secret_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::secrets::delete_secret(
        &client,
        &delete_secret.secret_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::secrets::delete_service_account(&client, &secret.name_blind_index, &secret.vault_id)
        .await?;

    crate::layout::redirect_and_snackbar(&super::secret_route(vault_id as i32), "Secret Deleted")
}

markup::define! {
    DeleteSecretForm<'a>(
        secret_id: u32,
        vault_id: u32,
        secret_name: String,
        user_vault: &'a queries::user_vaults::Get) {

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
