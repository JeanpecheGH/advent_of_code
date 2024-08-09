pub mod basic_parser;
pub mod chinese_remainders;
pub mod coord;
pub mod hashers;
pub mod intcode;
pub mod orientation;
pub mod registry;
pub mod wrist_device;

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

pub fn gcd(a: isize, b: isize) -> isize {
    let mut dd: isize = a.abs();
    let mut dv: isize = b.abs();

    while dv != 0 {
        let r: isize = dd % dv;
        dd = dv;
        dv = r;
    }
    dd
}

pub fn lcm(a: isize, b: isize) -> isize {
    let g: isize = gcd(a, b);
    a * b / g
}
