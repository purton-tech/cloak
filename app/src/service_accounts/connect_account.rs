use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};
use axum::extract::Path;
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
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(connect_form): Form<ConnectServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    queries::service_accounts::connect(
        &client,
        &connect_form.vault_id,
        &connect_form.environment_id,
        &connect_form.service_account_id,
        &(current_user.user_id as i32),
    )
    .await?;

    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &AuditAction::ConnectServiceAccount,
        &AuditAccessType::Web,
        &format!("Service account {} connected", &connect_form.service_account_id)
    )
    .await?;

    Ok(Redirect::to(&super::index_route(team.id)))
}
