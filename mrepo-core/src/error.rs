use std::{fmt, io, result};
use std::fmt::{Debug, Display};

pub struct Error {
    err: Box<ErrorImpl>,
}

pub type Result<T> = result::Result<T, Error>;

struct ErrorImpl {
    code: ErrorCode,
}

pub(crate) enum ErrorCode {
    Message(Box<str>),
    Io(io::Error),
    Json(serde_json::Error),
}

impl Error {
    #[cold]
    pub(crate) fn new(code: ErrorCode) -> Self {
        Error {
            err: Box::new(ErrorImpl { code }),
        }
    }

    #[cold]
    pub(crate) fn io(error: io::Error) -> Self {
        Self::new(ErrorCode::Io(error))
    }

    #[cold]
    pub(crate) fn json(error: serde_json::Error) -> Self {
        Self::new(ErrorCode::Json(error))
    }

    #[cold]
    pub fn custom<T: Display>(msg: T) -> Self {
        let msg = msg.to_string();

        Self::new(ErrorCode::Message(msg.into_boxed_str()))
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(error) => Display::fmt(error, f),
            ErrorCode::Json(error) => Display::fmt(error, f),
        }
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.code, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error({:?})", self.err.code.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.err, f)
    }
}

impl std::error::Error for Error {}
