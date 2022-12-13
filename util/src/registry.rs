use std::cell::Cell;

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

    pub fn is_zero(&self) -> bool {
        self.value.get() == 0
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
    use super::*;

    #[test]
    fn basic_registry() {
        let reg: Registry = Registry::new(-5);
        reg.triple();
        assert_eq!(reg.get(), -15);
    }
}
