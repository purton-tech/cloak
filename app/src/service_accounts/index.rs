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
    let service_accounts =
        models::service_account::ServiceAccount::get_all(&pool, &authentication).await?;
    let vaults = models::vault::Vault::get_all(&pool, &authentication).await?;

    let page = ServiceAccountsPage {
        service_accounts,
        vaults,
    };

    crate::layout::layout(
        "Service Accounts",
        &page.to_string(),
        &crate::layout::SideBar::ServiceAccounts,
    )
}

markup::define! {
    ServiceAccountsPage(service_accounts: Vec<models::service_account::ServiceAccount>,
        vaults: Vec<models::vault::Vault>) {
        div.m_card {
            div.header {
                span { "Service Accounts" }

                @super::new_account::ServiceAccountForm {}

                button.a_button.mini.primary[id="new-account"] { "Add Service Account" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Service Account Name" }
                            th { "Vault" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for service_account in service_accounts {
                            tr {
                                @if let Some(vault_name) = service_account.vault_name.clone() {
                                    td[id=format!("service-account-view-{}", service_account.id)] {
                                        a[href="#"]
                                        { {service_account.name} }
                                    }
                                    td {
                                        {vault_name}
                                    }
                                } else {
                                    td {
                                        {service_account.name}
                                    }
                                    td[id=format!("service-account-row-{}", service_account.id)] {
                                        a[href="#"]
                                        { "Attach to Vault" }
                                    }
                                }
                                td {
                                    relative_time[datetime=service_account.updated_at.to_rfc3339()] {}
                                }
                                td {
                                    relative_time[datetime=service_account.created_at.to_rfc3339()] {}
                                }
                                td {
                                    a[id=format!("delete-account-controller-{}", service_account.id), href="#"] {
                                        img[src=statics::get_delete_svg(), width="18"] {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Generate all the details flyouts
        @for service_account in service_accounts {
            @if service_account.vault_id.is_none() {
                @super::connect_account::ConnectServiceAccountDrawer{ service_account, vaults }
            } else {
                @super::view::ViewServiceAccount{ service_account }
            }
            @super::delete::DeleteServiceAccountForm {
                service_account_id: service_account.id as u32,
                service_account_name: service_account.name.clone()
            }
        }
    }
}
