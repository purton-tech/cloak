use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteServiceAccount {
    pub service_account_id: u32,
}

pub async fn delete(
    authentication: Authentication,
    Form(delete_service_account): Form<DeleteServiceAccount>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    models::ServiceAccount::delete(
        &pool,
        delete_service_account.service_account_id,
        authentication.user_id,
    )
    .await?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    DeleteServiceAccountForm(service_account_id: u32, service_account_name: String) {

        form.m_form[method="post", action=super::DELETE] {
            side_drawer[label=format!("Delete Service Account ({})?", service_account_name),
                id=format!("delete-account-drawer-{}", service_account_id)] {

                template[slot="body"] {
                    input[type="hidden", name="service_account_id", value=service_account_id.to_string()] {}
                }

                template[slot="footer"] {
                    button.a_button.auto.danger[slot="footer", type = "submit"] { "Delete Service Account" }
                }
            }
        }

    }
}
