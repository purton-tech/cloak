use actix_web::{dev::Payload, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::errors::CustomError;

use futures::future::{err, ok, Ready};

#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    pub user_id: u32,
}

impl FromRequest for Authentication {
    type Error = CustomError;
    type Future = Ready<Result<Authentication, CustomError>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        if let Some(user_id) = req.headers().get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<u32>() {
                    return ok(Authentication { user_id });
                }
            }
        }
        err(CustomError::Unauthorized(
            "x-user-id not found or unparseable".to_string(),
        ))
    }
}
