use crate::models;
use app::vault::*;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct VaultService {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl app::vault::vault_server::Vault for VaultService {
    async fn create_vault(
        &self,
        _request: Request<CreateVaultRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn list_vaults(
        &self,
        _request: Request<ListVaultsRequest>,
    ) -> Result<Response<ListVaultsResponse>, Status> {
        let vaults = models::Vault::get_all(self.pool.clone(), 1).await?;

        let vaults: Vec<VaultResponse> = vaults
            .iter()
            .map(|v| VaultResponse {
                name: v.name.clone(),
            })
            .collect();

        let response = ListVaultsResponse { vaults };

        Ok(Response::new(response))
    }
}
