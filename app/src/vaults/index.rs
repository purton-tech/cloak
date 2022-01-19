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
    let vaults = models::Vault::get_all(&pool, authentication.user_id).await?;

    let page = VaultsPage { vaults };

    crate::layout::layout("Home", &page.to_string(), &crate::layout::SideBar::Vaults)
}

markup::define! {
    VaultsPage(vaults: Vec<models::Vault>) {
        div.m_card {
            div.header {
                span { "Vaults" }

                @super::new_vault::VaultForm {}

                button.a_button.mini.primary[id="new-vault"] { "Add Vault" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "More" }
                        }
                    }
                    tbody {
                        @for vault in vaults {
                            tr {
                                td {
                                    a[href=crate::secrets::secret_route(vault.id)] {
                                        {vault.name}
                                    }
                                }
                                td {
                                    relative_time[datetime=vault.updated_at.to_rfc3339()] {}
                                }
                                td {
                                    relative_time[datetime=vault.created_at.to_rfc3339()] {}
                                }
                                td {
                                    a[href=crate::secrets::secret_route(vault.id)] {
                                        img[src=statics::get_more_info_svg(), width="18"] {}
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
