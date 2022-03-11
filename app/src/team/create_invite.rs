use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::invitation;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewInvite {
    #[validate(length(min = 1, message = "The email is mandatory"))]
    pub email: String,
    #[validate(length(min = 1, message = "The invitation is mandatory"))]
    pub invitation: String,
}

pub async fn create_invite(
    Path(org): Path<u32>,
    Extension(pool): Extension<PgPool>,
    Form(new_invite): Form<NewInvite>,
    authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    invitation::Invitation::create(
        &pool,
        &authentication,
        org,
        new_invite.email,
        new_invite.invitation,
    )
    .await?;

    crate::layout::redirect_and_snackbar(super::INDEX, "Invitation Created")
}

markup::define! {
    InviteUserPage(organisation_id: i32, user_id: u32) {
        invite_user[label="Invite User",
            user=format!("{}", user_id),
            organisation=format!("{}", organisation_id)] {
            template[slot="body"] {
                form.m_form[id="create-invite-form", method = "post",
                    action=format!("/app/team/create_invite/{}", organisation_id)] {
                    p {
                        "Invite people into your team."
                    }

                    fieldset {
                        label[for="email"] { "Email" }
                        input[type="email", autocomplete="off", required="", name="email"] {}
                        input[id="invite", type="hidden", name="invitation"] {}
                    }
                }
            }

            template[slot="footer"] {
                button.a_button.auto.success { "Create Invitation" }
                button.a_button.auto.danger { "Cancel" }
            }
        }
    }
}
