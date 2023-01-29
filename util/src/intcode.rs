use std::str::FromStr;

pub struct IntCode {
    start_ops: Vec<isize>,
    pub ops: Vec<isize>,
}

impl IntCode {
    pub fn compute(&mut self) {
        let size: usize = self.ops.len();
        let mut idx: usize = 0;
        while self.ops[idx] != 99 {
            let op: isize = self.ops[idx];
            let idx_a = self.ops[idx + 1] as usize;
            if idx_a >= size {
                self.reset();
                break;
            }
            let a: isize = self.ops[idx_a];
            let idx_b = self.ops[idx + 2] as usize;
            if idx_b >= size {
                self.reset();
                break;
            }
            let b: isize = self.ops[idx_b];
            let target: usize = self.ops[idx + 3] as usize;
            if target >= size {
                self.reset();
                break;
            }

            let result: isize = if op == 1 { a + b } else { a * b };
            self.ops[target] = result;
            idx += 4;
        }
    }

    pub fn set(&mut self, pos: usize, n: isize) {
        self.ops[pos] = n
    }

    pub fn pos(&self, n: usize) -> isize {
        self.ops[n]
    }

    pub fn reset(&mut self) {
        self.ops = self.start_ops.clone();
    }
}

impl FromStr for IntCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<isize> = s.split(',').map(|n| n.parse::<isize>().unwrap()).collect();
        Ok(IntCode {
            start_ops: ops.clone(),
            ops,
        })
    }
}
