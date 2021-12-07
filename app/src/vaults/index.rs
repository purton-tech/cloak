use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Html<String>, CustomError> {
    let vaults = sqlx::query_as!(
        super::Vault,
        "
            SELECT name FROM vaults
        "
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| CustomError::Database(e.to_string()))?;

    let page = VaultsPage { vaults };

    crate::layout::layout("Home", &page.to_string())
}

markup::define! {
    VaultsPage(vaults: Vec<super::Vault>) {
        div.m_card {
            div.header {
                span { "Vaults" }

                sl_drawer[label="Add Vault", class="add-vault"] {
                    p {
                        "Folders keep related secrets together.
                        For example you could have a folder called Database with all
                        the secrets related to database access."
                    }

                    form.m_form[style="margin-top: 2em", method = "post", action=super::NEW] {
                        fieldset {
                            label[for="name"] { "Name" }
                            input[type="text", required="", name="name"] {}
                        }
                        button.a_button.auto.success[slot="footer", type = "submit"] { "Create Vault" }
                    }
                    button[class="a_button", slot="footer", type="primary"] { "Close" }
                }

                button.a_button.mini.primary."drawer-opener" { "Add Vault" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Items" }
                        }
                    }
                    tbody {
                        @for vault in vaults {
                            tr {
                                td { {vault.name} }
                                td { "Updated" }
                                td { "Created" }
                                td { "Items" }
                            }
                        }
                    }
                }
            }
        }
    }
}
