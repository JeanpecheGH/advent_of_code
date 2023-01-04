pub mod chinese_remainders;
pub mod registry;

use std::fs::File;
use std::io::{Error, Read};

const DOUBLE_LF: &str = "\n\n";
const DOUBLE_CRLF: &str = "\r\n\r\n";

pub fn file_as_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn split_blocks(data: &str) -> Vec<&str> {
    data.split(DOUBLE_CRLF)
        .flat_map(|l| l.split(DOUBLE_LF).collect::<Vec<&str>>())
        .collect()
}
