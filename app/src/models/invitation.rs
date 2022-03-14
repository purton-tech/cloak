use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::{organisation, user};
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

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
        pool: &PgPool,
        authenticated_user: &Authentication,
        email: &str,
    ) -> Result<(String, String), CustomError> {
        let org = organisation::Organisation::get_primary_org(pool, authenticated_user).await?;

        let invitation_selector = rand::thread_rng().gen::<[u8; 8]>();
        let invitation_selector_base64 =
            base64::encode_config(invitation_selector, base64::URL_SAFE);
        let invitation_verifier = rand::thread_rng().gen::<[u8; 24]>();
        let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
        let invitation_verifier_hash_base64 =
            base64::encode_config(invitation_verifier_hash, base64::URL_SAFE);
        let invitation_verifier_base64 =
            base64::encode_config(invitation_verifier, base64::URL_SAFE);

        sqlx::query!(
            "
                INSERT INTO 
                    invitations (organisation_id, email, invitation_selector, invitation_verifier_hash)
                VALUES($1, $2, $3, $4) 
            ",
            org.id as i32,
            email,
            invitation_selector_base64,
            invitation_verifier_hash_base64,
        )
        .execute(pool)
        .await?;

        Ok((invitation_verifier_base64, invitation_selector_base64))
    }

    pub async fn accept_invitation(
        pool: &PgPool,
        current_user: &Authentication,
        invitation_selector: &str,
        invitation_verifier: &str,
    ) -> Result<(), CustomError> {
        let invitation_verifier = base64::decode_config(invitation_verifier, base64::URL_SAFE)
            .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
        let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
        let invitation_verifier_hash_base64 =
            base64::encode_config(invitation_verifier_hash, base64::URL_SAFE);

        let invitation = sqlx::query_as!(
            Invitation,
            "
                SELECT 
                    id, 
                    organisation_id, 
                    email, 
                    invitation_selector, 
                    invitation_verifier_hash,
                    created_at,
                    updated_at
                FROM 
                    invitations 
                WHERE
                    invitation_selector = $1
            ",
            invitation_selector,
        )
        .fetch_one(pool)
        .await?;

        if invitation.invitation_verifier_hash == invitation_verifier_hash_base64 {
            let user = user::User::get_dangerous(pool, current_user.user_id).await?;

            // Make sure the user accepting the invitation is the user that we emailed
            if user.email == invitation.email {
                organisation::Organisation::add_user_dangerous(
                    pool,
                    &invitation.email,
                    invitation.organisation_id as u32,
                )
                .await?;

                sqlx::query!(
                    r#"
                        DELETE FROM
                            invitations
                        WHERE
                            email = $1
                        AND
                            organisation_id = $2
                    "#,
                    &invitation.email,
                    invitation.organisation_id
                )
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn get_all(
        pool: &PgPool,
        current_user: &Authentication,
    ) -> Result<Vec<Invitation>, CustomError> {
        let org = organisation::Organisation::get_primary_org(pool, current_user).await?;

        Ok(sqlx::query_as!(
            Invitation,
            "
                SELECT  
                    id, 
                    email,
                    invitation_selector, 
                    invitation_verifier_hash,
                    organisation_id,
                    updated_at, 
                    created_at  
                FROM 
                    invitations 
                WHERE organisation_id = $1
            ",
            org.id as i32,
        )
        .fetch_all(pool)
        .await?)
    }
}
