use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types::public::{AuditAccessType, AuditAction};
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    queries::service_accounts::insert()
        .bind(
            &transaction,
            &organisation_id,
            &new_service_account.name.as_ref(),
            &new_service_account.public_key.as_ref(),
            &new_service_account.encrypted_private_key.as_ref(),
        )
        .await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &(current_user.user_id as i32),
            &organisation_id,
            &AuditAction::NewServiceAccount,
            &AuditAccessType::Web,
            &String::from("Service account created").as_ref(),
        )
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(&super::index_route(team.id)))
}
