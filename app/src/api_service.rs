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
    ) -> Result<Response<CreateVaultResponse>, Status> {
        let response = CreateVaultResponse {
            name: "Test".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn list_vaults(
        &self,
        _request: Request<ListVaultsRequest>,
    ) -> Result<Response<ListVaultsResponse>, Status> {
        let vaults = models::Vault::get_all(&self.pool, 1).await?;

        let vaults: Vec<VaultResponse> = vaults
            .iter()
            .map(|v| VaultResponse {
                name: v.name.clone(),
            })
            .collect();

        let response = ListVaultsResponse { vaults };

        Ok(Response::new(response))
    }

    async fn list_secrets(
        &self,
        request: Request<ListSecretsRequest>,
    ) -> Result<Response<ListSecretsResponse>, Status> {
        let req = request.into_inner();

        dbg!(&req);

        let vaults = models::Vault::get_all(&self.pool, 1).await?;

        let mut secrets: Vec<SecretResponse> = Default::default();

        for vault in vaults.iter() {
            let zecrets = models::Secret::get_all(&self.pool, 1, vault.id).await?;
            for secret in zecrets.iter() {
                secrets.push(SecretResponse {
                    encrypted_name: secret.name.clone(),
                    encrypted_secret_value: secret.name.clone(),
                });
            }
        }

        let response = ListSecretsResponse { secrets };

        Ok(Response::new(response))
    }
}
