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
    Path(vault_id): Path<i32>,
    current_user: Authentication,
    Form(delete_member): Form<DeleteMember>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;
    queries::user_vaults::remove_user_from_vault(
        &client,
        &delete_member.user_id,
        &delete_member.vault_id,
        &(current_user.user_id as i32),
    )
    .await?;

    // If we remove ourself, redirect to vaults page.
    let url = if delete_member.user_id == (current_user.user_id as i32) {
        crate::vaults::INDEX.to_string()
    } else {
        super::member_route(vault_id)
    };

    crate::layout::redirect_and_snackbar(&url, "Member Removed From Vault")
}
