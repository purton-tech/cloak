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
}

pub async fn new(
    authentication: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    dbg!(&new_vault);

    sqlx::query!(
        "
            INSERT INTO 
                vaults (user_id, name)
            VALUES($1, $2) 
        ",
        authentication.user_id as i32,
        new_vault.name,
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}
