use crate::errors::CustomError;
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    email: String,
    time: u64,
    sig: String,
}

pub async fn invite(
    Path(org): Path<i32>,
    Query(params): Query<Params>,
    //Path(signature): Path<String>,
    //Extension(pool): Extension<PgPool>,
    //authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    dbg!(org);
    dbg!(params.email);
    dbg!(params.sig);
    // Reconstruct URL and check signature

    Ok(Redirect::to("/app/vaults".parse()?))
}

markup::define! {
    InviteUserPage(organisation_id: i32) {

        form.m_form {
            invite_user[label="Invite User",
                organisation=format!("{}", organisation_id)] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }

                    fieldset {
                        label[for="email"] { "Email" }
                        input[type="email", required="", name="name"] {}

                        label[for="invite"] { "Invite" }
                        textarea[name="invite", rows="7", readonly="readonly"] {

                        }

                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success { "Create Invitation" }
                    button.a_button.auto.danger { "Cancel" }
                }
            }
        }
    }
}
