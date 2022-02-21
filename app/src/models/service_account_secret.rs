use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
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

    // This method is called when a service account is created (before it has a vault id)
    // And whenever a secret is added to a vault.
    pub async fn create(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_secrets: Vec<ServiceAccountSecret>,
    ) -> Result<(), CustomError> {
        for secret in &idor_secrets {
            // Is the service accounbt connected to a vault
            let service_account = models::service_account::ServiceAccount::get_dangerous(
                pool,
                secret.service_account_id as u32,
            )
            .await?;

            // If the vault is already connected we can do an IDOR check
            // And see if the user actually has access to the vault.
            if let Some(vault_id) = service_account.vault_id {
                sqlx::query!(
                    "
                        SELECT user_id 
                        FROM 
                            users_vaults 
                        WHERE 
                            user_id = $1
                        AND
                            vault_id = $2
                    ",
                    authenticated_user.user_id as i32,
                    vault_id
                )
                .fetch_one(pool)
                .await?;
            }

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
