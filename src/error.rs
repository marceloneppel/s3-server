//! Generic s3 error type.

use crate::path::ParseS3PathError;
use crate::utils::BoxStdError;

use std::convert::Infallible as Never;
use std::error::Error;
use std::fmt::{self, Display};

// TODO: switch to thiserror
// See https://github.com/dtolnay/thiserror/issues/79

/// Result carrying a generic `S3Error<E>`
pub type S3Result<T, E = Never> = Result<T, S3Error<E>>;

/// Generic s3 error type.
#[derive(Debug)]
pub enum S3Error<E = Never> {
    /// A operation-specific error occurred
    Operation(E),
    /// An error occurred when parsing and validating a request
    InvalidRequest(InvalidRequestError),
    /// An error occurred when converting storage output to a response
    InvalidOutput(InvalidOutputError),
    /// An error occurred when operating the storage
    Storage(BoxStdError),
    /// An error occurred when the operation is not supported
    NotSupported,
}

/// An error occurred when parsing and validating a request
#[derive(Debug, thiserror::Error)]
pub enum InvalidRequestError {
    /// ParsePath error
    #[error(transparent)]
    ParsePath(#[from] ParseS3PathError),
    // FIXME: add other errors
}

/// An error occurred when converting storage output to a response
#[derive(Debug, thiserror::Error)]
pub enum InvalidOutputError {
    /// InvalidHeaderName error
    #[error(transparent)]
    InvalidHeaderName(#[from] hyper::header::InvalidHeaderName),

    /// InvalidHeaderValue error
    #[error(transparent)]
    InvalidHeaderValue(#[from] hyper::header::InvalidHeaderValue),

    /// XmlWriter error
    #[error(transparent)]
    XmlWriter(#[from] xml::writer::Error),
}

impl<E: Display> Display for S3Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(e) => write!(f, "Operation: {}", e),
            Self::InvalidRequest(e) => write!(f, "Invalid request: {}", e),
            Self::InvalidOutput(e) => write!(f, "Invalid output: {}", e),
            Self::Storage(e) => write!(f, "Storage: {}", e),
            Self::NotSupported => write!(f, "Not supported"),
        }
    }
}

impl<E: Error + 'static> Error for S3Error<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Operation(e) => Some(e),
            Self::InvalidRequest(e) => Some(e),
            Self::InvalidOutput(e) => Some(e),
            Self::Storage(err) => Some(err.as_ref()),
            Self::NotSupported => None,
        }
    }
}
