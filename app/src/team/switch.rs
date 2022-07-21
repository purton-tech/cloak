use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;

pub async fn switch(
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let teams = queries::organisations::get_teams(&client, &(current_user.user_id as i32)).await?;

    let mut buf = Vec::new();
    crate::templates::team::switch_html(&mut buf, "Your Vaults", teams).unwrap();
    let html = format!("{}", String::from_utf8_lossy(&buf));

    Ok(Html(html))
}
