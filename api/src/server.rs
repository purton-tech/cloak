use tonic::{Request, Response, Status};

use crate::vault::vault_server::Vault;
use crate::vault::{VaultReply, VaultRequest};

#[derive(Debug, Default)]
pub struct VaultImplementation {}

#[tonic::async_trait]
impl Vault for VaultImplementation {
    async fn create_vault(
        &self,
        request: Request<VaultRequest>,
    ) -> Result<Response<VaultReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = VaultReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
