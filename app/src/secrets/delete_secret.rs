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
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

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

    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &AuditAction::DeleteSecret,
        &AuditAccessType::Web,
        &format!("Secret deleted from Vault with ID {}", vault_id)
    )
    .await?;

    queries::secrets::delete_service_account(&client, &secret.name_blind_index, &secret.vault_id)
        .await?;

    crate::layout::redirect_and_snackbar(&super::secret_route(vault_id as i32), "Secret Deleted")
}
