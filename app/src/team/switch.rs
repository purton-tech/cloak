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
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    let teams = queries::organisations::get_teams(&client, &(current_user.user_id as i32)).await?;

    let mut buf = Vec::new();
    crate::templates::team::switch_html(&mut buf, "Your Vaults", teams, &team).unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}
