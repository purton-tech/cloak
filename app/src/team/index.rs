use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types;
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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let users = queries::organisations::get_users()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let permissions: Vec<types::public::Permission> = queries::rbac::permissions()
        .bind(
            &transaction,
            &(current_user.user_id as i32),
            &organisation_id,
        )
        .all()
        .await?;

    let can_manage_team = permissions
        .iter()
        .any(|p| p == &types::public::Permission::ManageTeam);

    let user = queries::users::get()
        .bind(&transaction, &(current_user.user_id as i32))
        .one()
        .await?;

    let invites = queries::invitations::get_all()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let initials =
        crate::layout::initials(&user.email, user.first_name.clone(), user.last_name.clone());

    Ok(crate::render(|buf| {
        crate::ructe::templates::team::index_html(
            buf,
            &initials,
            users,
            invites,
            &team,
            &user,
            can_manage_team,
        )
    }))
}
