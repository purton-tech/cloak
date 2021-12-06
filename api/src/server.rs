use crate::auth_id::AuthId;
use crate::errors::CustomError;
use crate::vault::vault_server::Vault;
use crate::vault::VaultRequest;
use sqlx::PgPool;
use tonic::{Request, Response, Status};
use tracing::{info, instrument};

pub struct VaultImplementation {
    pub db_pool: PgPool,
}

// Becuse we dont want to print the pool to the logs
impl std::fmt::Debug for VaultImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[tonic::async_trait]
impl Vault for VaultImplementation {
    #[instrument]
    async fn create_vault(&self, request: Request<VaultRequest>) -> Result<Response<()>, Status> {
        let auth_token = AuthId::from_request(&request)?;
        let new_vault = request.into_inner();

        info!("Creating Vault");

        sqlx::query!(
            "
                INSERT INTO 
                    vaults (user_id, name)
                VALUES($1, $2) 
            ",
            auth_token.user_id as i32,
            new_vault.name,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        Ok(Response::new(()))
    }
}
