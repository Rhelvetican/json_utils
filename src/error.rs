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
    Custom(&'static str),
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

    pub(crate) fn custom(msg: &'static str) -> Self {
        Self::Custom(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Json(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}
