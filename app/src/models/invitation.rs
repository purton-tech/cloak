use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::organisation;
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

pub struct Invitation {
    pub id: i32,
    pub organisation_id: i32,
    pub email: String,
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
        let invitation_selector_base64 = base64::encode(invitation_selector);
        let invitation_verifier = rand::thread_rng().gen::<[u8; 24]>();
        let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
        let invitation_verifier_hash_base64 = base64::encode(invitation_verifier_hash);
        let invitation_verifier_base64 = base64::encode(invitation_verifier);

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

    pub async fn delete_dangerous(
        pool: &PgPool,
        email: &str,
        organisation_id: u32,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            r#"
                DELETE FROM
                    invitations
                WHERE
                    email = $1
                AND
                    organisation_id = $2
            "#,
            email,
            organisation_id as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(
        pool: &PgPool,
        authenticated_user: &Authentication,
        organisation_id: u32,
    ) -> Result<Vec<Invitation>, CustomError> {
        Ok(sqlx::query_as!(
            Invitation,
            "
                SELECT  
                    id, 
                    email,
                    organisation_id,
                    updated_at, 
                    created_at  
                FROM 
                    invitations 
                WHERE organisation_id = $1
                    AND
                        organisation_id 
                    IN
                        (SELECT id 
                        FROM
                            organisation_users
                        WHERE
                            user_id = $2)
            ",
            organisation_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}
