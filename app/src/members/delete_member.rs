use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form, Path},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DeleteMember {
    pub user_id: u32,
    pub vault_id: u32,
}

pub async fn delete(
    Path(vault_id): Path<u32>,
    authentication: Authentication,
    Form(delete_member): Form<DeleteMember>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    models::user_vault::UserVault::remove_user_from_vault(
        &pool,
        &authentication,
        delete_member.user_id,
        delete_member.vault_id,
    )
    .await?;

    // If we remove ourself, redirect to vaults page.
    let url = if delete_member.user_id == authentication.user_id {
        crate::vaults::INDEX.to_string()
    } else {
        super::member_route(vault_id)
    };

    crate::layout::redirect_and_snackbar(&url, "Member Removed From Vault")
}

markup::define! {
    DeleteMemberForm<'a>(
        user: &'a models::user_vault::UserDetails) {

        form.m_form[method="post", action=super::delete_route(user.vault_id as u32)] {
            side_drawer[label="Remove Member from Vault?",
                id=format!("delete-member-drawer-{}", user.user_id)] {

                template[slot="body"] {
                    p {
                        {format!("Are you sure you want to remove {}", user.email)}
                    }
                    input[type="hidden", name="user_id", value=user.user_id.to_string()] {}
                    input[type="hidden", name="vault_id", value=user.vault_id.to_string()] {}
                }
                template[slot="footer"] {
                    button.a_button.auto.danger[type = "submit"] { "Remove Member" }
                }
            }
        }

    }
}
