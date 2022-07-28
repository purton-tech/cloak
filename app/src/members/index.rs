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

    let team = queries::organisations::organisation(&transaction, &team_id).await?;

    // Blow up if the user doesn't have access to the vault
    queries::user_vaults::get(&transaction, &(current_user.user_id as i32), &vault_id).await?;

    let members = queries::user_vaults::get_users_dangerous(&transaction, &vault_id).await?;

    let non_members =
        queries::user_vaults::get_non_members_dangerous(&transaction, &vault_id, &team_id).await?;

    let user_vault =
        queries::user_vaults::get(&transaction, &(current_user.user_id as i32), &vault_id).await?;

    let environments =
        queries::environments::get_all(&transaction, &user_vault.vault_id, &(current_user.user_id as i32))
            .await?;

    let user = queries::users::get_dangerous(&transaction, &(current_user.user_id as i32)).await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    Ok(crate::render(|buf| {
        crate::templates::members::index_html(
            buf, 
            &initials,
            user_vault,
            members,
            non_members,
            environments,
            &team
        )
    }))
}