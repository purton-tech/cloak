use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path(idor_vault_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    // Blow up if the user doesn't have access to the vault
    queries::user_vaults::get(&client, &(current_user.user_id as i32), &idor_vault_id).await?;

    let members = queries::user_vaults::get_users_dangerous(&client, &idor_vault_id).await?;

    let non_members =
        queries::user_vaults::get_non_members_dangerous(&client, &idor_vault_id, &org.id).await?;

    let user_vault =
        queries::user_vaults::get(&client, &(current_user.user_id as i32), &idor_vault_id).await?;

    let environments =
        queries::environments::get_all(&client, &user_vault.vault_id, &(current_user.user_id as i32))
            .await?;

    let mut buf = Vec::new();
    crate::templates::service_accounts::empty_html(&mut buf, "Your Vaults").unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}