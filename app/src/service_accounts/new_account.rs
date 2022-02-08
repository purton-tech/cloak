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
pub struct NewServiceAccount {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_private_key: String,
}

pub async fn new(
    authenticated_user: Authentication,
    Form(new_service_account): Form<NewServiceAccount>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    let new_account = models::service_account::NewAccount {
        name: new_service_account.name,
        ecdh_public_key: new_service_account.public_key,
        encrypted_ecdh_private_key: new_service_account.encrypted_private_key,
    };

    models::service_account::ServiceAccount::create(&pool, &authenticated_user, new_account)
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
                        input[id="secret-name", type="text", required="", name="name"] {}

                        label[for="secret"] { "ECDH Public Key" }
                        input[id="public-key", type="text", required="", name="public_key"] {}

                        label[for="kry"] { "Wrapped ECDH Private Key" }
                        textarea[rows="8", required="", readonly="", name="encrypted_private_key", id="private-key"] {}
                        span.a_help_text { "The key for this service account" }

                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Service Account" }
                }
            }
        }

    }
}
