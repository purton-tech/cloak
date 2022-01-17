use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::PgPool;

pub async fn index(
    Path(id): Path<u32>,
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let secrets = models::Secret::get_all(&pool, authentication.user_id, id).await?;

    let user_vault = models::UserVault::get(&pool, authentication.user_id, id).await?;

    let page = SecretsPage {
        user_vault: &user_vault,
        secrets,
    };

    crate::layout::layout("Home", &page.to_string(), &crate::layout::SideBar::Vaults)
}

markup::define! {
    SecretsPage<'a>(user_vault: &'a models::UserVault, secrets: Vec<models::Secret>) {
        div.m_card[id="secrets-table-controller"] {
            div.header {
                span { "Secrets" }

                @super::new_secret::NewSecretPage { user_vault }

                button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
            }
            div.body {
                table.m_table.secrets_table {
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
                                td {
                                    span[class="cipher"] {
                                        {secret.name}
                                    }
                                }
                                td { "Updated" }
                                td { "Created" }
                            }
                        }
                    }
                }
            }
            input[type="hidden", id="wrapped-vault-key", value={user_vault.encrypted_vault_key.clone()}] {}
        }
    }
}
