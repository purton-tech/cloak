use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;
use tonic::{Code, Status};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
    InvalidInput(String),
    //Unauthorized(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            //CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {}", cause)
            }
            CustomError::InvalidInput(ref cause) => write!(f, "Invalid Input: {}", cause),
        }
    }
}

// For gRPC we raise a custom error and it gets converted to a gRPC status code.
impl From<CustomError> for Status {
    fn from(error: CustomError) -> Status {
        match error {
            CustomError::Database(cause) => Status::new(Code::Internal, cause),
            CustomError::FaultySetup(cause) => Status::new(Code::Internal, cause),
            CustomError::InvalidInput(cause) => Status::new(Code::Internal, cause),
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (StatusCode::UNPROCESSABLE_ENTITY, self).into_response()
    }
}

// Any errors from sqlx get converted to CustomError
impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

// Any errors from sqlx get converted to CustomError
impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}
