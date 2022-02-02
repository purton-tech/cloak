use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
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
    let org = models::organisation::Organisation::get_primary_org(&pool, &authentication).await?;

    let team =
        models::organisation::Organisation::get_users(&pool, &authentication, org.id).await?;

    let page = MembersDrawer {
        _vault_name: "vaults".to_string(),
        _team: team,
    };

    crate::layout::vault_layout(
        "Home",
        &page.to_string(),
        "",
        &crate::layout::SideBar::Vaults,
        Some(idor_vault_id),
    )
}

markup::define! {
    MembersDrawer(
        _vault_name: String,
        //members: &'a Vec<models::user_vault::UserDetails>,
        _team: Vec<models::organisation::User>)
    {

    }
}
