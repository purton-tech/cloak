use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::{invitation, organisation};
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let org = organisation::Organisation::get_primary_org(&pool, &authentication).await?;

    let users = organisation::Organisation::get_users(&pool, &authentication, org.id).await?;

    let invites = invitation::Invitation::get_all(&pool, &authentication).await?;

    let page = TeamPage { users, invites };

    crate::layout::layout("Team", &page.to_string(), &crate::layout::SideBar::Team)
}

markup::define! {
    TeamPage(
        users: Vec<organisation::User>,
        invites: Vec<invitation::Invitation>) {

        div.m_card {
            div.header {
                span { "Team" }

                @super::create_invite::InviteUserPage {}

                button.a_button.mini.primary[id="invite-user"] { "New User" }
            }
            div.body {
                table.m_table.secrets_table {
                    thead {
                        tr {
                            th { "Email" }
                            th { "Status" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for user in users {
                            tr {
                                td {
                                    {user.email}
                                }
                                @if user.is_admin {
                                    td {
                                        "Administrator"
                                    }
                                } else {
                                    td {
                                        "Invitation Accepted"
                                    }
                                }
                                td {

                                }
                            }
                        }
                        @for invite in invites {
                            tr {
                                td {
                                    {invite.email}
                                }
                                td {
                                    "Invitation Pending"
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
