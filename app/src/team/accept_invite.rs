use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::cornucopia::queries;
use axum::{
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use deadpool_postgres::Pool;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
pub struct Invite {
    invite_selector: String,
    invite_validator: String,
}

pub async fn invite(
    Query(invite): Query<Invite>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    accept_invitation(
        &pool,
        &current_user,
        &invite.invite_selector,
        &invite.invite_validator,
    )
    .await?;

    Ok(Redirect::to("/app/team".parse()?))
}

pub async fn accept_invitation(
    pool: &Pool,
    current_user: &Authentication,
    invitation_selector: &str,
    invitation_verifier: &str,
) -> Result<(), CustomError> {
    let invitation_verifier = base64::decode_config(invitation_verifier, base64::URL_SAFE)
        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE);

    let client = pool.get().await?;

    let invitation = queries::invitations::get_invitation(&client, invitation_selector).await?;

    if invitation.invitation_verifier_hash == invitation_verifier_hash_base64 {
        let user =
            queries::users::get_dangerous(&client, &(current_user.user_id as i32)).await?;

        // Make sure the user accepting the invitation is the user that we emailed
        if user.email == invitation.email {

            let user = queries::users::get_by_email_dangerous(&client, &user.email).await?;

            queries::organisations::add_user_to_organisation(&client, &user.id, &invitation.organisation_id)
                .await?;

            queries::invitations::delete_invitation(
                &client,
                &invitation.email,
                &invitation.organisation_id,
            )
            .await?;
        }
    }

    Ok(())
}
