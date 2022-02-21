use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::{organisation, user};
use axum::{
    body::Body,
    extract::{Extension, Path, Query},
    response::{IntoResponse, Redirect},
};
use hyper::Request;
use p256::pkcs8::DecodePublicKey;
use p256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    PublicKey,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct Params {
    id: u32,
    email: String,
    time: u64,
    sig: String,
}

pub async fn invite(
    Path(org): Path<u32>,
    Query(params): Query<Params>,
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
    request: Request<Body>, // Request<Body> Has to be the last extractor
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
                "{scheme}://{host}:{port}/app/team/invite/{org}?id={id}&email={email}&time={date}",
                id = &params.id,
                email = &params.email,
                date = params.time
            );

            // We need to know, does the user who created the link have authorizatiion
            // to invite users?
            if let Ok(org_user) =
                organisation::Organisation::get_dangerous(&pool, params.id, org).await
            {
                // Is the user doing the inviations an ad in for the org they want to invite
                // the user to?
                if org_user.is_admin {
                    let user = user::User::get_dangerous(&pool, params.id).await?;

                    // Now we can do the public key check
                    let user_key_der = base64::decode(user.ecdsa_public_key)
                        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
                    let user_public_key = PublicKey::from_public_key_der(&user_key_der)
                        .map_err(|e| CustomError::FaultySetup(e.to_string()))?;

                    let sig_der = base64::decode(params.sig)
                        .map_err(|e| CustomError::InvalidInput(e.to_string()))?;

                    let verify_key = VerifyingKey::from(&user_public_key);
                    let signature = Signature::from_der(&sig_der)
                        .map_err(|e| CustomError::InvalidInput(e.to_string()))?;

                    if verify_key.verify(url.as_bytes(), &signature).is_ok() {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        if since_the_epoch.as_millis() < (params.time + (24 * 60 * 60000)).into() {
                            // All details are correct add the user to the team.
                            organisation::Organisation::add_user_dangerous(
                                &pool,
                                &authentication,
                                org,
                            )
                            .await?;
                        }
                    }
                }
            }
        }
        _ => {
            dbg!("Unable to destruct");
        }
    };

    Ok(Redirect::to("/app/vaults".parse()?))
}

markup::define! {
    InviteUserPage(organisation_id: i32, user_id: u32) {

        form.m_form {
            invite_user[label="Invite User",
                user=format!("{}", user_id),
                organisation=format!("{}", organisation_id)] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }

                    fieldset {
                        label[for="email"] { "Email" }
                        input[type="email", required="", name="name"] {}

                        label[for="invite"] { "Invite" }
                        p[id="invite"] {

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
