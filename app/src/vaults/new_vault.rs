use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;
use crate::cornucopia::types::public::{AuditAction, AuditAccessType};

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_vault_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
}

pub async fn new(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let vault_id =
        queries::vaults::insert(&transaction, &organisation_id, &new_vault.name).await?;

    queries::vaults::insert_user_vaults(
        &transaction,
        &(current_user.user_id as i32),
        &vault_id,
        &new_vault.public_key,
        &new_vault.encrypted_vault_key,
    )
    .await?;

    queries::audit::insert(
        &transaction,
        &(current_user.user_id as i32),
        &organisation_id,
        &AuditAction::CreateVault,
        &AuditAccessType::Web,
        &format!("{} vault created", &new_vault.name)
    )
    .await?;

    let envs = queries::environments::setup_environments(&transaction, &vault_id).await?;
    for env in envs {
        queries::environments::connect_environment_to_user(
            &transaction,
            &(current_user.user_id as i32),
            &env.id,
        )
        .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(&super::index_route(team.id), "Vault Created")
}
