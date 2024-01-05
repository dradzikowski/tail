use std::fs::File;
use std::io::{Read, Seek, Write};
use std::io::prelude::*;

use crate::error::Error;

mod error;

enum Mode {
    Lines(i64),
    Bytes(i64),
}

fn main() -> Result<(), Error> {
    return Ok(());
}

fn tail<T: Write>(file: &mut File, mode: Mode, output: &mut T) -> Result<(), Error> {
    match mode {
        Mode::Lines(count) => {
            write!(output, "lines: {}", count).unwrap();
            Ok(())
        }
        Mode::Bytes(count) => {
            write!(output, "bytes: {}", count).unwrap();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{Mode, tail};

    #[test]
    fn tail_returns_back_mode_and_count() {
        let mut file = File::open("./how-are-you.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let offset = tail(&mut file, Mode::Lines(2), &mut output).unwrap();

        assert_eq!(offset, ());
        assert_eq!(output, "lines: 2".as_bytes());
    }
}