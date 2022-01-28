use crate::{authentication::Authentication, errors::CustomError, models::organisation};
use axum::{
    extract::Extension,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use sqlx::PgPool;

pub static INDEX: &str = "/app/post_registration";
pub static REDIRECT_URL: &str = "/app/vaults";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(post_registration))
}

// After a user has ,loggede in or registered check they have an entry in
// the organisation table. If not, then create one.
pub async fn post_registration(
    authenticated_user: Authentication,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    if organisation::Organisation::get_primary_org(&pool, &authenticated_user)
        .await
        .is_err()
    {
        organisation::Organisation::create(&pool, &authenticated_user).await?;
    }

    Ok(Redirect::to(REDIRECT_URL.parse()?))
}
