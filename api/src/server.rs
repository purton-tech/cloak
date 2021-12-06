use crate::errors::CustomError;
use crate::vault::vault_server::Vault;
use crate::vault::{VaultReply, VaultRequest};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct VaultImplementation {
    pub db_pool: PgPool,
}

#[tonic::async_trait]
impl Vault for VaultImplementation {
    async fn create_vault(
        &self,
        request: Request<VaultRequest>,
    ) -> Result<Response<VaultReply>, Status> {
        let user_id = 1;
        let new_vault = request.into_inner();
        println!("Got a request: {:?}", new_vault);

        sqlx::query!(
            "
                INSERT INTO 
                    vaults (
                        user_id, 
                        name)
                VALUES($1, $2) 
            ",
            user_id,
            new_vault.name,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        let reply = VaultReply {
            message: format!("Hello {}!", new_vault.name),
        };

        Ok(Response::new(reply))
    }
}
