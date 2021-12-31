use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_vault_key: String,
}

pub async fn new(
    authentication: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    dbg!(&new_vault);

    let vault = sqlx::query!(
        "
            INSERT INTO 
                vaults (user_id, name)
            VALUES($1, $2) 
            RETURNING id
        ",
        authentication.user_id as i32,
        new_vault.name,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    sqlx::query!(
        "
            INSERT INTO 
                users_vaults (user_id, vault_id, encrypted_vault_key)
            VALUES($1, $2, $3) 
        ",
        authentication.user_id as i32,
        vault.id,
        new_vault.encrypted_vault_key
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}
