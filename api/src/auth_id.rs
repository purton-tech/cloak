use crate::errors::CustomError;
use tonic::Request;

pub struct AuthId {
    pub user_id: u32,
}

impl AuthId {
    pub fn from_request<T>(req: &Request<T>) -> Result<AuthId, CustomError> {
        if let Some(user_id) = req.metadata().get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<u32>() {
                    return Ok(AuthId { user_id });
                }
            }
        }
        Err(CustomError::Unauthorised(
            "x-user-id not found or unparseable".to_string(),
        ))
    }
}
