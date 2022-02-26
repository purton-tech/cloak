use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use crate::statics;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::PgPool;

pub async fn index(
    Path(idor_vault_id): Path<u32>,
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let org = models::organisation::Organisation::get_primary_org(&pool, &authentication).await?;

    let vault = models::user_vault::UserVault::get(&pool, &authentication, idor_vault_id).await?;

    let members =
        models::user_vault::UserVault::get_users_dangerous(&pool, vault.vault_id as u32).await?;

    let team =
        models::organisation::Organisation::get_users(&pool, &authentication, org.id).await?;

    let user_vault =
        models::user_vault::UserVault::get(&pool, &authentication, idor_vault_id).await?;

    let page = MembersPage {
        _vault_name: "vaults".to_string(),
        members: &members,
    };
    let header = MembersHeader {
        _vault_name: "vaults".to_string(),
        team: &team,
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
        _vault_name: String,
        user_vault: &'a models::user_vault::UserVault,
        team: &'a Vec<models::organisation::User>)
    {
        @super::add_member::AddMemberDrawer {
            user_vault: *user_vault,
            team: *team
        }
        button.a_button.mini.primary[id="add-member"] { "Add Member" }
    }
    MembersPage<'a>(
        _vault_name: String,
        members: &'a Vec<models::user_vault::UserDetails>)
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
                                    a[href="#"] {
                                        img[src=statics::get_delete_svg(), width="18"] {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
