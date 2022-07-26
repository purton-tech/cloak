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
) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    let users = queries::organisations::get_users(
        &client,
        &(current_user.user_id as i32),
        &organisation_id,
    )
    .await?;

    let user = queries::users::get_dangerous(&client, &(current_user.user_id as i32)).await?;

    let invites = queries::invitations::get_all(&client, &organisation_id).await?;

    let initials = crate::layout::initials(&user.email, user.first_name.clone(), user.last_name.clone());

    Ok(crate::render(|buf| {
        crate::templates::team::index_html(buf, "Team", &initials, users, invites, &team, &user)
    }))
}
