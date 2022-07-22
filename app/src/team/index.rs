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
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    let users = queries::organisations::get_users(
        &client,
        &(current_user.user_id as i32),
        &organisation_id,
    )
    .await?;

    let invites = queries::invitations::get_all(&client, &organisation_id).await?;

    let mut buf = Vec::new();
    crate::templates::team::index_html(&mut buf, "Your Vaults", users, invites, &team).unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}
