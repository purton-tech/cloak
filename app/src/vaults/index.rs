use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{extract::Extension, response::Html};
use sqlx::PgPool;

pub async fn index(
    authentication: Authentication,
    Extension(pool): Extension<PgPool>,
) -> Result<Html<String>, CustomError> {
    let vaults = models::vault::Vault::get_all(&pool, &authentication).await?;

    let page = VaultsPage { vaults };

    let header = VaultHeader {};

    crate::layout::layout_with_header(
        "Vaults",
        &page.to_string(),
        &header.to_string(),
        &crate::layout::SideBar::Vaults,
    )
}

markup::define! {
    VaultHeader {
        @super::new_vault::VaultForm {}
        button.a_button.mini.primary[id="new-vault"] { "Create A New Vault" }
    }
    VaultsPage(
        vaults: Vec<models::vault::VaultSummary>) {

        @for vault in vaults {
            .m_card."vault-card".clickable[href=crate::secrets::secret_route(vault.id)] {
                .body."m-vault-card-body" {
                    div {
                        h4.title { {vault.name} }
                        .created {
                            "Created "
                            relative_time[datetime=vault.created_at.to_rfc3339()] {}
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
