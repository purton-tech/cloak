mod index;
mod new_vault;
use crate::errors::CustomError;
use sqlx::PgPool;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/vaults";
pub static NEW: &str = "/app/new_vault";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_vault::new))
}

// Our models
pub struct Vault {
    pub name: String,
}

impl Vault {
    pub async fn get_all(pool: PgPool, _user_id: u32) -> Result<Vec<Vault>, CustomError> {
        Ok(sqlx::query_as!(
            Vault,
            "
                SELECT name FROM vaults
            "
        )
        .fetch_all(&pool)
        .await?)
    }
}
