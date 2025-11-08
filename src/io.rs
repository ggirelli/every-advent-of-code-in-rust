use anyhow::{Error, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::fs::read_to_string;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use utf8_decode::UnsafeDecoder;

/// Get all chars from a file content. Supports GZ compression.
pub fn read_chars(input: PathBuf) -> Result<Vec<char>, Error> {
    Ok(if input.to_str().unwrap_or(".").ends_with(".gz") {
        UnsafeDecoder::new(BufReader::new(GzDecoder::new(File::open(input)?)).bytes())
            .map(|r| r.unwrap())
            .collect()
    } else {
        read_to_string(input)?.chars().collect()
    })
}

/// Get all lines from a file content. Supports GZ compression.
pub fn read_lines(input: PathBuf) -> Result<Vec<String>, Error> {
    Ok(if input.to_str().unwrap_or(".").ends_with(".gz") {
        BufReader::new(GzDecoder::new(File::open(input)?))
            .lines()
            .map(|l| l.unwrap())
            .collect()
    } else {
        read_to_string(input)?
            .lines()
            .map(|l| l.to_string())
            .collect()
    })
}

/// Get file content as a single string. Supports GZ compression.
pub fn read(input: PathBuf) -> Result<String, Error> {
    Ok(if input.to_str().unwrap_or(".").ends_with(".gz") {
        let mut out_string: String = String::new();
        BufReader::new(GzDecoder::new(File::open(input)?)).read_to_string(&mut out_string)?;
        out_string
    } else {
        read_to_string(input)?
    })
}
