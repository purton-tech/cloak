use crate::errors::CustomError;
use sqlx::PgPool;

// Our models
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

impl ServiceAccount {
    pub async fn get_all(pool: &PgPool, user_id: u32) -> Result<Vec<ServiceAccount>, CustomError> {
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
            user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_by_vault(
        pool: &PgPool,
        vault_id: u32,
        user_id: u32,
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
                AND
                    sa.user_id = $2
            "#,
            vault_id as i32,
            user_id as i32
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
        user_id: u32,
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
            user_id as i32
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

    pub async fn get(pool: &PgPool, user_id: u32, vault_id: u32) -> Result<Vault, CustomError> {
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
            vault_id as i32,
            user_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_all(pool: &PgPool, user_id: u32) -> Result<Vec<Vault>, CustomError> {
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
            user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}

pub struct UserVault {
    pub vault_id: i32,
    pub user_id: i32,
    pub encrypted_vault_key: String,
}

impl UserVault {
    pub async fn get(pool: &PgPool, user_id: u32, vault_id: u32) -> Result<UserVault, CustomError> {
        Ok(sqlx::query_as!(
            UserVault,
            "
                SELECT 
                    vault_id, user_id, encrypted_vault_key  
                FROM users_vaults 
                WHERE 
                    user_id = $1 AND vault_id = $2
            ",
            user_id as i32,
            vault_id as i32
        )
        .fetch_one(pool)
        .await?)
    }
}

pub struct Secret {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Secret {
    pub async fn get_all(
        pool: &PgPool,
        _user_id: u32,
        vault_id: u32,
    ) -> Result<Vec<Secret>, CustomError> {
        Ok(sqlx::query_as!(
            Secret,
            "
                SELECT  
                    id, name, secret ,
                    updated_at, created_at  
                FROM secrets WHERE vault_id = $1
            ",
            vault_id as i32
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete(pool: &PgPool, secret_id: u32, user_id: u32) -> Result<(), CustomError> {
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
            secret_id as i32,
            user_id as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct ServiceAccountSecret {
    pub id: i32,
    pub service_account_id: i32,
    pub name: String,
    pub secret: String,
}

impl ServiceAccountSecret {
    pub async fn get_all(
        pool: &PgPool,
        service_account_id: u32,
    ) -> Result<Vec<ServiceAccountSecret>, CustomError> {
        Ok(sqlx::query_as!(
            ServiceAccountSecret,
            "
                SELECT  
                    id, service_account_id, name, secret 
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
        secrets: Vec<ServiceAccountSecret>,
    ) -> Result<(), CustomError> {
        for secret in secrets {
            sqlx::query!(
                "
                    INSERT INTO service_account_secrets
                        (service_account_id, name, secret)
                    VALUES
                        ($1, $2, $3)
                ",
                secret.service_account_id as i32,
                secret.name,
                secret.secret
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
