use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::statics;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;
use time::format_description::well_known::Rfc3339;

pub async fn index(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let service_accounts =
        queries::service_accounts::get_all(&client, &(current_user.user_id as i32)).await?;

    //let vaults = queries::vaults::get_all(&client, &(current_user.user_id as i32)).await?;
    let environments_and_vaults =
        queries::environments::get_environments_and_vaults(&client, &(current_user.user_id as i32)).await?;

    if service_accounts.is_empty() {
        let empty_page = EmptyServiceAccounts {};
        crate::layout::layout(
            "Service Accounts",
            &empty_page.to_string(),
            &crate::layout::SideBar::ServiceAccounts,
        )
    } else {
        let header = ServiceAccountsHeader {};

        let page = ServiceAccountsPage {
            service_accounts,
            environments_and_vaults,
        };

        crate::layout::layout_with_header(
            "Service Accounts",
            &page.to_string(),
            &header.to_string(),
            &crate::layout::SideBar::ServiceAccounts,
        )
    }
}

markup::define! {
    ServiceAccountsHeader {
        @super::new_account::ServiceAccountForm {}

        button.a_button.mini.primary[id="new-account"] { "Add Service Account" }
    }
    EmptyServiceAccounts {
        .empty_page {
            div {
                h2 { "No Service Accounts Created"}
                h3 { "Service accounts give you access to secrets held in vaults" }
                @super::new_account::ServiceAccountForm {}

                button.a_button.mini.primary[id="new-account"] { "Add Service Account" }
            }
        }
    }
    ServiceAccountsPage(
        service_accounts: Vec<queries::service_accounts::GetAll>,
        environments_and_vaults: Vec<queries::environments::GetEnvironmentsAndVaults>
    ) {
        div.m_card {
            div.header {
                span { "Service Accounts" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Service Account Name" }
                            th { "Vault" }
                            th { "Environment" }
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
                                        { {service_account.account_name} }
                                    }
                                    td {
                                        {vault_name}
                                    }
                                } else {
                                    td {
                                        {service_account.account_name}
                                    }
                                    td[id=format!("service-account-row-{}", service_account.id)] {
                                        a[href="#"]
                                        { "Connect to Vault" }
                                    }
                                }
                                @if let Some(env_name) = service_account.environment_name.clone() {
                                    td {                                        
                                        {env_name}
                                    }
                                } else {
                                    td {}
                                }
                                td {
                                    relative_time[datetime=service_account.updated_at.format(&Rfc3339).unwrap()] {}
                                }
                                td {
                                    relative_time[datetime=service_account.created_at.format(&Rfc3339).unwrap()] {}
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
                @super::connect_account::ConnectServiceAccountDrawer{ 
                    service_account, environments_and_vaults 
                }
            } else {
                @super::view::ViewServiceAccount{ service_account }
            }
            @super::delete::DeleteServiceAccountForm {
                service_account_id: service_account.id as u32,
                service_account_name: service_account.account_name.clone()
            }
        }
    }
}
