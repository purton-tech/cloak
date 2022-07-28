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
    Path((organisation_id, vault_id)): Path<(i32, i32)>,
    current_user: Authentication,
    Form(delete_secret): Form<DeleteSecret>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let secret = queries::secrets::get(
        &transaction,
        &delete_secret.secret_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::secrets::delete_secret(
        &transaction,
        &delete_secret.secret_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::audit::insert(
        &transaction,
        &(current_user.user_id as i32),
        &organisation_id,
        &AuditAction::DeleteSecret,
        &AuditAccessType::Web,
        &format!("Secret deleted from Vault with ID {}", vault_id)
    )
    .await?;

    queries::secrets::delete_service_account(&transaction, &secret.name_blind_index, &secret.vault_id)
        .await?;
        
    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&super::index_route(vault_id, team.id), "Secret Deleted")
}
