use super::VaultSummary;
use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<&'static str>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation(&transaction, &organisation_id).await?;

    let vaults =
        queries::vaults::get_all(&transaction, &(current_user.user_id as i32), &organisation_id).await?;

    let user = queries::users::get_dangerous(&transaction, &(current_user.user_id as i32)).await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    if vaults.is_empty() {
        Ok(crate::render(|buf| {
            crate::templates::vaults::empty_html(buf, &initials, &team)
        }))
    } else {
        let mut summary_vaults: Vec<VaultSummary> = Default::default();

        for vault in vaults {
            let user_count = queries::vaults::user_vault_count(&transaction, &vault.id).await?;

            let secret_count = queries::vaults::secrets_count(&transaction, &vault.id).await?;

            summary_vaults.push(VaultSummary {
                user_count: user_count as i32,
                secrets_count: secret_count as i32,
                id: vault.id,
                name: vault.name,
                created_at: vault.created_at,
                updated_at: vault.updated_at,
            });
        }

        Ok(crate::render(|buf| {
            crate::templates::vaults::index_html(
                buf,
                &initials,
                summary_vaults,
                &team,
            )
        }))
    }
}
