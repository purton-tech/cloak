use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::types::public::{AuditAccessType, AuditAction};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct RenameVault {
    pub vault_id: i32,
    pub name: String,
}

pub async fn rename(
    Extension(pool): Extension<Pool>,
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(idor_rename_vault): Form<RenameVault>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    queries::vaults::rename()
        .bind(
            &transaction,
            &idor_rename_vault.name.as_ref(),
            &idor_rename_vault.vault_id,
        )
        .await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &current_user.user_id,
            &organisation_id,
            &AuditAction::RenameVault,
            &AuditAccessType::Web,
            &format!("{} vault renamed", idor_rename_vault.name).as_ref(),
        )
        .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_components::routes::vaults::index_route(team.id),
        "Vault Renamed",
    )
}
