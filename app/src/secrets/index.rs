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
    let secrets = models::secret::Secret::get_all(&pool, &authentication, idor_vault_id).await?;

    let user_vault =
        models::user_vault::UserVault::get(&pool, &authentication, idor_vault_id).await?;

    if secrets.is_empty() {
        let empty_page = EmptySecretsPage {
            user_vault: &user_vault,
        };
        crate::layout::vault_layout(
            "Secrets",
            &empty_page.to_string(),
            "",
            &crate::layout::SideBar::Secrets,
            Some(idor_vault_id),
        )
    } else {
        let header = SecretsHeader {
            user_vault: &user_vault,
        };

        let page = SecretsPage {
            user_vault: &user_vault,
            secrets,
        };

        crate::layout::vault_layout(
            "Secrets",
            &page.to_string(),
            &header.to_string(),
            &crate::layout::SideBar::Secrets,
            Some(idor_vault_id),
        )
    }
}

markup::define! {
    SecretsHeader<'a>(user_vault: &'a models::user_vault::UserVault) {
        @super::new_secret::NewSecretPage { user_vault }
        button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
    }
    EmptySecretsPage<'a>(user_vault: &'a models::user_vault::UserVault) {
        .empty_page {
            div {
                h2 { "No Secrets Created"}
                h3 { "Create your first secret and add it too the vault"}
                @super::new_secret::NewSecretPage { user_vault }
                button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
            }
        }
    }
    SecretsPage<'a>(user_vault: &'a models::user_vault::UserVault, secrets: Vec<models::secret::Secret>) {
        div.m_card[id="secrets-table-controller"] {
            div.header {
                span { "Secrets" }
            }
            div.body {
                table.m_table.secrets_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Action" }
                        }
                    }
                    tbody {
                        @for secret in secrets {
                            tr {
                                td {
                                    ecdh_cipher[cipher=secret.name.clone(),
                                        "wrapped-aes-key"=user_vault.encrypted_vault_key.clone(),
                                        "ecdh-public-key"=user_vault.ecdh_public_key.clone()] {}
                                }
                                td {
                                    relative_time[datetime=secret.updated_at.to_rfc3339()] {}
                                }
                                td {
                                    relative_time[datetime=secret.created_at.to_rfc3339()] {}
                                }
                                td {
                                    a[id=format!("delete-secret-controller-{}", secret.id), href="#"] {
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
        @for secret in secrets {
            @super::delete::DeleteSecretForm {
                secret_id: secret.id as u32,
                vault_id: user_vault.vault_id as u32,
                secret_name: secret.name.clone(),
                user_vault
            }
        }
    }
}
