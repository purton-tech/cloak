use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types::public::{AuditAccessType, AuditAction};
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use deadpool_postgres::Pool;

pub async fn index(
    Path((team_id, vault_id)): Path<(i32, i32)>,
    Extension(pool): Extension<Pool>,
    current_user: Authentication,
) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let team = queries::organisations::organisation(&client, &team_id).await?;

    let secrets =
        queries::secrets::get_all(&client, &vault_id, &(current_user.user_id as i32)).await?;

    let user_vault =
        queries::user_vaults::get(&client, &(current_user.user_id as i32), &vault_id).await?;

    let environments =
        queries::environments::get_all(&client, &vault_id, &(current_user.user_id as i32))
            .await?;

    if secrets.is_empty() {
        let mut buf = Vec::new();
        crate::templates::secrets::empty_html(
            &mut buf,
            "Your Secrets",
            &user_vault,
            environments,
            &team,
        )
        .unwrap();
        let html = format!("{}", String::from_utf8_lossy(&buf));

        Ok(Html(html))
    } else {
        let mut buf = Vec::new();
        crate::templates::secrets::index_html(
            &mut buf,
            "Your Secrets",
            &user_vault,
            environments,
            secrets,
            &team,
        )
        .unwrap();
        let html = format!("{}", String::from_utf8_lossy(&buf));

        queries::audit::insert(
            &client,
            &(current_user.user_id as i32),
            &team.id,
            &AuditAction::AccessSecrets,
            &AuditAccessType::Web,
            &format!("Secrets  accesed from vault {}", &user_vault.vault_id),
        )
        .await?;

        Ok(Html(html))
    }
}
