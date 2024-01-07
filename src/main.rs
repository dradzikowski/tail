use std::cmp;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::prelude::*;

use crate::error::Error;

mod error;

enum Mode {
    Lines(usize),
    Bytes(usize),
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
            let size = file.metadata()?.len();
            let effective_count = cmp::min(count, size as usize);
            file.seek(SeekFrom::End(-(effective_count as i64)))?;

            let mut buffer: Vec<u8> = Vec::with_capacity(count);
            file.read_to_end(&mut buffer)?;

            let str = std::str::from_utf8(&buffer)?;
            write!(output, "{}", str)?;
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

        let result = tail(&mut file, Mode::Lines(2), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output, "lines: 2".as_bytes());
    }

    #[test]
    fn tail_trailing_new_lines() {
        let mut file = File::open("./how-are-you.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let result = tail(&mut file, Mode::Bytes(2), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output, "\n\n".as_bytes());
    }

    #[test]
    fn tail_small_piece_of_file() {
        let mut file = File::open("./how-are-you.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let result = tail(&mut file, Mode::Bytes(6), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output, "you?\n\n".as_bytes());
    }

    #[test]
    fn tail_all_bytes_available() {
        let mut file = File::open("./how-are-you.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let result = tail(&mut file, Mode::Bytes(14), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
    }

    #[test]
    fn tail_whole_file_when_requested_excessive_bytes() {
        let mut file = File::open("./how-are-you.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let result = tail(&mut file, Mode::Bytes(100), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
    }

    #[test]
    fn tail_huge_piece_of_file() {
        let mut file = File::open("./large-file.txt").unwrap();
        let mut output: Vec<u8> = Vec::new();

        let result = tail(&mut file, Mode::Bytes(50000), &mut output).unwrap();

        assert_eq!(result, ());
        assert_eq!(output.len(), 50000);
    }
}