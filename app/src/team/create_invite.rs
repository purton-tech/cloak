use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::invitation;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use lettre::Message;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewInvite {
    #[validate(length(min = 1, message = "The email is mandatory"))]
    pub email: String,
}

pub async fn create_invite(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
    authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash =
        invitation::Invitation::create(&pool, &authentication, &new_invite.email).await?;

    let invitation_selector_base64 = invite_hash.1;
    let invitation_verifier_base64 = invite_hash.0;

    if let Some(smtp_config) = &config.smtp_config {
        let url = format!(
            "{}/app/team/accept_invite/{}/{}",
            smtp_config.domain, invitation_selector_base64, invitation_verifier_base64
        );

        let email = Message::builder()
            .from(smtp_config.from_email.clone())
            .to(new_invite.email.parse().unwrap())
            .subject("You to a Cloak Team")
            .body(
                format!(
                    "
                        Click {} to accept the invite
                    ",
                    url
                )
                .trim()
                .to_string(),
            )
            .unwrap();

        crate::email::send_email(&config, email)
    }

    crate::layout::redirect_and_snackbar(super::INDEX, "Invitation Created")
}

markup::define! {
    InviteUserPage {
        form.m_form[id="create-invite-form", method = "post",
            action=super::CREATE_INVITE] {
            invite_user[label="Invite User"] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }

                    fieldset {
                        label[for="email"] { "Email" }
                        input[type="email", autocomplete="off", required="", name="email"] {}
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type="submit"] { "Create Invitation" }
                    button.a_button.auto.danger { "Cancel" }
                }
            }
        }
    }
}
