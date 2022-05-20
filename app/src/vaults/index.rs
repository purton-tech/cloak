use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};
use deadpool_postgres::Pool;
use super::VaultSummary;

pub async fn index(
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let vaults = queries::vaults::get_all(&client, &(current_user.user_id as i32)).await?;
    
    if vaults.is_empty() {
        let mut buf = Vec::new();
        crate::templates::vaults::empty_html(&mut buf, "Your Vaults").unwrap();
        let html = format!("{}", String::from_utf8_lossy(&buf));
    
        Ok(Html(html))
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
    
        let mut buf = Vec::new();
        crate::templates::vaults::index_html(&mut buf, "Your Vaults", summary_vaults).unwrap();
        
        let html = format!("{}", String::from_utf8_lossy(&buf));
    
        Ok(Html(html))
    }
}
