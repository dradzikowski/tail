use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self {
            msg: io_error.to_string()
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(utf8_error: Utf8Error) -> Self {
        Self {
            msg: utf8_error.to_string()
        }
    }
}