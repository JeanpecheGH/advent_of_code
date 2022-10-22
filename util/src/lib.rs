use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines, Read};

pub fn file_as_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn file_as_lines(path: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
