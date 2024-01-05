use std::io;

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