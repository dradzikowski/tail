use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::prelude::*;
use std::io::BufReader;

enum TailMode {
    Lines(i64),
    Bytes(i64),
}

fn main() -> Result<(), TailError> {
    let mut f = File::open("./file.txt")?;
    let offset = find_offset(&mut f, TailMode::Bytes(10))?;


    return Ok(());
}

fn find_offset(file: &mut File, mode: TailMode) -> Result<(), TailError> {
    match mode {
        TailMode::Lines(l) => {
            Err(TailError {
                msg: "Option not yet supported".to_string()
            })
        }
        TailMode::Bytes(b) => {
            file.seek(SeekFrom::End(-b))?;
            let mut buffer = [0; 512];

            let bytes_read = file.read(&mut buffer)?;
            let data = &buffer[..bytes_read];
            io::stdout().write(data)?;
            Ok(())
        }
    }
}


#[derive(Debug)]
struct TailError {
    msg: String
}

impl From<io::Error> for TailError {
    fn from(io_error: io::Error) -> Self {
        Self {
            msg: io_error.to_string()
        }
    }
}