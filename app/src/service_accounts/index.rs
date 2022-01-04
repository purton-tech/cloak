use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    authentication: Authentication,
    Extension(pool): Extension<PgPool>,
) -> Result<Html<String>, CustomError> {
    let service_accounts = models::ServiceAccount::get_all(&pool, authentication.user_id).await?;

    let page = ServiceAccountsPage { service_accounts };

    crate::layout::layout(
        "Service Accounts",
        &page.to_string(),
        &crate::layout::SideBar::ServiceAccounts,
    )
}

markup::define! {
    ServiceAccountsPage(service_accounts: Vec<models::ServiceAccount>) {
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
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for service_account in service_accounts {
                            tr[id=format!("service-account-row-{}", service_account.id), style="cursor: pointer;"] {
                                td { {service_account.name} }
                                td { "Updated" }
                                td { "Created" }
                                td { a[href="#"] { "Attach to Vault" } }
                            }
                        }
                    }
                }
            }
        }
        // Generate all the details flyouts
        @for service_account in service_accounts {
            @super::view::ViewServiceAccount{ service_account }
        }

    }
}
