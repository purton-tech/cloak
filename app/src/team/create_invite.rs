use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use lettre::Message;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewInvite {
    #[validate(length(min = 1, message = "The email is mandatory"))]
    pub email: String,
}

pub async fn create_invite(
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
    authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash = create(&pool, &authentication, &new_invite.email).await?;

    let invitation_selector_base64 = invite_hash.1;
    let invitation_verifier_base64 = invite_hash.0;

    if let Some(smtp_config) = &config.smtp_config {
        let url = format!(
            "{}/app/team/accept_invite/?invite_selector={}&invite_validator={}",
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

pub async fn create(
    pool: &Pool,
    current_user: &Authentication,
    email: &str,
) -> Result<(String, String), CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    let invitation_selector = rand::thread_rng().gen::<[u8; 8]>();
    let invitation_selector_base64 = base64::encode_config(invitation_selector, base64::URL_SAFE);
    let invitation_verifier = rand::thread_rng().gen::<[u8; 24]>();
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE);
    let invitation_verifier_base64 = base64::encode_config(invitation_verifier, base64::URL_SAFE);

    queries::invitations::insert_invitation(
        &client,
        &org.id,
        &email,
        &invitation_selector_base64,
        &invitation_verifier_hash_base64,
    )
    .await?;

    Ok((invitation_verifier_base64, invitation_selector_base64))
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
