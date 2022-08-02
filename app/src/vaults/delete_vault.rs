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
pub struct DeleteVault {
    pub vault_id: i32,
    pub name: String,
}

pub async fn delete(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(idor_delete_vault): Form<DeleteVault>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let vault = queries::vaults::get(
        &transaction,
        &idor_delete_vault.vault_id,
        &(current_user.user_id as i32),
    )
    .await?;

    if vault.name == idor_delete_vault.name {
        queries::vaults::delete(
            &transaction,
            &idor_delete_vault.vault_id,
            &(current_user.user_id as i32),
        )
        .await?;

        queries::audit::insert(
            &transaction,
            &(current_user.user_id as i32),
            &organisation_id,
            &AuditAction::DeleteVault,
            &AuditAccessType::Web,
            &format!("{} vault deleted", vault.name)
        )
        .await?;

        transaction.commit().await?;
    } else {
        return crate::layout::redirect_and_snackbar(&super::index_route(team.id), "Name did not match");
    }

    crate::layout::redirect_and_snackbar(&super::index_route(team.id), "Vault Deleted")
}
