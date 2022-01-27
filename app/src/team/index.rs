use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::organisation;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let org = organisation::Organisation::get_primary_org(&pool, &authentication).await?;

    let users = organisation::Organisation::get_users(&pool, &authentication, org.id).await?;

    let page = TeamPage {
        users,
        organisation_id: org.id,
    };

    crate::layout::layout("Team", &page.to_string(), &crate::layout::SideBar::Team)
}

markup::define! {
    TeamPage(users: Vec<organisation::User>, organisation_id: i32) {
        div.m_card {
            div.header {
                span { "Team" }

                @super::invite_user::InviteUserPage {
                    organisation_id: *organisation_id
                }

                button.a_button.mini.primary[id="invite-user"] { "New User" }
            }
            div.body {
                table.m_table.secrets_table {
                    thead {
                        tr {
                            th { "Email" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for user in users {
                            tr {
                                td {
                                    {user.email}
                                }
                                td {

                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
