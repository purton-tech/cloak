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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let teams = queries::organisations::get_teams()
        .bind(&transaction, &(current_user.user_id as i32))
        .all()
        .await?;

    let user = queries::users::get()
        .bind(&transaction, &(current_user.user_id as i32))
        .one()
        .await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    Ok(crate::render(|buf| {
        crate::ructe::templates::team::switch_html(buf, &initials, teams, &team)
    }))
}
