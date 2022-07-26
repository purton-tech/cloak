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
pub struct SetDetails {
    #[validate(length(min = 1, message = "The first name is mandatory"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "The last name is mandatory"))]
    pub last_name: String
}

pub async fn set_details(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(set_name): Form<SetDetails>,
) -> Result<impl IntoResponse, CustomError> {

    let client = pool.get().await?;

    queries::users::set_name(
        &client,
        &(current_user.user_id as i32),
        &set_name.first_name,
        &set_name.last_name,
    )
    .await?;


    crate::layout::redirect_and_snackbar(&crate::profile::index_route(organisation_id), "Details Updated")
}
