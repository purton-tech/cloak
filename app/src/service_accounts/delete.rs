use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteServiceAccount {
    pub service_account_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Form(idor_delete_service_account): Form<DeleteServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::service_accounts::delete_service_account(
        &client,
        &idor_delete_service_account.service_account_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &AuditAction::ConnectServiceAccount,
        &AuditAccessType::Web,
        &format!("Service account {} deleted", idor_delete_service_account.service_account_id)
    )
    .await?;

    Ok(Redirect::to(super::INDEX))
}
