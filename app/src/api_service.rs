use crate::authentication;
use crate::models;
use app::vault::*;
use sqlx::PgPool;
use tonic::{Code, Request, Response, Status};

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
        let authenticated_user = authenticate(&request).await?;

        //let req = request.into_inner();

        dbg!(&authenticated_user);

        let vaults = models::Vault::get_all(&self.pool, authenticated_user.user_id).await?;

        let mut secrets: Vec<SecretResponse> = Default::default();

        for vault in vaults.iter() {
            let zecrets =
                models::Secret::get_all(&self.pool, authenticated_user.user_id, vault.id as u32)
                    .await?;
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

const X_USER_ID: &str = "x-user-id";

// We have 2 types of authentication
// 1. If we have a header set to "authentication-type" then envoy with have set a x-user-id
// 2. If it is not set then we must have an API-KEY which we can use to get the user if.
async fn authenticate<T>(req: &Request<T>) -> Result<authentication::Authentication, Status> {
    if let Some(api_key) = req.metadata().get(X_USER_ID) {
        let user_id = api_key
            .to_str()
            .map_err(|_| Status::new(Code::Internal, "x-user-id not found"))?;

        let user_id: u32 = user_id
            .parse::<u32>()
            .map_err(|_| Status::new(Code::Internal, "x-user-id not parseable as unsigned int"))?;

        Ok(authentication::Authentication { user_id })
    } else {
        Err(Status::new(
            Code::PermissionDenied,
            "You need to set an API Key",
        ))
    }
}
