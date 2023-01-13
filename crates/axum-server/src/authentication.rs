use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct Authentication {
    pub user_id: i32,
}

// From a request extract our authentication token.
#[async_trait]
impl<B> FromRequest<B> for Authentication
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(user_id) = req.headers().get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<i32>() {
                    return Ok(Authentication { user_id });
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "x-user-id not found or unparseable as i32",
        )
            .into_response())
    }
}
