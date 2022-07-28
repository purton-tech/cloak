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

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let users = queries::organisations::get_users(
        &transaction,
        &(current_user.user_id as i32),
        &organisation_id,
    )
    .await?;

    let permissions: Vec<queries::rbac::Permissions> =
        queries::rbac::permissions(&transaction, &(current_user.user_id as i32), &organisation_id)
            .await?;

    let can_manage_team = permissions
        .iter()
        .any(|p| p.permission == types::public::Permission::ManageTeam);

    let user = queries::users::get_dangerous(&transaction, &(current_user.user_id as i32)).await?;

    let invites = queries::invitations::get_all(&transaction, &organisation_id).await?;

    let initials =
        crate::layout::initials(&user.email, user.first_name.clone(), user.last_name.clone());

    Ok(crate::render(|buf| {
        crate::templates::team::index_html(
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
