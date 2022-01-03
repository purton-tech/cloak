use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::PgPool;

pub async fn index(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let secrets = models::Secret::get_all(&pool, authentication.user_id, id).await?;

    let user_vault = models::UserVault::get(&pool, authentication.user_id as i32, id).await?;

    let page = SecretsPage {
        user_vault: &user_vault,
        secrets,
    };

    crate::layout::layout("Home", &page.to_string(), &crate::layout::SideBar::Vaults)
}

markup::define! {
    SecretsPage<'a>(user_vault: &'a models::UserVault, secrets: Vec<models::Secret>) {
        div.m_card {
            div.header {
                span { "Secrets" }

                form.m_form[id="add-secret-form", style="margin-top: 2em", method = "post",
                    action=super::new_route(user_vault.vault_id)] {
                    sl_drawer[label="Add Secret", class="add-secret"] {
                        p {
                            "Folders keep related secrets together.
                            For example you could have a folder called Database with all
                            the secrets related to database access."
                        }

                        fieldset {
                            label[for="name"] { "Name" }
                            input[id="secret-name", type="text", required="", name="name"] {}

                            label[for="secret"] { "Secret" }
                            input[id="secret-value", type="text", required="", name="secret"] {}

                        }

                        // Store the encrypted vault key here, then we can use it in the client to
                        // encrypt the secrets we create.
                        input[type="hidden",
                            id="vault-key",
                            value=user_vault.encrypted_vault_key.clone()] {}

                        button.a_button.auto.success[slot="footer", id="create-secret"] { "Create Secret" }
                    }
                }

                button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                        }
                    }
                    tbody {
                        @for secret in secrets {
                            tr {
                                td { {secret.name} }
                                td { "Updated" }
                                td { "Created" }
                            }
                        }
                    }
                }
            }
        }
    }
}
