use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct ServiceAccountSecret {
    pub id: i32,
    pub service_account_id: i32,
    pub name: String,
    pub name_blind_index: String,
    pub secret: String,
    pub ecdh_public_key: String,
}

impl ServiceAccountSecret {
    // Do not pass in any parameters that can be tampered with into this method
    pub async fn get_all_dangerous(
        pool: &PgPool,
        service_account_id: u32,
    ) -> Result<Vec<ServiceAccountSecret>, CustomError> {
        Ok(sqlx::query_as!(
            ServiceAccountSecret,
            "
                SELECT  
                    id, service_account_id, name, name_blind_index, secret, ecdh_public_key 
                FROM 
                    service_account_secrets 
                WHERE 
                    service_account_id = $1
            ",
            service_account_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn create(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_secrets: Vec<ServiceAccountSecret>,
    ) -> Result<(), CustomError> {
        for secret in &idor_secrets {
            // Is the service account attached to one of the users vaults
            // If not, this will blow up.
            sqlx::query!(
                "
                        SELECT sa.id 
                        FROM service_accounts sa
                        LEFT JOIN vaults v ON v.id = sa.vault_id
                        WHERE 
                            -- Only vaults the user has access to.
                            v.id IN (
                                SELECT user_id 
                                FROM users_vaults 
                                WHERE user_id = $1)
                        AND
                            sa.id = $2
                    ",
                authenticated_user.user_id as i32,
                secret.service_account_id,
            )
            .fetch_one(pool)
            .await?;

            // If yes, save the secret
            sqlx::query!(
                "
                        INSERT INTO service_account_secrets
                            (service_account_id, name, name_blind_index, secret, ecdh_public_key)
                        VALUES
                            ($1, $2, $3, $4, $5)
                    ",
                secret.service_account_id,
                secret.name,
                secret.name_blind_index,
                secret.secret,
                secret.ecdh_public_key
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
