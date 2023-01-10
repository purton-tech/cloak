use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use db::queries;
use db::Pool;

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let service_accounts = queries::service_accounts::get_all()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let environments_and_vaults = queries::environments::get_environments_and_vaults()
        .bind(&transaction)
        .all()
        .await?;

    Ok(Html(ui_components::service_accounts::index(
        team.id,
        service_accounts,
        environments_and_vaults,
    )))
}
