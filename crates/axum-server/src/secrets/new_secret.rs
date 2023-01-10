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
    Path((organisation_id, id)): Path<(i32, i32)>,
    current_user: Authentication,
    Form(new_secret): Form<NewSecret>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    // This will blow up if the user doesn't have access to the vault
    queries::vaults::get()
        .bind(&transaction, &id, &current_user.user_id)
        .one()
        .await?;

    queries::secrets::insert()
        .bind(
            &transaction,
            &id,
            &new_secret.name.as_ref(),
            &new_secret.name_blind_index.as_ref(),
            &new_secret.secret.as_ref(),
            &new_secret.environment_id,
        )
        .await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &current_user.user_id,
            &organisation_id,
            &AuditAction::AddSecret,
            &AuditAccessType::Web,
            &format!("Secret created for Vault with ID {}", id).as_ref(),
        )
        .await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(&ui_components::routes::secrets::index_route(
        id, team.id,
    )))
}
