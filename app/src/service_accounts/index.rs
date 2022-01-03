use crate::errors::CustomError;
use axum::response::Html;

pub async fn index() -> Result<Html<String>, CustomError> {
    let page = ServiceAccountsPage {};

    crate::layout::layout(
        "Home",
        &page.to_string(),
        &crate::layout::SideBar::ServiceAccounts,
    )
}

markup::define! {
    ServiceAccountsPage {
        div.m_card {
            div.header {
                span { "Secrets" }


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
                    }
                }
            }
        }
    }
}
