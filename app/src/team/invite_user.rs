use crate::errors::CustomError;
use axum::{
    body::Body,
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use hyper::Request;
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
    request: Request<Body>,
    //Path(signature): Path<String>,
    //Extension(pool): Extension<PgPool>,
    //authentication: Authentication,
) -> Result<impl IntoResponse, CustomError> {
    let request_uri = (
        request.uri().host(),
        request.uri().scheme(),
        request.uri().port(),
    );

    match request_uri {
        (Some(host), Some(scheme), Some(port)) => {
            // Reconstruct URL and check signature
            let url = format!(
                "{scheme}//{host}:{port}/app/team/invite/{org}?email={email}&time={date}",
                email = &params.email,
                date = params.time
            );

            dbg!(url);
        }
        _ => {
            dbg!("Unable to destruct");
        }
    };

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
