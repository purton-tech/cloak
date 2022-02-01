use crate::errors::CustomError;
use sqlx::PgPool;

pub struct User {
    pub id: i32,
    pub email: String,
    pub ecdsa_public_key: String,
}

impl User {
    pub async fn get_dangerous(pool: &PgPool, user_id: u32) -> Result<User, CustomError> {
        Ok(sqlx::query_as!(
            User,
            "
                SELECT 
                    id, email, ecdsa_public_key
                FROM 
                    users
                WHERE
                    id = $1
            ",
            user_id as i32
        )
        .fetch_one(pool)
        .await?)
    }
}
