use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    ServerCommunications(String),
    Unauthorized(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::ServerCommunications(ref cause) => {
                write!(f, "Communications Error: {}", cause)
            }
        }
    }
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::FaultySetup(err) => HttpResponse::InternalServerError().body(err),
            CustomError::Unauthorized(err) => HttpResponse::InternalServerError().body(err),
            CustomError::ServerCommunications(err) => HttpResponse::InternalServerError().body(err),
        }
    }
}

// rust bitcoin Allow this type to be treated like an error
impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<tonic::transport::Error> for CustomError {
    fn from(err: tonic::transport::Error) -> CustomError {
        CustomError::ServerCommunications(err.to_string())
    }
}

impl From<tonic::metadata::errors::InvalidMetadataValue> for CustomError {
    fn from(err: tonic::metadata::errors::InvalidMetadataValue) -> CustomError {
        CustomError::ServerCommunications(err.to_string())
    }
}

impl From<tonic::Status> for CustomError {
    fn from(err: tonic::Status) -> CustomError {
        CustomError::ServerCommunications(err.to_string())
    }
}

// Age using a buffered writer
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<horrorshow::Error> for CustomError {
    fn from(err: horrorshow::Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<std::str::Utf8Error> for CustomError {
    fn from(err: std::str::Utf8Error) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(err: std::num::ParseIntError) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for CustomError {
    fn from(err: std::num::ParseFloatError) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}
