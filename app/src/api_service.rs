use crate::cornucopia::queries;
use crate::{authentication, errors::CustomError};
use app::vault::*;
use deadpool_postgres::Pool;
use tonic::{Code, Request, Response, Status};

pub struct VaultService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl app::vault::vault_server::Vault for VaultService {
    async fn get_service_account(
        &self,
        request: Request<GetServiceAccountRequest>,
    ) -> Result<Response<GetServiceAccountResponse>, Status> {
        let req = request.into_inner();
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        let service_account =
            queries::service_accounts::get_by_ecdh_public_key(&client, &req.ecdh_public_key)
                .await
                .map_err(|e| CustomError::Database(e.to_string()))?;

        if let Some(vault_id) = service_account.vault_id {
            queries::vaults::get_dangerous(&client, &vault_id)
                .await
                .map_err(|e| CustomError::Database(e.to_string()))?;

            let secrets =
                queries::service_account_secrets::get_all_dangerous(&client, &service_account.id)
                    .await
                    .map_err(|e| CustomError::Database(e.to_string()))?;

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

        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        let secrets = queries::secrets::get_all(
            &client,
            &(req.vault_id as i32),
            &(authenticated_user.user_id as i32),
        )
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        let vault = queries::vaults::get(
            &client,
            &(req.vault_id as i32),
            &(authenticated_user.user_id as i32),
        )
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        let user_vault = queries::user_vaults::get(
            &client,
            &(authenticated_user.user_id as i32),
            &(req.vault_id as i32),
        )
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        let service_accounts = queries::service_accounts::get_by_vault(
            &client,
            &(req.vault_id as i32),
            &(authenticated_user.user_id as i32),
        )
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

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
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        let authenticated_user = authenticate(&request).await?;

        let service_account = request.into_inner();

        for account_secret in service_account.account_secrets {
            // Get the service account this request is trying to access
            let sa = queries::service_accounts::get_dangerous(
                &client,
                &(account_secret.service_account_id as i32),
            )
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

            // If the vault is already connected we can do an IDOR check
            // And see if the user actually has access to the vault.
            if let Some(vault_id) = sa.vault_id {
                // Blow up, if the user doesn't have access to the vault.
                queries::service_account_secrets::get_users_vaults(
                    &client,
                    &(authenticated_user.user_id as i32),
                    &vault_id,
                )
                .await
                .map_err(|e| CustomError::Database(e.to_string()))?;
            }

            // If yes, save the secret
            for secret in account_secret.secrets {
                queries::service_account_secrets::insert(
                    &client,
                    &(account_secret.service_account_id as i32),
                    &secret.encrypted_name,
                    &secret.name_blind_index,
                    &secret.encrypted_secret_value,
                    &account_secret.public_ecdh_key,
                )
                .await
                .map_err(|e| CustomError::Database(e.to_string()))?;
            }
        }

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
