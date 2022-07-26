use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn switch(
    Path(organisation_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    let teams = queries::organisations::get_teams(&client, &(current_user.user_id as i32)).await?;

    let user = queries::users::get_dangerous(&client, &(current_user.user_id as i32)).await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    Ok(crate::render(|buf| {
        crate::templates::team::switch_html(buf, &initials, teams, &team)
    }))
}
