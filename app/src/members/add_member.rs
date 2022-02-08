use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AddMember {
    pub user_id: u32,
    pub wrapped_vault_key: String,
    #[validate(length(min = 1, message = "The ecdh_public_key is mandatory"))]
    pub ecdh_public_key: String,
}

pub async fn add(
    Path(id): Path<u32>,
    authenticated_user: Authentication,
    Form(add_member): Form<AddMember>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, CustomError> {
    let user_vault = models::user_vault::UserVault {
        vault_id: id as i32,
        user_id: add_member.user_id as i32,
        encrypted_vault_key: add_member.wrapped_vault_key,
        ecdh_public_key: add_member.ecdh_public_key,
    };

    models::user_vault::UserVault::add_user_vault(&pool, &authenticated_user, &user_vault, id)
        .await?;

    Ok(Redirect::to(super::member_route(id).parse()?))
}

markup::define! {
    AddMemberDrawer<'a>(team: &'a Vec<models::organisation::User>,
        user_vault: &'a models::user_vault::UserVault) {

        form.m_form[id="add-team-member", method = "post", action=super::add_route(user_vault.vault_id)] {
            add_member[label="Add Member"] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }


                    select[id="user-selection", name="user_id"] {
                        @for user in *team {
                            option[value=format!("{}", user.id), "data-ecdh-pub-key"=user.ecdh_public_key.clone()] {
                                {user.email}
                            }
                        }
                    }

                    // Store the encrypted vault key here, then we can use it in the client to
                    // encrypt the secrets we create.
                    input[type="hidden",
                        id="encrypted-vault-key",
                        value=user_vault.encrypted_vault_key.clone()] {}
                    input[type="hidden",
                        id="user-vault-ecdh-public-key",
                        value=user_vault.ecdh_public_key.clone()] {}
                    input[type="hidden",
                        id="vault-id",
                        value=user_vault.vault_id] {}
                    input[type="hidden",
                        id="wrapped-vault-key", name="wrapped_vault_key"] {}
                    input[type="hidden",
                        id="ecdh-public-key", name="ecdh_public_key"] {}
                }

                template[slot="footer"] {
                    button.a_button.auto.success { "Update Members" }
                    button.a_button.auto.danger { "Cancel" }
                }
            }
        }
    }
}
