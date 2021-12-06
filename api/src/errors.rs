use std::convert::From;
use tonic::{Code, Status};

/// Description of a Transaction, pending or in the chain.
#[derive(Debug)] // Allow the use of "{:?}" format specifier
pub enum CustomError {
    Database(String),
}

// Status is the error returned to GRPC.
// https://github.com/hyperium/tonic/blob/master/tonic/src/status.rs#L39
impl From<CustomError> for Status {
    fn from(error: CustomError) -> Status {
        match error {
            CustomError::Database(cause) => Status::new(Code::Internal, cause),
        }
    }
}

// If anything goes wrong with sqlx
impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
