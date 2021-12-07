use crate::auth_id::AuthId;
use crate::errors::CustomError;
use crate::vault::vault_server::Vault;
use crate::vault::{CreateVaultRequest, ListVaultsRequest, ListVaultsResponse, VaultResponse};
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
    async fn create_vault(
        &self,
        request: Request<CreateVaultRequest>,
    ) -> Result<Response<()>, Status> {
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

    #[instrument]
    async fn list_vaults(
        &self,
        _request: Request<ListVaultsRequest>,
    ) -> Result<Response<ListVaultsResponse>, Status> {
        //let auth_token = AuthId::from_request(&request)?;
        //let new_vault = request.into_inner();

        info!("Getting alist of vaults");

        let vaults = sqlx::query_as!(
            VaultResponse,
            "
                SELECT name FROM vaults
            "
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        let response = ListVaultsResponse { vaults };

        Ok(Response::new(response))
    }
}
