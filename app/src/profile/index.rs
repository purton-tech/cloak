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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let user = queries::users::get_dangerous(&transaction, &(current_user.user_id as i32)).await?;
    let initials = crate::layout::initials(&user.email, user.first_name.clone(), user.last_name.clone());

    Ok(crate::render(|buf| {
        crate::templates::profile::index_html(buf, &initials, &team, &user)
    }))
}
