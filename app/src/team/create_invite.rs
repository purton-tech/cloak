use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types;
use crate::cornucopia::types::public::{AuditAccessType, AuditAction};
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
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
    #[validate(length(min = 1, message = "The first name is mandatory"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "The last name is mandatory"))]
    pub last_name: String,
    pub admin: Option<String>,
}

pub async fn create_invite(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Extension(config): Extension<crate::config::Config>,
    Form(new_invite): Form<NewInvite>,
    authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let invite_hash = create(&pool, &authentication, &new_invite, organisation_id).await?;

    let invitation_verifier_base64 = invite_hash.0;
    let invitation_selector_base64 = invite_hash.1;

    if let Some(smtp_config) = &config.smtp_config {
        let url = format!(
            "{}/app/invite/{}/{}",
            smtp_config.domain, invitation_selector_base64, invitation_verifier_base64
        );

        let body = format!(
            "
                Click {} to accept the invite
            ",
            url
        )
        .trim()
        .to_string();

        let email = Message::builder()
            .from(smtp_config.from_email.clone())
            .to(new_invite.email.parse().unwrap())
            .subject("You are invited to a Cloak Team")
            .body(body)
            .unwrap();

        crate::email::send_email(&config, email)
    }

    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &(current_user.user_id as i32),
            &organisation_id,
            &AuditAction::CreateInvite,
            &AuditAccessType::Web,
            &format!("{} invited", &new_invite.email).as_ref(),
        )
        .await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    crate::layout::redirect_and_snackbar(&super::index_route(team.id), "Invitation Created")
}

pub async fn create(
    pool: &Pool,
    current_user: &Authentication,
    new_invite: &NewInvite,
    organisation_id: i32,
) -> Result<(String, String), CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, current_user).await?;

    let invitation_selector = rand::thread_rng().gen::<[u8; 6]>();
    let invitation_selector_base64 =
        base64::encode_config(invitation_selector, base64::URL_SAFE_NO_PAD);
    let invitation_verifier = rand::thread_rng().gen::<[u8; 8]>();
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE_NO_PAD);
    let invitation_verifier_base64 =
        base64::encode_config(invitation_verifier, base64::URL_SAFE_NO_PAD);

    let roles = if new_invite.admin.is_some() {
        vec![
            types::public::Role::Administrator,
            types::public::Role::Collaborator,
        ]
    } else {
        vec![types::public::Role::Collaborator]
    };

    queries::invitations::insert_invitation()
        .bind(
            &transaction,
            &organisation_id,
            &new_invite.email.as_ref(),
            &new_invite.first_name.as_ref(),
            &new_invite.last_name.as_ref(),
            &invitation_selector_base64.as_ref(),
            &invitation_verifier_hash_base64.as_ref(),
            &roles.as_ref(),
        )
        .await?;

    transaction.commit().await?;

    Ok((invitation_verifier_base64, invitation_selector_base64))
}
