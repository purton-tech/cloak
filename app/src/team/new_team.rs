use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewTeam {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new_team(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Form(new_team): Form<NewTeam>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    let org_id =
        queries::organisations::insert_organisation(&client, &(current_user.user_id as i32))
            .await?;

    let roles = vec![
        types::public::Role::Administrator,
        types::public::Role::Collaborator,
    ];

    queries::organisations::insert_user_into_org(
        &client,
        &(current_user.user_id as i32),
        &org_id,
        &roles,
    )
    .await?;

    queries::organisations::set_name(
        &client,
        &(current_user.user_id as i32),
        &org_id,
        &new_team.name,
    )
    .await?;

    crate::layout::redirect_and_snackbar(&super::switch_route(organisation_id), "New Team Created")
}
