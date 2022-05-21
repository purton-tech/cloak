use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;

pub async fn index(
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    let users =
        queries::organisations::get_users(&client, &(current_user.user_id as i32), &org.id).await?;

    let invites = queries::invitations::get_all(&client, &org.id).await?;

    let teams = queries::organisations::get_teams(&client, &(current_user.user_id as i32)).await?;

    let mut buf = Vec::new();
    crate::templates::team::index_html(&mut buf, "Your Vaults", users, invites, teams).unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}
