use crate::errors::CustomError;
use sqlx::PgPool;

// Our models
pub struct Vault {
    pub id: i32,
    pub name: String,
}

impl Vault {
    pub async fn get_all(pool: PgPool, _user_id: u32) -> Result<Vec<Vault>, CustomError> {
        Ok(sqlx::query_as!(
            Vault,
            "
                SELECT id, name FROM vaults
            "
        )
        .fetch_all(&pool)
        .await?)
    }
}

pub struct Secret {
    pub id: i32,
    pub name: String,
    pub secret: String,
}

impl Secret {
    pub async fn get_all(
        pool: PgPool,
        _user_id: u32,
        vault_id: i32,
    ) -> Result<Vec<Secret>, CustomError> {
        Ok(sqlx::query_as!(
            Secret,
            "
                SELECT  id, name, secret 
                FROM secrets WHERE vault_id = $1
            ",
            vault_id
        )
        .fetch_all(&pool)
        .await?)
    }
}
