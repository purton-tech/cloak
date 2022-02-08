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
    async fn get_service_account(
        &self,
        request: Request<GetServiceAccountRequest>,
    ) -> Result<Response<GetServiceAccountResponse>, Status> {
        let req = request.into_inner();

        let service_account = models::service_account::ServiceAccount::get_by_ecdh_public_key(
            &self.pool,
            req.ecdh_public_key,
        )
        .await?;

        if let Some(vault_id) = service_account.vault_id {
            let _vault = models::vault::Vault::get_dangerous(&self.pool, vault_id as u32).await?;

            let secrets = models::service_account_secret::ServiceAccountSecret::get_all_dangerous(
                &self.pool,
                service_account.id as u32,
            )
            .await?;

            let secrets = secrets
                .into_iter()
                .map(|secret| ServiceAccountSecret {
                    encrypted_name: secret.name,
                    name_blind_index: secret.name_blind_index,
                    encrypted_secret_value: secret.secret,
                    ecdh_public_key: secret.ecdh_public_key,
                })
                .collect();

            let response = GetServiceAccountResponse {
                service_account_id: service_account.id as u32,
                secrets,
            };

            return Ok(Response::new(response));
        }

        Err(Status::invalid_argument(
            "This service account is not attached to a vault",
        ))
    }

    async fn get_vault(
        &self,
        request: Request<GetVaultRequest>,
    ) -> Result<Response<GetVaultResponse>, Status> {
        let authenticated_user = authenticate(&request).await?;

        let req = request.into_inner();

        dbg!(&authenticated_user);

        let secrets =
            models::secret::Secret::get_all(&self.pool, &authenticated_user, req.vault_id).await?;
        let vault =
            models::vault::Vault::get(&self.pool, &authenticated_user, req.vault_id).await?;
        let user_vault =
            models::user_vault::UserVault::get(&self.pool, &authenticated_user, req.vault_id)
                .await?;
        let service_accounts = models::service_account::ServiceAccount::get_by_vault(
            &self.pool,
            req.vault_id,
            &authenticated_user,
        )
        .await?;

        let secrets = secrets
            .into_iter()
            .map(|s| Secret {
                encrypted_name: s.name,
                name_blind_index: s.name_blind_index,
                encrypted_secret_value: s.secret,
            })
            .collect();

        let service_accounts = service_accounts
            .into_iter()
            .map(|s| ServiceAccount {
                service_account_id: s.id as u32,
                public_ecdh_key: s.ecdh_public_key,
            })
            .collect();

        let response = GetVaultResponse {
            name: vault.name,
            user_vault_encrypted_vault_key: user_vault.encrypted_vault_key,
            user_vault_public_ecdh_key: user_vault.ecdh_public_key,
            secrets,
            service_accounts,
        };

        Ok(Response::new(response))
    }

    async fn create_secrets(
        &self,
        request: Request<CreateSecretsRequest>,
    ) -> Result<Response<CreateSecretsResponse>, Status> {
        dbg!(&request);

        let authenticated_user = authenticate(&request).await?;

        let service_account = request.into_inner();

        let mut secrets: Vec<models::service_account_secret::ServiceAccountSecret> =
            Default::default();

        for account_secret in service_account.account_secrets {
            let ecdh_public_key = account_secret.public_ecdh_key.clone();
            for secret in account_secret.secrets {
                secrets.push(models::service_account_secret::ServiceAccountSecret {
                    id: 0,
                    service_account_id: account_secret.service_account_id as i32,
                    name: secret.encrypted_name,
                    name_blind_index: secret.name_blind_index,
                    secret: secret.encrypted_secret_value,
                    ecdh_public_key: ecdh_public_key.clone(),
                })
            }
        }

        models::service_account_secret::ServiceAccountSecret::create(
            &self.pool,
            &authenticated_user,
            secrets,
        )
        .await?;

        let response = CreateSecretsResponse {};

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
