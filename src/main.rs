use std::fs;
use std::env;
use std::io::{self, BufRead, BufReader};

extern crate blake2;
use blake2::{Blake2s, Digest};

fn main() -> io::Result<()> {
    let input = env::args().nth(1);
    let mut reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).expect("Failed to open file."))),
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut hasher = Blake2s::new();
    hasher.input(buffer);

    println!("{:x}", hasher.result());
    Ok(())
}
