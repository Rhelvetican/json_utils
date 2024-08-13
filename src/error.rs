use serde_json::Error as JsonError;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    Io(IoError),
    Utf8(FromUtf8Error),
    Other(&'static str),
}

impl Error {
    pub(crate) fn json(err: JsonError) -> Self {
        Self::Json(err)
    }

    pub(crate) fn io(err: IoError) -> Self {
        Self::Io(err)
    }

    pub(crate) fn utf8(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }

    pub(crate) fn other(msg: &'static str) -> Self {
        Self::Other(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Json(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(feature = "anyhow")]
impl From<Error> for anyhow::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Json(err) => anyhow::Error::new(err),
            Error::Io(err) => anyhow::Error::new(err),
            Error::Utf8(err) => anyhow::Error::new(err),
            Error::Other(msg) => anyhow::Error::msg(msg),
        }
    }
}
