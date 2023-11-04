use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::io::BufReader;
use std::io::prelude::*;

enum TailMode {
    Lines(i64),
    Bytes(i64),
}

fn main() -> Result<(), TailError> {
    let mut f = File::open("./file.txt")?;
    let offset = find_offset(&mut f, TailMode::Lines(5))?;


    return Ok(());
}

fn find_offset(file: &mut File, mode: TailMode) -> Result<(), TailError> {
    match mode {
        TailMode::Lines(l) => {
            let mut reader = BufReader::new(file);
            let mut i = 1i64;
            let mut buffer: Vec<u8> = Vec::new();


            let eof = reader.seek(SeekFrom::End(0)).unwrap();
            println!("EOF: {}", eof);
            let last_run = 512u64.min(eof) == eof;
            let pos = reader.seek(SeekFrom::End(-i * 512u64.min(eof) as i64));
            println!("position: {}", pos.unwrap());
            reader.read_to_end(&mut buffer)?;
            let mut newlines = 0;
            let mut backward_position = 0;
            /*for item in &buffer2 {
                println!("Item: {item}");
            }*/
            for single_byte in buffer.iter().rev() {
                backward_position = backward_position + 1;
                if *single_byte == '\n' as u8 {
                    //println!("Single byte: {}", *single_byte as char);
                    newlines = newlines + 1;
                }
                if newlines == l {
                    io::stdout().write(&buffer.as_slice()[(eof as usize - backward_position)..])?;
                    return Ok(())
                }
            }
            println!("Newlines: {newlines}");

            Ok(())
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
    msg: String,
}

impl From<io::Error> for TailError {
    fn from(io_error: io::Error) -> Self {
        Self {
            msg: io_error.to_string()
        }
    }
}