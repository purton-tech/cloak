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
        button.a_button.mini.primary[id="new-vault"] { "Add Vault" }
    }
    VaultsPage(vaults: Vec<models::vault::Vault>) {

        @for vault in vaults {
            .m_card."vault-card".clickable[href=crate::secrets::secret_route(vault.id)] {
                .body {
                    h4.title { {vault.name} }
                    .created {
                        "Created "
                        relative_time[datetime=vault.created_at.to_rfc3339()] {}
                    }
                    @super::members::MembersDrawer {
                        vault_name: vault.name.clone()
                    }
                    button."open-members-drawer" {
                        {"Members"}
                    }
                }
            }
        }
    }
}
