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
) -> Result<Html<&'static str>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let team = queries::organisations::organisation()
        .bind(&transaction, &team_id)
        .one()
        .await?;

    let secrets = queries::secrets::get_all()
        .bind(&transaction, &vault_id)
        .all()
        .await?;

    let user_vault = queries::user_vaults::get()
        .bind(&transaction, &current_user.user_id, &vault_id)
        .one()
        .await?;

    let environments = queries::environments::get_all()
        .bind(&transaction, &vault_id)
        .all()
        .await?;

    let user = queries::users::get()
        .bind(&transaction, &current_user.user_id)
        .one()
        .await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    if secrets.is_empty() {
        Ok(crate::render(|buf| {
            crate::ructe::templates::secrets::empty_html(
                buf,
                &initials,
                &user_vault,
                environments,
                &team,
            )
        }))
    } else {
        queries::audit::insert()
            .bind(
                &transaction,
                &current_user.user_id,
                &team.id,
                &AuditAction::AccessSecrets,
                &AuditAccessType::Web,
                &format!("Secrets  accesed from vault {}", &user_vault.vault_id).as_ref(),
            )
            .await?;

        Ok(crate::render(|buf| {
            crate::ructe::templates::secrets::index_html(
                buf,
                &initials,
                &user_vault,
                environments,
                secrets,
                &team,
            )
        }))
    }
}
