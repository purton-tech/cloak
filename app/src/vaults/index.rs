use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use crate::statics;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    authentication: Authentication,
    Extension(pool): Extension<PgPool>,
) -> Result<Html<String>, CustomError> {
    let vaults = models::Vault::get_all(pool, authentication.user_id).await?;

    let page = VaultsPage { vaults };

    crate::layout::layout("Home", &page.to_string())
}

markup::define! {
    VaultsPage(vaults: Vec<models::Vault>) {
        div.m_card {
            div.header {
                span { "Vaults" }

                form.m_form[style="margin-top: 2em", method = "post", action=super::NEW] {
                    sl_drawer[label="Add Vault"] {
                        p {
                            "Vaults keep related secrets together.
                            For example you could have a vault called My Project with all
                            the secrets related to your project."
                        }

                        fieldset {
                            label[for="name"] { "Name *" }
                            input[type="text", required="", name="name"] {}
                            span.a_help_text { "Give your vault a name" }

                            label[for="kry"] { "Wrapped AES Key" }
                            textarea[rows="4", required="", readonly="", name="aes-key", id="new-vault-key"] {}
                            span.a_help_text { "The key for this vault" }
                        }

                        button.a_button.auto.success[slot="footer", type = "submit"] { "Create Vault" }
                    }
                }

                button.a_button.mini.primary[id="new-vault"] { "Add Vault" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Items" }
                            th { "More" }
                        }
                    }
                    tbody {
                        @for vault in vaults {
                            tr {
                                td { {vault.name} }
                                td { "Updated" }
                                td { "Created" }
                                td { "Items" }
                                td {
                                    a[href=crate::secrets::secret_route(vault.id)] {
                                        img[src=statics::get_more_info_svg(), style="width: 18px"] {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
