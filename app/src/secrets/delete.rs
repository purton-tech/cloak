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
    DeleteSecretForm(secret_id: u32, vault_id: u32, secret_name: String) {

        form.m_form[method="post", action=super::delete_route(*vault_id)] {
            side_drawer[label=format!("Delete Secret ({})?", secret_name),
                id=format!("delete-secret-drawer-{}", secret_id)] {

                template[slot="body"] {
                    input[type="hidden", name="secret_id", value=secret_id.to_string()] {}
                }
                template[slot="footer"] {
                    button.a_button.auto.danger[type = "submit"] { "Delete Secret" }
                }
            }
        }

    }
}
