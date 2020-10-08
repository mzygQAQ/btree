use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    UnexpectedError,
    TryFromSliceError(&'static str),
    UTF8Error,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::UnexpectedError
    }
}
