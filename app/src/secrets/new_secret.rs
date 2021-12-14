use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewSecret {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub secret: String,
}

pub async fn new(
    Path(id): Path<i32>,
    _authentication: Authentication,
    Form(new_secret): Form<NewSecret>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "
            INSERT INTO 
                secrets (vault_id, name, secret)
            VALUES($1, $2, $3) 
        ",
        id,
        new_secret.name,
        new_secret.secret,
    )
    .execute(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    Ok(Redirect::to(super::secret_route(id).parse().unwrap()))
}
