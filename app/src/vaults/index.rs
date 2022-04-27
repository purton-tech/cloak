use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub struct VaultSummary {
    pub id: i32,
    pub name: String,
    pub user_count: i32,
    pub secrets_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn index(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let vaults = queries::vaults::get_all(&client, &(current_user.user_id as i32)).await?;

    if vaults.is_empty() {
        let empty_page = EmptyVaultPage {};
        crate::layout::layout_with_header(
            "Vaults",
            &empty_page.to_string(),
            "",
            &crate::layout::SideBar::Vaults,
        )
    } else {
        let mut summary_vaults: Vec<VaultSummary> = Default::default();

        for vault in vaults {
            let user_count = queries::vaults::user_vault_count(&client, &vault.id).await?;

            let secret_count = queries::vaults::secrets_count(&client, &vault.id).await?;

            summary_vaults.push(VaultSummary {
                user_count: user_count as i32,
                secrets_count: secret_count as i32,
                id: vault.id,
                name: vault.name,
                created_at: vault.created_at,
                updated_at: vault.updated_at,
            });
        }

        let header = VaultHeader {};
        let page = VaultsPage {
            vaults: summary_vaults,
        };
        crate::layout::layout_with_header(
            "Vaults",
            &page.to_string(),
            &header.to_string(),
            &crate::layout::SideBar::Vaults,
        )
    }
}

markup::define! {
    VaultHeader {
        @super::new_vault::VaultForm {}
        button.a_button.mini.primary[id="new-vault"] { "Create A New Vault" }
    }
    EmptyVaultPage {
        .empty_page {
            div {
                h2 { "No Vaults Created"}
                h3 { "Create your first vault to get started with Cloak"}
                @super::new_vault::VaultForm {}
                button.a_button.mini.primary[id="new-vault"] { "Create A New Vault" }
            }
        }
    }
    VaultsPage(
        vaults: Vec<VaultSummary>) {

        @for vault in vaults {
            .m_card."vault-card".clickable[href=crate::secrets::secret_route(vault.id)] {
                .body."m-vault-card-body" {
                    div {
                        h4.title { {vault.name} }
                        .created {
                            "Created "
                            relative_time[datetime=vault.created_at.format(&Rfc3339).unwrap()] {}
                        }
                    }
                    div {
                        h4.title { "Team Members" }
                        p {
                            {format!("{}", vault.user_count)}
                        }
                    }
                    div {
                        h4.title { "Secrets" }
                        p {
                            {format!("{}", vault.secrets_count)}
                        }
                    }
                    div.settings {
                        button.a_button.ghost.danger[id=format!("delete-vault-{}", vault.id), href="#"] {
                            { "Delete "}
                        }
                    }
                }
            }
        }

        // Generate all the delete vault flyouts
        @for vault in vaults {
            @super::delete_vault::DeleteVaultForm {
                vault_id: vault.id as u32,
                vault_name: vault.name.clone()
            }
        }
    }
}
