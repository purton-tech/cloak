use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use db::queries;
use db::types::public::{AuditAccessType, AuditAction};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteServiceAccount {
    pub service_account_id: i32,
}

pub async fn delete(
    Extension(pool): Extension<Pool>,
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(idor_delete_service_account): Form<DeleteServiceAccount>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    queries::service_accounts::delete_service_account()
        .bind(
            &transaction,
            &idor_delete_service_account.service_account_id,
            &organisation_id,
        )
        .await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &current_user.user_id,
            &organisation_id,
            &AuditAction::ConnectServiceAccount,
            &AuditAccessType::Web,
            &format!(
                "Service account {} deleted",
                idor_delete_service_account.service_account_id
            )
            .as_ref(),
        )
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(
        &ui_components::routes::service_accounts::index_route(team.id),
    ))
}
