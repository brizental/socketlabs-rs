use std::{fmt, result};

use failure::{Backtrace, Context, Fail};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

pub type SocketLabsResult<T> = result::Result<T, SocketLabsError>;

#[derive(Debug)]
pub struct SocketLabsError {
    inner: Context<SocketLabsErrorKind>,
}

impl SocketLabsError {
    pub fn kind(&self) -> &SocketLabsErrorKind {
        self.inner.get_context()
    }
}

impl Fail for SocketLabsError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for SocketLabsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug, Fail)]
pub enum SocketLabsErrorKind {
    #[fail(display = "You must provide at least one message per request.")]
    MessageCountError,
    #[fail(display = "Error parsing message {}", _0)]
    MessageParsingError(String),
    #[fail(display = "{}", _0)]
    RequestError(String),
    #[fail(display = "Server redirecting too many times or making loop.")]
    TooManyRedirects,
    #[fail(
        display = "Unexpected error. Please file a bug at: https://github.com/brizental/socketlabs-rs/issues"
    )]
    UnexpectedError,
}

impl From<SocketLabsErrorKind> for SocketLabsError {
    fn from(kind: SocketLabsErrorKind) -> SocketLabsError {
        Context::new(kind).into()
    }
}

impl From<Context<SocketLabsErrorKind>> for SocketLabsError {
    fn from(inner: Context<SocketLabsErrorKind>) -> SocketLabsError {
        SocketLabsError { inner }
    }
}

impl From<ReqwestError> for SocketLabsError {
    fn from(error: ReqwestError) -> SocketLabsError {
        if error.is_http() {
            return match error.url() {
                None => SocketLabsErrorKind::UnexpectedError.into(),
                Some(_) => SocketLabsErrorKind::RequestError(
                    "Problem making request to SocketLabs.".to_string(),
                ).into(),
            };
        }

        if error.is_serialization() {
            return SocketLabsErrorKind::UnexpectedError.into();
        }

        if error.is_redirect() {
            return SocketLabsErrorKind::TooManyRedirects.into();
        }

        SocketLabsErrorKind::UnexpectedError.into()
    }
}

impl From<SerdeError> for SocketLabsError {
    fn from(error: SerdeError) -> SocketLabsError {
        SocketLabsErrorKind::MessageParsingError(error.to_string()).into()
    }
}
