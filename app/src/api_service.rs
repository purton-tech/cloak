use app::vault::vault_server::Vault;
use app::vault::{CreateVaultRequest, ListVaultsRequest, ListVaultsResponse, VaultResponse};
use tonic::{Request, Response, Status};

pub struct VaultService {}

#[tonic::async_trait]
impl Vault for VaultService {
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
        let response = ListVaultsResponse {
            vaults: vec![VaultResponse {
                name: "Harrry".to_string(),
            }],
        };

        Ok(Response::new(response))
    }
}
