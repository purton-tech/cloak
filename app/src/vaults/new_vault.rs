use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub encrypted_vault_key: String,
    #[validate(length(min = 1, message = "Where did the vault key go?"))]
    pub public_key: String,
}

pub async fn new(
    authentication: Authentication,
    Form(new_vault): Form<NewVault>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    let vault = models::vault::NewVault {
        name: new_vault.name,
        encrypted_vault_key: new_vault.encrypted_vault_key,
        ecdh_public_key: new_vault.public_key,
    };

    models::vault::Vault::create(&pool, &authentication, vault).await?;

    crate::layout::redirect_and_snackbar(super::INDEX, "Vault Created")
}

markup::define! {
    VaultForm {

        form.m_form[method = "post", action=super::NEW] {
            new_vault[label="Add Vault"] {
                template[slot="body"] {
                    p {
                        "Vaults keep related secrets together.
                        For example you could have a vault called My Project with all
                        the secrets related to your project."
                    }

                    fieldset {
                        label[for="name"] { "Name *" }
                        input[type="text", required="", name="name"] {}
                        span.a_help_text { "Give your vault a name" }

                        label[for="encrypted_vault_key"] { "Wrapped AES Key" }
                        textarea[rows="4", required="", readonly="", name="encrypted_vault_key", id="new-vault-key"] {}
                        span.a_help_text { "The key for this vault" }

                        label[for="public_key"] { "ECDH Public Key" }
                        input[id="public-key", type="text", required="", name="public_key"] {}
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Vault" }
                }
            }
        }

    }
}
