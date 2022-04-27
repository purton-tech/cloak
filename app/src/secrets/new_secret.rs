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
pub struct NewSecret {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
    #[validate(length(min = 1, message = "The blind index is mandatory"))]
    pub name_blind_index: String,
    #[validate(length(min = 1, message = "The secret is mandatory"))]
    pub secret: String,
}

pub async fn new(
    Path(id): Path<i32>,
    current_user: Authentication,
    Form(new_secret): Form<NewSecret>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    // This will blow up if the user doesn't have access to the vault
    queries::vaults::get(&client, &id, &(current_user.user_id as i32)).await?;

    queries::secrets::insert(
        &client,
        &id,
        &new_secret.name,
        &new_secret.name_blind_index,
        &new_secret.secret,
    )
    .await?;

    Ok(Redirect::to(super::secret_route(id).parse()?))
}

markup::define! {
    NewSecretPage<'a>(user_vault: &'a queries::user_vaults::Get) {

        form.m_form[id="add-secret-form", method = "post",
            action=super::new_route(user_vault.vault_id)] {
            new_secret[label="Add Secret", class="add-secret"] {
                template[slot="body"] {
                    p {
                        "Folders keep related secrets together.
                        For example you could have a folder called Database with all
                        the secrets related to database access."
                    }

                    fieldset {
                        label[for="name"] { "Name" }
                        input[id="secret-name", type="text", required="", autocomplete="off", name="name"] {}

                        label[for="secret"] { "Secret" }
                        textarea[rows="10", id="secret-value", type="text", autocomplete="off", required="", name="secret"] {}

                    }

                    // Store the encrypted vault key here, then we can use it in the client to
                    // encrypt the secrets we create.
                    input[type="hidden",
                        id="encrypted-vault-key",
                        value=user_vault.encrypted_vault_key.clone()] {}
                    input[type="hidden",
                        id="user-vault-ecdh-public-key",
                        value=user_vault.ecdh_public_key.clone()] {}
                    input[type="hidden",
                        id="vault-id",
                        value=user_vault.vault_id] {}
                    input[type="hidden",
                        id="name-blind-index", name="name_blind_index"] {}
                }

                template[slot="footer"] {
                    button.a_button.auto.success[id="create-secret"] { "Create Secret" }
                }
            }
        }
    }
}
