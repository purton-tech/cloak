use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub organisation_id: i32,
    pub user_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(delete_member): Form<DeleteMember>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::organisations::remove_user(
        &client,
        &delete_member.user_id,
        &delete_member.organisation_id,
        &(current_user.user_id as i32),
    )
    .await?;

    crate::layout::redirect_and_snackbar("/app/team", "User Removed")
}
