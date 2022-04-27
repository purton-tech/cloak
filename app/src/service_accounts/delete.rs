use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteServiceAccount {
    pub service_account_id: i32,
}

pub async fn delete(
    current_user: Authentication,
    Form(idor_delete_service_account): Form<DeleteServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::service_accounts::delete_service_account(
        &client,
        &idor_delete_service_account.service_account_id,
        &(current_user.user_id as i32),
    )
    .await?;

    // TODO - Danger this is an IDOR issue
    queries::service_accounts::delete_service_account_secrets(
        &client,
        &idor_delete_service_account.service_account_id,
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
