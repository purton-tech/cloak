use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteVault {
    pub vault_id: i32,
    pub name: String,
}

pub async fn delete(
    current_user: Authentication,
    Form(idor_delete_vault): Form<DeleteVault>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {

    let client = pool.get().await?;

    let vault = queries::vaults::get(
        &client,
        &(current_user.user_id as i32),
        &idor_delete_vault.vault_id,
    )
    .await?;

    if vault.name == idor_delete_vault.name {
        queries::vaults::delete(
            &client,
            &idor_delete_vault.vault_id,
            &(current_user.user_id as i32),
        )
        .await?;
    } else {
        return crate::layout::redirect_and_snackbar(super::INDEX, "Name did not match");
    }

    crate::layout::redirect_and_snackbar(super::INDEX, "Vault Deleted")
}

markup::define! {
    DeleteVaultForm(vault_id: u32, vault_name: String) {

        form.m_form[method="post", action=super::DELETE] {
            side_drawer[label="Delete Vault ?",
                id=format!("delete-vault-drawer-{}", vault_id)] {

                template[slot="body"] {

                    p {
                        "Are you sure you want to delete this vault?"
                    }

                    p {
                        "If so then type the name of the vault "
                        strong {
                            {format!("\"{}\"", vault_name)}
                        }
                        " into the input field"
                    }

                    fieldset {
                        label[for="name"] { "Name *" }
                        input[type="text", required="", name="name", autocomplete="off"] {}
                        span.a_help_text { "Please confirm the name of the vault you wish to delete" }
                    }

                    input[type="hidden", name="vault_id", value=vault_id.to_string()] {}
                }

                template[slot="footer"] {
                    button.a_button.auto.danger[slot="footer", type = "submit"] { "Delete Vault" }
                }
            }
        }

    }
}
