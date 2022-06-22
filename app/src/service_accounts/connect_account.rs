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
pub struct ConnectServiceAccount {
    pub vault_id: i32,
    pub environment_id: i32,
    pub service_account_id: i32,
}

pub async fn connect(
    current_user: Authentication,
    Form(connect_form): Form<ConnectServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::service_accounts::connect(
        &client,
        &connect_form.vault_id,
        &connect_form.environment_id,
        &connect_form.service_account_id,
        &(current_user.user_id as i32),
    )
    .await?;

    Ok(Redirect::to(super::INDEX))
}
