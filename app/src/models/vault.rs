use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct Vault {
    pub id: i32,
    pub name: String,
    pub encrypted_ecdh_private_key: String,
    pub ecdh_public_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Vault {
    // Only call this if you are sure the user has access.
    pub async fn get_dangerous(pool: &PgPool, vault_id: u32) -> Result<Vault, CustomError> {
        Ok(sqlx::query_as!(
            Vault,
            "
                SELECT 
                    id, name, encrypted_ecdh_private_key, ecdh_public_key, updated_at, created_at
                FROM 
                    vaults
                WHERE
                    id = $1 
            ",
            vault_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_vault_id: u32,
    ) -> Result<Vault, CustomError> {
        Ok(sqlx::query_as!(
            Vault,
            "
                SELECT 
                    id, name, encrypted_ecdh_private_key, ecdh_public_key, updated_at, created_at
                FROM 
                    vaults
                WHERE
                    id = $1 
                AND
                    user_id = $2
            ",
            idor_vault_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_all(
        pool: &PgPool,
        authenticated_user: &Authentication,
    ) -> Result<Vec<Vault>, CustomError> {
        Ok(sqlx::query_as!(
            Vault,
            "
                SELECT 
                    id, name, encrypted_ecdh_private_key, ecdh_public_key, updated_at, created_at
                FROM 
                    vaults
                WHERE
                    user_id = $1
            ",
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}
