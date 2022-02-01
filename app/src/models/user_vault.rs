use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct UserVault {
    pub vault_id: i32,
    pub user_id: i32,
    pub encrypted_vault_key: String,
}

pub struct UserDetails {
    pub vault_id: i32,
    pub user_id: i32,
    pub email: String,
}

impl UserVault {
    // With an authenticated user get one of their vaults.
    pub async fn get(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_vault_id: u32,
    ) -> Result<UserVault, CustomError> {
        Ok(sqlx::query_as!(
            UserVault,
            "
                SELECT 
                    vault_id, user_id, encrypted_vault_key  
                FROM users_vaults 
                WHERE 
                    user_id = $1 AND vault_id = $2
            ",
            authenticated_user.user_id as i32,
            idor_vault_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    // Given an authenticated user fetch all the users for a vault.
    // We need to make sure the user actually has access to the vault as the
    // vault_id coud have been tampered with.
    pub async fn get_users(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_vault_id: u32,
    ) -> Result<Vec<UserDetails>, CustomError> {
        Ok(sqlx::query_as!(
            UserDetails,
            "
                SELECT 
                    uv.vault_id, uv.user_id, u.email  
                FROM users_vaults uv
                LEFT JOIN users u ON u.id = uv.user_id
                WHERE 
                    uv.user_id = $1 AND uv.vault_id = $2
                AND
                    -- Make sure the user has access to this vault
                    $1 IN (SELECT user_id FROM users_vaults WHERE vault_id = $2)
            ",
            authenticated_user.user_id as i32,
            idor_vault_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}
