use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::{Extension, Path}, response::Html};
use deadpool_postgres::Pool;

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &organisation_id).await?;

    let audits = queries::audit::audit(&client , &(current_user.user_id as i32)).await?;

    Ok(crate::render(|buf| {
        crate::templates::audit::index_html(buf, audits, &team)
    }))
}
