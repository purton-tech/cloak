use crate::cornucopia::queries;
use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SetName {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String
}

pub async fn set_name(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetName>,
) -> Result<impl IntoResponse, CustomError> {

    let client = pool.get().await?;

    queries::organisations::set_name(
        &client,
        &(current_user.user_id as i32),
        &organisation_id,
        &set_name.name,
    )
    .await?;


    crate::layout::redirect_and_snackbar(&super::index_route(organisation_id), "Team Name Updated")
}
