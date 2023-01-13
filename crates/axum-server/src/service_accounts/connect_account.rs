use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::Path;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use db::queries;
use db::types::public::{AuditAccessType, AuditAction};
use db::Pool;
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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    queries::service_accounts::connect()
        .bind(
            &transaction,
            &connect_form.vault_id,
            &connect_form.environment_id,
            &connect_form.service_account_id,
            &current_user.user_id,
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
                "Service account {} connected",
                &connect_form.service_account_id
            )
            .as_ref(),
        )
        .await?;

    transaction.commit().await?;

    Ok(Redirect::to(
        &ui_components::routes::service_accounts::index_route(team.id),
    ))
}
