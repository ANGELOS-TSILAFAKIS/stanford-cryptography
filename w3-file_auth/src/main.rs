use std::env;
use std::fs::File;
use std::fs::Metadata;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::process;

use sha2::{Sha256, Digest};

const KB: u64 = 1024;
const DEFAULT_BUF_SIZE: usize = 1024;

fn total_blocks(metadata: &Metadata) -> u64 {
    let size = metadata.len();
    println!("File size: {}", size);

    let pos = size as f64 / KB as f64;
    pos.ceil() as u64
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() < 1 {
        println!("Error: File path argument is missing");
        process::exit(1);
    }
    let filename = &args[0];

    let path = Path::new(filename);
    let mut f = File::open(path)?;
    let metadata = f.metadata()?;

    // Move cursor to last block of the file
    let pos = (total_blocks(&metadata) - 1) * KB;
    f.seek(SeekFrom::Start(pos))?;

    // Read last block
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let len = match f.read(&mut buf) {
        Ok(0) => return Ok(()),
        Ok(len) => len,
        Err(e) => return Err(e),
    };

    println!("Block length: {}", len);

    // Hash last block
    let mut hasher = Sha256::new();
    hasher.input(&buf.to_vec());
    let result = hasher.result();

    println!("Hash: {:x}", result);

    Ok(())
}
