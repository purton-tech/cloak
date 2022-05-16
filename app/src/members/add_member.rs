use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::{IntoResponse, Redirect},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct AddMember {
    pub user_id: i32,
    pub wrapped_vault_key: String,
    #[validate(length(min = 1, message = "The ecdh_public_key is mandatory"))]
    pub ecdh_public_key: String,
    // Comma separated list of environemnt id's
    pub environments: String,
}

pub async fn add(
    Path(id): Path<i32>,
    current_user: Authentication,
    Form(add_member): Form<AddMember>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, CustomError> {
    let client = pool.get().await?;

    // The environments we have selected for the ser come in as a comma
    // separated list of ids.
    let envs: Vec<i32> = add_member
        .environments
        .split(",")
        .map(|e| e.parse::<i32>().unwrap_or(-1))
        .filter(|e| *e != -1)
        .collect();

    // Do an IDOR check, does this user have access to the vault. This will
    // blow up if we don't
    queries::vaults::get(&client, &id, &(current_user.user_id as i32)).await?;

    queries::user_vaults::insert(
        &client,
        &add_member.user_id,
        &id,
        &add_member.ecdh_public_key,
        &add_member.wrapped_vault_key,
    )
    .await?;

    for env in envs {
        queries::environments::connect_environment_to_user(&client, &add_member.user_id, &env)
            .await?;
    }

    Ok(Redirect::to(super::member_route(id).parse()?))
}

markup::define! {
    AddMemberDrawer<'a>(
        non_members: &'a Vec<queries::user_vaults::GetNonMembersDangerous>,
        environments: &'a Vec<queries::environments::GetAll>,
        user_vault: &'a queries::user_vaults::Get
    ) {

        form.m_form[id="add-team-member", method = "post", action=super::add_route(user_vault.vault_id)] {
            add_member[label="Add Member"] {
                template[slot="body"] {

                    fieldset {
                        label[for="name"] { "Which team member do you want ot give access to?" }
                        select[id="user-selection", name="user_id"] {
                            @for user in *non_members {
                                option[value=format!("{}", user.id), "data-ecdh-pub-key"=user.ecdh_public_key.clone()] {
                                    {user.email}
                                }
                            }
                        }
                        span.a_help_text {
                            "Select a user"
                        }

                        label[for="name"] { "Which environments do you want the user to have access to?" }

                        @for env in *environments {
                            label[] {
                                input[type="checkbox", name="env", value=env.id] {  }
                                {env.name}
                            }
                        }
                        span.a_help_text {
                            "Select at least one environment"
                        }
                    }

                    // We convert the checkboxes into a comma separated lsit of environment id's
                    input[type="hidden", name="environments", id="environments"] {}

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
