use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::statics;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path(idor_vault_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let org =
        queries::organisations::get_primary_organisation(&client, &(current_user.user_id as i32))
            .await?;

    // Blow up if the user doesn't have access to the vault
    queries::user_vaults::get(&client, &(current_user.user_id as i32), &idor_vault_id).await?;

    let members = queries::user_vaults::get_users_dangerous(&client, &idor_vault_id).await?;

    let non_members =
        queries::user_vaults::get_non_members_dangerous(&client, &idor_vault_id, &org.id).await?;

    let user_vault =
        queries::user_vaults::get(&client, &(current_user.user_id as i32), &idor_vault_id).await?;

    let environments =
        queries::environments::get_all(&client, &user_vault.vault_id, &(current_user.user_id as i32))
            .await?;

    let page = MembersPage {
        members: &members,
    };
    let header = MembersHeader {
        environments: &environments,
        non_members: &non_members,
        user_vault: &user_vault,
    };

    crate::layout::vault_layout(
        "Vault Members",
        &page.to_string(),
        &header.to_string(),
        &crate::layout::SideBar::Members,
        Some(idor_vault_id),
    )
}

markup::define! {
    MembersHeader<'a>(
        user_vault: &'a queries::user_vaults::Get,
        environments: &'a Vec<queries::environments::GetAll>,
        non_members: &'a Vec<queries::user_vaults::GetNonMembersDangerous>
    ) {
        @if non_members.len() != 0 {
            @super::add_member::AddMemberDrawer {
                user_vault: *user_vault,
                environments: *environments,
                non_members: *non_members
            }
            button.a_button.mini.primary[id="add-member"] { "Add Member" }
        }
    }
    MembersPage<'a>(
        members: &'a Vec<queries::user_vaults::GetUsersDangerous>)
    {
        div.m_card {
            div.header {
                span { "Members" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Environments" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for member in *members {
                            tr {
                                td {
                                    span[class="cipher"] {
                                        {member.email}
                                    }
                                }
                                td {
                                    @if let Some(envs) = &member.environments {
                                        {envs}
                                    }
                                }
                                td {
                                    a[id=format!("delete-member-{}", member.user_id), href="#"] {
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
        @for member in *members {
            @super::delete_member::DeleteMemberForm {
                user: member,
            }
        }
    }
}
