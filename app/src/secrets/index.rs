use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
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
        let mut buf = Vec::new();
        crate::templates::secrets::empty_html(&mut buf, "Your Secrets", user_vault, environments).unwrap();
        let html = format!("{}", String::from_utf8_lossy(&buf));
    
        Ok(Html(html))
    } else {
        let mut buf = Vec::new();
        crate::templates::secrets::index_html(&mut buf, "Your Secrets", user_vault, environments, secrets).unwrap();
        let html = format!("{}", String::from_utf8_lossy(&buf));
    
        Ok(Html(html))
    }
}
