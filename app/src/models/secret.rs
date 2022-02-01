use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct Secret {
    pub id: i32,
    pub vault_id: i32,
    pub name: String,
    pub name_blind_index: String,
    pub secret: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Secret {
    pub async fn get_all(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_vault_id: u32,
    ) -> Result<Vec<Secret>, CustomError> {
        Ok(sqlx::query_as!(
            Secret,
            "
                SELECT  
                    id, vault_id, name, name_blind_index, secret,
                    updated_at, created_at  
                FROM secrets WHERE vault_id = $1
                AND
                    vault_id 
                IN
                    (SELECT vault_id 
                    FROM
                        users_vaults
                    WHERE
                        user_id = $2)
            ",
            idor_vault_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_secret_id: u32,
    ) -> Result<Secret, CustomError> {
        Ok(sqlx::query_as!(
            Secret,
            "
                SELECT  
                    id, vault_id, name, name_blind_index, secret,
                    updated_at, created_at  
                FROM secrets WHERE id = $1
                AND
                    vault_id 
                IN
                    (SELECT vault_id 
                    FROM
                        users_vaults
                    WHERE
                        user_id = $2)
            ",
            idor_secret_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(
        pool: &PgPool,
        idor_secret_id: u32,
        authenticated_user: &Authentication,
    ) -> Result<(), CustomError> {
        let secret = Secret::get(pool, authenticated_user, idor_secret_id).await?;

        sqlx::query!(
            r#"
                DELETE FROM
                    secrets
                WHERE
                    id = $1
                AND
                    vault_id 
                IN
                    (SELECT vault_id 
                    FROM
                        users_vaults
                    WHERE
                        user_id = $2)
            "#,
            idor_secret_id as i32,
            authenticated_user.user_id as i32
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM
                    service_account_secrets
                WHERE
                    name_blind_index = $1
                AND
                    service_account_id
                IN
                    (SELECT id FROM service_accounts WHERE vault_id = $2)
            "#,
            secret.name_blind_index,
            secret.vault_id as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
