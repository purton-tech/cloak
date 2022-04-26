use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::statics;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;

pub async fn index(
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    let users =
        queries::organisations::get_users(&client, &(current_user.user_id as i32), &org.id).await?;

    let invites = queries::invitations::get_all(&client, &org.id).await?;

    let teams = queries::organisations::get_teams(&client, &(current_user.user_id as i32)).await?;

    let page = TeamPage {
        users,
        invites,
        teams,
    };

    crate::layout::layout("Team", &page.to_string(), &crate::layout::SideBar::Team)
}

markup::define! {
    TeamPage(
        users: Vec<queries::organisations::GetUsers>,
        invites: Vec<queries::invitations::GetAll>,
        teams: Vec<queries::organisations::GetTeams>) {

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
                table.m_table.team_table {
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
                table.m_table.memberships {
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
