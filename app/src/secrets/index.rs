use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use crate::statics;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;
use time::format_description::well_known::Rfc3339;

pub async fn index(
    Path(idor_vault_id): Path<i32>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let secrets =
        queries::secrets::get_all(&client, &idor_vault_id, &(current_user.user_id as i32)).await?;

    let user_vault =
        queries::user_vaults::get(&client, &(current_user.user_id as i32), &idor_vault_id).await?;

    let environments =
        queries::environments::get_all(&client, &idor_vault_id, &(current_user.user_id as i32))
            .await?;

    if secrets.is_empty() {
        let empty_page = EmptySecretsPage {
            user_vault: &user_vault,
            environments,
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
            environments: &environments,
        };

        let page = SecretsPage {
            user_vault: &user_vault,
            secrets,
            environments: &environments,
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
    SecretsHeader<'a>(
        user_vault: &'a queries::user_vaults::Get,
        environments: &'a Vec<queries::environments::GetAll>
    ) {
        @super::new_secret::NewSecretPage {
            user_vault,
            environments: *environments
        }
        button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
    }

    EmptySecretsPage<'a>(
        user_vault: &'a queries::user_vaults::Get,
        environments: Vec<queries::environments::GetAll>
    ) {
        .empty_page {
            div {
                h2 { "No Secrets Created"}
                h3 { "Create your first secret and add it to the vault"}
                @super::new_secret::NewSecretPage {
                    user_vault,
                    environments: environments
                }
                button.a_button.mini.primary[id="new-secret"] { "Add Secret" }
            }
        }
    }

    SecretsPage<'a>(
        user_vault: &'a queries::user_vaults::Get, 
        secrets: Vec<queries::secrets::GetAll>,
        environments: &'a Vec<queries::environments::GetAll>
    ) {
        @for environment in *environments {
            div.m_card[id="secrets-table-controller"] {
                div.header {
                    span { {environment.name} }
                }
                div.body {
                    table.m_table.secrets_table {
                        thead {
                            tr {
                                th { "Environment" }
                                th { "Name" }
                                th { "Updated" }
                                th { "Created" }
                                th { "Action" }
                            }
                        }
                        tbody {
                            @for secret in secrets {
                                @if secret.environment_name == environment.name {
                                    tr {
                                        td {
                                            {secret.environment_name}
                                        }
                                        td {
                                            ecdh_cipher[cipher=secret.name.clone(),
                                                "wrapped-aes-key"=user_vault.encrypted_vault_key.clone(),
                                                "ecdh-public-key"=user_vault.ecdh_public_key.clone()] {}
                                        }
                                        td {
                                            relative_time[datetime=secret.updated_at.format(&Rfc3339).unwrap()] {}
                                        }
                                        td {
                                            relative_time[datetime=secret.created_at.format(&Rfc3339).unwrap()] {}
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
            }
        }

        // Generate all the details flyouts
        @for secret in secrets {
            @super::delete_secret::DeleteSecretForm {
                secret_id: secret.id as u32,
                vault_id: user_vault.vault_id as u32,
                secret_name: secret.name.clone(),
                user_vault
            }
        }
    }
}
