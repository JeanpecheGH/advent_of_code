pub mod registry;

use std::fs::File;
use std::io::{Error, Read};

pub fn file_as_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
