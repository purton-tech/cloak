use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path((team_id, vault_id)): Path<(i32, i32)>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<&'static str>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    // Blow up if the user doesn't have access to the vault
    queries::user_vaults::get()
        .bind(&transaction, &(current_user.user_id as i32), &vault_id)
        .one()
        .await?;

    let members = queries::user_vaults::get_users_dangerous()
        .bind(&transaction, &vault_id)
        .all()
        .await?;

    let non_members = queries::user_vaults::get_non_members_dangerous()
        .bind(&transaction, &team_id, &vault_id)
        .all()
        .await?;

    let user_vault = queries::user_vaults::get()
        .bind(&transaction, &(current_user.user_id as i32), &vault_id)
        .one()
        .await?;

    let environments = queries::environments::get_all()
        .bind(&transaction, &user_vault.vault_id)
        .all()
        .await?;

    let user = queries::users::get_dangerous()
        .bind(&transaction, &(current_user.user_id as i32))
        .one()
        .await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    Ok(crate::render(|buf| {
        crate::templates::members::index_html(
            buf,
            &initials,
            user_vault,
            members,
            non_members,
            environments,
            &team,
        )
    }))
}
