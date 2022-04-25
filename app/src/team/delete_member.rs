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
