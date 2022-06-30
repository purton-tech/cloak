use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AddMember {
    pub user_id: i32,
    pub wrapped_vault_key: String,
    #[validate(length(min = 1, message = "The ecdh_public_key is mandatory"))]
    pub ecdh_public_key: String,
    // Comma separated list of environemnt id's
    pub environments: String,
}

pub async fn add(
    Path(id): Path<i32>,
    current_user: Authentication,
    Form(add_member): Form<AddMember>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    // The environments we have selected for the ser come in as a comma
    // separated list of ids.
    let envs: Vec<i32> = add_member
        .environments
        .split(",")
        .map(|e| e.parse::<i32>().unwrap_or(-1))
        .filter(|e| *e != -1)
        .collect();

    // Do an IDOR check, does this user have access to the vault. This will
    // blow up if we don't
    queries::vaults::get(&client, &id, &(current_user.user_id as i32)).await?;

    queries::user_vaults::insert(
        &client,
        &add_member.user_id,
        &id,
        &add_member.ecdh_public_key,
        &add_member.wrapped_vault_key,
    )
    .await?;

    for env in envs {
        queries::environments::connect_environment_to_user(&client, &add_member.user_id, &env)
            .await?;
    }

    Ok(Redirect::to(&super::member_route(id)))
}
