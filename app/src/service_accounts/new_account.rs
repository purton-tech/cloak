use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewServiceAccount {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_private_key: String,
}

pub async fn new(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(new_service_account): Form<NewServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    queries::service_accounts::insert(
        &client,
        &organisation_id,
        &new_service_account.name,
        &new_service_account.public_key,
        &new_service_account.encrypted_private_key,
    )
    .await?;

    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &organisation_id,
        &AuditAction::NewServiceAccount,
        &AuditAccessType::Web,
        "Service account created"
    )
    .await?;

    Ok(Redirect::to(&super::index_route(team.id)))
}
