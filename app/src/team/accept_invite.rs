use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct Invite {
    invite_selector: String,
    invite_validator: String,
}

pub async fn invite(
    Path(invite): Path<Invite>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let team_id = accept_invitation(
        &pool,
        &current_user,
        &invite.invite_selector,
        &invite.invite_validator,
    )
    .await?;

    Ok(Redirect::to(&super::switch_route(team_id)))
}

pub async fn accept_invitation(
    pool: &Pool,
    current_user: &Authentication,
    invitation_selector: &str,
    invitation_verifier: &str,
) -> Result<i32, CustomError> {
    let invitation_verifier = base64::decode_config(invitation_verifier, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        base64::encode_config(invitation_verifier_hash, base64::URL_SAFE_NO_PAD);

    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let invitation = queries::invitations::get_invitation(&transaction, invitation_selector).await?;

    if invitation.invitation_verifier_hash == invitation_verifier_hash_base64 {
        let user = queries::users::get_dangerous(&transaction, &(current_user.user_id as i32)).await?;

        // Make sure the user accepting the invitation is the user that we emailed
        if user.email == invitation.email {
            let user = queries::users::get_by_email_dangerous(&transaction, &user.email).await?;

            queries::organisations::add_user_to_organisation(
                &transaction,
                &user.id,
                &invitation.organisation_id,
                &invitation.roles,
            )
            .await?;

            // I the user has not set their name yet, we do it for them based on the invitation.
            if (None, None) == (user.first_name, user.last_name) {
                queries::users::set_name(
                    &transaction,
                    &(current_user.user_id as i32),
                    &invitation.first_name,
                    &invitation.last_name,
                )
                .await?;
            }

            queries::invitations::delete_invitation(
                &transaction,
                &invitation.email,
                &invitation.organisation_id,
            )
            .await?;
        }
    }

    transaction.commit().await?;

    Ok(invitation.organisation_id)
}
