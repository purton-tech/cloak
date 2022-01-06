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
    async fn get_vault(
        &self,
        request: Request<GetVaultRequest>,
    ) -> Result<Response<GetVaultResponse>, Status> {
        let authenticated_user = authenticate(&request).await?;

        let req = request.into_inner();

        dbg!(&authenticated_user);

        let secrets =
            models::Secret::get_all(&self.pool, authenticated_user.user_id, req.vault_id).await?;
        let vault =
            models::Vault::get(&self.pool, authenticated_user.user_id, req.vault_id).await?;
        let user_vault =
            models::UserVault::get(&self.pool, authenticated_user.user_id, req.vault_id).await?;

        let secrets = secrets
            .into_iter()
            .map(|s| Secret {
                encrypted_name: s.name,
                encrypted_secret_value: s.secret,
            })
            .collect();

        let response = GetVaultResponse {
            name: vault.name,
            encrypted_vault_key: user_vault.encrypted_vault_key,
            secrets,
        };

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
