use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewSecret {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    pub environment_id: i32,
    #[validate(length(min = 1, message = "The blind index is mandatory"))]
    pub name_blind_index: String,
    #[validate(length(min = 1, message = "The secret is mandatory"))]
    pub secret: String,
}

pub async fn new(
    Path(organisation_id): Path<i32>,
    Path(id): Path<i32>,
    current_user: Authentication,
    Form(new_secret): Form<NewSecret>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    // This will blow up if the user doesn't have access to the vault
    queries::vaults::get(&client, &id, &(current_user.user_id as i32)).await?;

    queries::secrets::insert(
        &client,
        &id,
        &new_secret.name,
        &new_secret.name_blind_index,
        &new_secret.secret,
        &new_secret.environment_id,
    )
    .await?;

    queries::audit::insert(
        &client,
        &(current_user.user_id as i32),
        &AuditAction::AddSecret,
        &AuditAccessType::Web,
        &format!("Secret created for Vault with ID {}", id)
    )
    .await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    Ok(Redirect::to(&super::index_route(id, team.id)))
}
