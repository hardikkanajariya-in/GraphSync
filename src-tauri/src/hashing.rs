use std::io::Read;
use std::path::Path;

use anyhow::Result;
use sha2::{Digest, Sha256};

const CHUNK_SIZE: usize = 64 * 1024;

pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn sha256_reader(reader: &mut dyn Read) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_SIZE];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    sha256_reader(&mut file)
}

pub fn hashes_equal(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}
