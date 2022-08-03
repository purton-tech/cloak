use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<&'static str>, CustomError> {
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

    let user = queries::users::get()
        .bind(&transaction, &(current_user.user_id as i32))
        .one()
        .await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    if service_accounts.is_empty() {
        Ok(crate::render(|buf| {
            crate::templates::service_accounts::empty_html(buf, &initials, &team)
        }))
    } else {
        Ok(crate::render(|buf| {
            crate::templates::service_accounts::index_html(
                buf,
                &initials,
                service_accounts,
                environments_and_vaults,
                &team,
            )
        }))
    }
}
