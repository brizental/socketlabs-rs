use std::{fmt, result};

use failure::{Backtrace, Context, Fail};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Context::new(kind).into()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Error {
        if error.is_http() {
            return match error.url() {
                None => ErrorKind::UnexpectedError.into(),
                Some(_) => ErrorKind::RequestError(
                    "Problem making request to SocketLabs.".to_string(),
                ).into(),
            };
        }

        if error.is_serialization() {
            return ErrorKind::UnexpectedError.into();
        }

        if error.is_redirect() {
            return ErrorKind::TooManyRedirects.into();
        }

        ErrorKind::UnexpectedError.into()
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        ErrorKind::MessageParsingError(error.to_string()).into()
    }
}
