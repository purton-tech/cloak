use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::response::{IntoResponse, Redirect};

pub async fn new(_authentication: Authentication) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}

markup::define! {
    ServiceAccountForm {

        form.m_form[id="add-secret-form", style="margin-top: 2em", method = "post",
            action=super::NEW] {
            sl_drawer[label="Add Service Accounts", class="add-secret"] {
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

                    label[for="kry"] { "ECDH Private Key" }
                    textarea[rows="8", required="", readonly="", name="encrypted_private_key", id="private-key"] {}
                    span.a_help_text { "The key for this service account" }

                }

                button.a_button.auto.success[slot="footer", id="create-secret"] { "Create Service Account" }
            }
        }

    }
}
