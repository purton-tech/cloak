use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::{invitation, organisation};
use crate::statics;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    Extension(pool): Extension<PgPool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let org = organisation::Organisation::get_primary_org(&pool, &current_user).await?;

    let users = organisation::Organisation::get_users(&pool, &current_user, org.id).await?;

    let invites = invitation::Invitation::get_all(&pool, &current_user).await?;

    let teams = organisation::Organisation::get_teams(&pool, &current_user).await?;

    let page = TeamPage {
        users,
        invites,
        teams,
    };

    crate::layout::layout("Team", &page.to_string(), &crate::layout::SideBar::Team)
}

markup::define! {
    TeamPage(
        users: Vec<organisation::User>,
        invites: Vec<invitation::Invitation>,
        teams: Vec<organisation::Team>) {

        @for user in users {
            @super::delete_member::DeleteMemberForm {
                organisation_id: user.organisation_id as u32,
                user_id: user.id as u32,
                email: user.email.clone()
            }
        }

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
                                    a[
                                        href="#",
                                        "data-drawer-target"=format!(
                                            "delete-member-drawer-{}-{}",
                                            user.organisation_id,
                                            user.id
                                        )
                                    ] {
                                        img[src=statics::get_delete_svg(), width="18"] {}
                                    }
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

        div.m_card {
            div.header {
                span { "Team you are a member of" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Team Name" }
                            th { "Team Owner" }
                        }
                    }
                    tbody {
                        @for team in teams {
                            tr {
                                td {
                                    {team.organisation_name}
                                }
                                td {
                                    {team.team_owner}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
