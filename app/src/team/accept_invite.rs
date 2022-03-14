use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::invitation;
use axum::{
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Invite {
    invite_selector: String,
    invite_validator: String,
}

pub async fn invite(
    Query(invite): Query<Invite>,
    Extension(pool): Extension<PgPool>,
    current_user: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    invitation::Invitation::accept_invitation(
        &pool,
        &current_user,
        &invite.invite_selector,
        &invite.invite_validator,
    )
    .await?;

    Ok(Redirect::to("/app/team".parse()?))
}
