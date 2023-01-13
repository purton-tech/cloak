use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use db::queries;
use db::types::public::{AuditAccessType, AuditAction};
use db::Pool;
use serde::Deserialize;
use validator::Validate;

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

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let vault_id = queries::vaults::insert()
        .bind(&transaction, &organisation_id, &new_vault.name.as_ref())
        .one()
        .await?;

    queries::vaults::insert_user_vaults()
        .bind(
            &transaction,
            &current_user.user_id,
            &vault_id,
            &new_vault.public_key.as_ref(),
            &new_vault.encrypted_vault_key.as_ref(),
        )
        .await?;

    queries::audit::insert()
        .bind(
            &transaction,
            &current_user.user_id,
            &organisation_id,
            &AuditAction::CreateVault,
            &AuditAccessType::Web,
            &format!("{} vault created", &new_vault.name).as_ref(),
        )
        .await?;

    let envs = queries::environments::setup_environments()
        .bind(&transaction, &vault_id)
        .all()
        .await?;
    for env in envs {
        queries::environments::connect_environment_to_user()
            .bind(&transaction, &current_user.user_id, &env.id)
            .await?;
    }

    transaction.commit().await?;

    crate::layout::redirect_and_snackbar(
        &ui_components::routes::vaults::index_route(team.id),
        "Vault Created",
    )
}
