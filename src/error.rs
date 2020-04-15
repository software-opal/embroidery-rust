use embroidery_lib::errors::{Error as EmbError, ErrorWithContext, ReadError, StdError as EmbStdError, WriteError};

use simplelog::TermLogError;
use std::fmt;
use std::io;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Embroidery Read Error: {}", _0)]
    EmbRead(#[cause] ReadError),
    #[fail(display = "Embroidery Write Error: {}", _0)]
    EmbWrite(#[cause] WriteError),

    #[fail(display = "IO Error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Formatting Error: {}", _0)]
    Fmt(#[cause] fmt::Error),

    #[fail(display = "Logger Error: {}", _0)]
    Log(#[cause] TermLogError),

    #[fail(display = "Other Error: {}", _0)]
    Custom(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Fmt(err)
    }
}
impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        Error::EmbRead(err)
    }
}
impl From<WriteError> for Error {
    fn from(err: WriteError) -> Self {
        Error::EmbWrite(err)
    }
}
impl From<EmbError> for Error {
    fn from(err: EmbError) -> Self {
        match err {
            EmbError::Read(e, ctx) => Error::EmbRead(e.set_context(ctx)),
            EmbError::Write(e, ctx) => Error::EmbWrite(e.set_context(ctx)),
            // We loose the context here.
            EmbError::Standard(e, _ctx) => e.into(),
        }
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Custom(err.to_string())
    }
}

impl From<EmbStdError> for Error {
    fn from(err: EmbStdError) -> Self {
        match err {
            EmbStdError::Fmt(e) => e.into(),
            EmbStdError::Io(e) => e.into(),
        }
    }
}

impl From<TermLogError> for Error {
    fn from(err: TermLogError) -> Self {
        Error::Log(err)
    }
}
