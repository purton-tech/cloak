use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub user_id: i32,
    pub vault_id: i32,
}

pub async fn delete(
    Path(params): Path<(i32, i32)>,
    current_user: Authentication,
    Form(delete_member): Form<DeleteMember>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;
    queries::user_vaults::remove_user_from_vault()
        .bind(
            &transaction,
            &delete_member.vault_id,
            &delete_member.user_id,
            &current_user.user_id,
        )
        .await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &params.0)
        .one()
        .await?;

    // If we remove ourself, redirect to vaults page.
    let url = if delete_member.user_id == current_user.user_id {
        crate::vaults::index_route(team.id)
    } else {
        super::member_route(params.1, params.0)
    };

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&url, "Member Removed From Vault")
}
