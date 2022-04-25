use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::models::organisation;
use deadpool_postgres::Pool;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct Invitation {
    pub id: i32,
    pub organisation_id: i32,
    pub email: String,
    pub invitation_selector: String,
    pub invitation_verifier_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Invitation {
    pub async fn create(
        pool: &Pool,
        current_user: &Authentication,
        email: &str,
    ) -> Result<(String, String), CustomError> {
        let client = pool.get().await?;

        let org = queries::organisations::get_primary_organisation(
            &client,
            &(current_user.user_id as i32),
        )
        .await?;

        let invitation_selector = rand::thread_rng().gen::<[u8; 8]>();
        let invitation_selector_base64 =
            base64::encode_config(invitation_selector, base64::URL_SAFE);
        let invitation_verifier = rand::thread_rng().gen::<[u8; 24]>();
        let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
        let invitation_verifier_hash_base64 =
            base64::encode_config(invitation_verifier_hash, base64::URL_SAFE);
        let invitation_verifier_base64 =
            base64::encode_config(invitation_verifier, base64::URL_SAFE);

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
                organisation::Organisation::add_user_dangerous(
                    pool,
                    &invitation.email,
                    invitation.organisation_id,
                )
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

    pub async fn get_all(
        pool: &Pool,
        current_user: &Authentication,
    ) -> Result<Vec<queries::invitations::GetAll>, CustomError> {
        let client = pool.get().await?;

        let org = queries::organisations::get_primary_organisation(
            &client,
            &(current_user.user_id as i32)
        )
        .await?;

        Ok(queries::invitations::get_all(&client, &org.id).await?)
    }
}
