use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub organisation_id: u32,
    pub user_id: u32,
}

pub async fn delete(
    current_user: Authentication,
    Extension(pool): Extension<PgPool>,
    Form(delete_member): Form<DeleteMember>,
) -> Result<impl IntoResponse, CustomError> {
    models::organisation::Organisation::remove_user(
        &pool,
        &current_user,
        delete_member.organisation_id,
        delete_member.user_id,
    )
    .await?;

    crate::layout::redirect_and_snackbar("/app/team", "User Removed")
}

markup::define! {
    DeleteMemberForm(
        organisation_id: u32,
        user_id: u32,
        email: String
    ) {

        form.m_form[method="post", action=super::DELETE_MEMBER] {
            side_drawer[
                label="Remove this user?",
                id=format!("delete-member-drawer-{}-{}", organisation_id, user_id)
            ] {

                template[slot="body"] {
                    p {
                        {format!("Are you sure you want to remove {} from the team?", email)}
                    }
                    input[
                        type="hidden",
                        name="organisation_id",
                        value=organisation_id.to_string()
                    ] {}
                    input[
                        type="hidden",
                        name="user_id",
                        value=user_id.to_string()
                    ] {}
                }
                template[slot="footer"] {
                    button.a_button.auto.danger[type = "submit"] { "Remove User" }
                }
            }
        }

    }
}
