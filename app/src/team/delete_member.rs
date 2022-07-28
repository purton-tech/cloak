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
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

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
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::organisations::remove_user(
        &transaction,
        &delete_member.user_id,
        &delete_member.organisation_id,
    )
    .await?;

    queries::audit::insert(
        &transaction,
        &(current_user.user_id as i32),
        &delete_member.organisation_id,
        &AuditAction::CreateInvite,
        &AuditAccessType::Web,
        &format!("{} removed from team", &delete_member.user_id)
    )
    .await?;

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar("/app/team", "User Removed")
}
