use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct ServiceAccount {
    pub id: i32,
    pub vault_id: Option<i32>,
    pub name: String,
    pub vault_name: Option<String>,
    pub ecdh_public_key: String,
    pub encrypted_ecdh_private_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewAccount {
    pub name: String,
    pub ecdh_public_key: String,
    pub encrypted_ecdh_private_key: String,
}

pub struct ConnectAccount {
    pub vault_id: u32,
    pub service_account_id: u32,
}

impl ServiceAccount {
    pub async fn connect(
        pool: &PgPool,
        authenticated_user: &Authentication,
        connect_account: ConnectAccount,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            "
                UPDATE service_accounts 
                SET 
                    vault_id = $1
                WHERE 
                    id = $2
                AND 
                    -- Make sure the user has access to the vault
                    $1 IN (SELECT vault_id from users_vaults WHERE user_id = $3)
                AND user_id = $3
            ",
            connect_account.vault_id as i32,
            connect_account.service_account_id as i32,
            authenticated_user.user_id as i32
        )
        .execute(pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn create(
        pool: &PgPool,
        authenticated_user: &Authentication,
        new_account: NewAccount,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            "
                INSERT INTO 
                    service_accounts (user_id, name, ecdh_public_key, encrypted_ecdh_private_key)
                VALUES($1, $2, $3, $4) 
            ",
            authenticated_user.user_id as i32,
            new_account.name,
            new_account.ecdh_public_key,
            new_account.encrypted_ecdh_private_key
        )
        .execute(pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_all(
        pool: &PgPool,
        authenticated_user: &Authentication,
    ) -> Result<Vec<ServiceAccount>, CustomError> {
        Ok(sqlx::query_as!(
            ServiceAccount,
            r#"
                SELECT 
                    sa.id, sa.vault_id, sa.name, v.name as "vault_name?", 
                    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
                    sa.updated_at, sa.created_at 
                FROM 
                    service_accounts sa
                LEFT JOIN
                    vaults v
                ON 
                    v.id = sa.vault_id
                WHERE 
                    sa.user_id = $1
            "#,
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_by_vault(
        pool: &PgPool,
        vault_id: u32,

        authenticated_user: &Authentication,
    ) -> Result<Vec<ServiceAccount>, CustomError> {
        Ok(sqlx::query_as!(
            ServiceAccount,
            r#"
                SELECT 
                    sa.id, sa.vault_id, sa.name, v.name as "vault_name?", 
                    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
                    sa.updated_at, sa.created_at 
                FROM 
                    service_accounts sa
                LEFT OUTER JOIN
                    vaults v
                ON 
                    v.id = sa.vault_id
                WHERE 
                    sa.vault_id = $1
                -- Make sure the user actually as access to this vault
                AND
                    $2 IN
                        (SELECT user_id 
                        FROM
                            users_vaults
                        WHERE
                            vault_id = $1)
            "#,
            vault_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_by_ecdh_public_key(
        pool: &PgPool,
        ecdh_public_key: String,
    ) -> Result<ServiceAccount, CustomError> {
        Ok(sqlx::query_as!(
            ServiceAccount,
            r#"
                SELECT 
                    sa.id, sa.vault_id, sa.name, v.name as "vault_name?", 
                    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
                    sa.updated_at, sa.created_at 
                FROM 
                    service_accounts sa
                LEFT OUTER JOIN
                    vaults v
                ON 
                    v.id = sa.vault_id
                WHERE sa.ecdh_public_key = $1
            "#,
            ecdh_public_key
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn delete(
        pool: &PgPool,
        service_account_id: u32,
        authenticated_user: &Authentication,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            r#"
                DELETE FROM
                    service_accounts
                WHERE
                    id = $1
                AND
                    user_id = $2
            "#,
            service_account_id as i32,
            authenticated_user.user_id as i32
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM
                    service_account_secrets
                WHERE
                    service_account_id = $1
            "#,
            service_account_id as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
