use crate::errors::CustomError;
use axum::response::Html;

pub async fn index() -> Result<Html<String>, CustomError> {
    let page = TeamPage {};

    crate::layout::layout("Home", &page.to_string(), &crate::layout::SideBar::Team)
}

markup::define! {
    TeamPage {
        div.m_card {
            div.header {
                span { "Team" }

                //@super::new_account::ServiceAccountForm {}

                button.a_button.mini.primary[id="new-account"] { "New User" }
            }
            div.body {
            }
        }
    }
}
