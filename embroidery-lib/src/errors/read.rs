use std::result;

use super::{ErrorWithContext, StdError};

#[derive(Fail, Debug)]
#[non_exhaustive]
pub enum Error {
    #[fail(display = "Invalid format: {}\nError Context(deepest-first):", _0)]
    InvalidFormat(String, Vec<String>),
    #[fail(display = "Unexpected End of File: {}\nError Context(deepest-first):", _0)]
    UnexpectedEof(String, #[cause] std::io::Error, Vec<String>),
    #[fail(display = "{}\nError Context(deepest-first):", _0)]
    Std(#[cause] StdError, Vec<String>),
}

impl Error {
    pub fn invalid_format<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        Self::InvalidFormat(msg.into(), vec![])
    }
    pub fn unexpected_eof<S>(msg: S, err: std::io::Error) -> Self
    where
        S: Into<String>,
    {
        Self::UnexpectedEof(msg.into(), err, vec![])
    }
}

impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        let err = err.into();
        if let StdError::Io(ioe) = err {
            if ioe.kind() == std::io::ErrorKind::UnexpectedEof {
                panic!("You should not rely on the automatic conversion of UnexpectedEof errors into ReadErrors. Use the `read...` macros, or implement your own handling");
            }
            Error::Std(StdError::Io(ioe), vec![])
        } else {
            Error::Std(err, vec![])
        }
    }
}

impl ErrorWithContext for Error {
    fn context(&self) -> Vec<String> {
        match self {
            Self::InvalidFormat(_, c) => c.clone(),
            Self::UnexpectedEof(_, _, c) => c.clone(),
            Self::Std(_, c) => c.clone(),
        }
    }
    fn with_additional_context<S>(self, extra: S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Self::InvalidFormat(e, mut c) => {
                c.push(extra.into());
                Self::InvalidFormat(e, c)
            },
            Self::UnexpectedEof(m, e, mut c) => {
                c.push(extra.into());
                Self::UnexpectedEof(m, e, c)
            },
            Self::Std(e, mut c) => {
                c.push(extra.into());
                Self::Std(e, c)
            },
        }
    }
    fn without_context(self) -> Self {
        match self {
            Self::InvalidFormat(e, _) => Self::InvalidFormat(e, vec![]),
            Self::UnexpectedEof(m, e, _) => Self::UnexpectedEof(m, e, vec![]),
            Self::Std(e, _) => Self::Std(e, vec![]),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
