use std::cell::Cell;
use std::fs::File;
use std::io::{Error, Read};

pub fn file_as_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug)]
pub struct Registry {
    value: Cell<isize>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Registry {
    pub fn new(n: isize) -> Self {
        Registry {
            value: Cell::new(n),
        }
    }

    pub fn is_one(&self) -> bool {
        self.value.get() == 1
    }

    pub fn is_even(&self) -> bool {
        self.value.get() % 2 == 0
    }

    pub fn incr(&self) {
        self.value.set(self.value.get() + 1);
    }

    pub fn decr(&self) {
        self.value.set(self.value.get() - 1);
    }

    pub fn half(&self) {
        self.value.set(self.value.get() >> 1);
    }

    pub fn triple(&self) {
        self.value.set(self.value.get() * 3);
    }

    pub fn set(&self, n: isize) {
        self.value.set(n)
    }

    pub fn get(&self) -> isize {
        self.value.get()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
