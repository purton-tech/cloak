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
pub struct NewServiceAccount {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_private_key: String,
}

pub async fn new(
    current_user: Authentication,
    Form(new_service_account): Form<NewServiceAccount>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    queries::service_accounts::insert(
        &client,
        &(current_user.user_id as i32),
        &new_service_account.name,
        &new_service_account.public_key,
        &new_service_account.encrypted_private_key,
    )
    .await?;

    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    ServiceAccountForm {

        form.m_form[method = "post", action=super::NEW] {
            new_account[label="Add Service Accounts"] {
                template[slot="body"] {
                    p {
                        "To allow applications to access secrets without human intervention,
                        We support service accounts. A service account is a non-human account 
                        that is tied to one or more vaults."
                    }

                    fieldset {
                        label[for="name"] { "Name" }
                        input[id="secret-name", type="text", required="", name="name", autocomplete="off"] {}

                        input[id="public-key", type="hidden", required="", name="public_key"] {}

                        input[rows="8", required="", type="hidden", readonly="", name="encrypted_private_key", id="private-key"] {}

                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Service Account" }
                }
            }
        }

    }
}
