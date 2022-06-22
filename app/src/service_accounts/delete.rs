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

    // TODO - Danger this is an IDOR issue
    queries::service_accounts::delete_service_account_secrets(
        &client,
        &idor_delete_service_account.service_account_id,
    )
    .await?;

    Ok(Redirect::to(super::INDEX))
}
