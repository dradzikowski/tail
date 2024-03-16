use std::cmp;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::io::prelude::*;

use crate::error::Error;

mod error;

const FIXED_BUFFER_SIZE: i64 = 1024;

enum Mode {
    Lines(usize),
    Bytes(usize),
}

fn main() -> Result<(), Error> {
    return Ok(());
}

fn tail<T: Write>(file: &mut File, mode: Mode, output: &mut T) -> Result<(), Error> {
    match mode {
        Mode::Lines(lines_count) => {
            // go to the end
            // move back a number of bytes - buffer_size
            // read data of buffer_size to the buffer
            // analyse it backwards in the buffer to find new lines
            // new lines count = '\n'
            // when we found desired new lines count, after reading X buffers and in the middle of X+1 buffer
            // we set a position to X*buffer_size+backward_position_of_last_new_line_in_the_last_buffer ;)
            // we print everything from here

            let mut i = 0;

            let size = file.metadata()?.len();
            let mut lines_found = 0;
            let mut reader = BufReader::new(file);

            // where we are generally reading backwards
            let mut current_backward_position = 0;

            while lines_count > lines_found {
                i = i + 1;
                // how many bytes we need to move from the end to start reading in this iteration
                let backward_start_position = cmp::min(i * FIXED_BUFFER_SIZE, size as i64);

                // 1024, 2048, ..., 4096, 4199 (103)
                // 4199 % BUFFER_SIZE = 103 or 0 => BUFFER_SIZE
                let buffer_size = match backward_start_position % FIXED_BUFFER_SIZE {
                    0 => FIXED_BUFFER_SIZE,
                    buffer_size => buffer_size
                };

                let mut buffer: [u8; FIXED_BUFFER_SIZE as usize] = [0; FIXED_BUFFER_SIZE as usize];

                reader.seek(SeekFrom::End(-backward_start_position))?;
                let _ = reader.read(&mut buffer);

                for &byte in buffer[..buffer_size as usize].iter().rev() {
                    current_backward_position = current_backward_position + 1;
                    if byte == b'\n' {
                        lines_found = lines_found + 1;
                    }
                    if lines_found == lines_count {
                        let mut buffer: Vec<u8> = Vec::with_capacity(current_backward_position as usize);
                        reader.seek(SeekFrom::End(-current_backward_position))?;
                        reader.read_to_end(&mut buffer)?;

                        let str = std::str::from_utf8(&buffer)?;
                        write!(output, "{}", str)?;

                        return Ok(());
                    }
                }
            }


            write!(output, "lines: {}", lines_count).unwrap();
            Ok(())
        }
        Mode::Bytes(bytes_count) => {
            let size = file.metadata()?.len();
            let effective_bytes_count = cmp::min(bytes_count, size as usize);
            file.seek(SeekFrom::End(-(effective_bytes_count as i64)))?;

            let mut buffer: Vec<u8> = Vec::with_capacity(effective_bytes_count);
            file.read_to_end(&mut buffer)?;

            let str = std::str::from_utf8(&buffer)?;
            write!(output, "{}", str)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    mod tail_bytes {
        use std::fs::File;

        use crate::{Mode, tail};

        #[test]
        fn output_trailing_new_lines_when_file_ends_with_them() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Bytes(2), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "\n\n".as_bytes());
        }

        #[test]
        fn output_requested_bytes_when_they_are_subset_of_all_available_bytes() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Bytes(6), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "you?\n\n".as_bytes());
        }

        #[test]
        fn output_whole_file_when_all_available_bytes_requested() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Bytes(14), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
        }

        #[test]
        fn output_whole_file_when_more_than_available_bytes_requested() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Bytes(100), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
        }

        #[test]
        fn output_requested_50k_bytes_when_file_is_over_6_megabytes_size() {
            let mut file = File::open("./large-file.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Bytes(50000), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output.len(), 50000);
        }
    }

    mod tail_lines {
        use std::fs::File;

        use crate::{Mode, tail};

        #[test]
        fn output_empty_lines_when_file_ends_with_them() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Lines(2), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "\n\n".as_bytes());
        }

        #[test]
        fn output_requested_lines_when_they_are_subset_of_all_available_lines() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Lines(4), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "are\nyou?\n\n".as_bytes());
        }

        #[test]
        fn output_whole_file_when_all_available_lines_requested() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Lines(5), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
        }

        #[test]
        fn output_whole_file_when_more_than_available_lines_requested() {
            let mut file = File::open("./how-are-you.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Lines(100), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output, "How\nare\nyou?\n\n".as_bytes());
        }

        #[test]
        fn output_requested_1k_lines_when_file_is_over_6_megabytes_size() {
            let mut file = File::open("./large-file.txt").unwrap();
            let mut output: Vec<u8> = Vec::new();

            let result = tail(&mut file, Mode::Lines(1000), &mut output).unwrap();

            assert_eq!(result, ());
            assert_eq!(output.len(), 1000);
        }
    }
}