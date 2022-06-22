use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;

pub async fn index(
    _current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<&'static str>, CustomError> {
    let client = pool.get().await?;

    let audits = queries::audit::audit(&client).await?;

    Ok(crate::render(|buf| {
        crate::templates::audit::index_html(buf, audits)
    }))
}
