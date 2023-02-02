use std::str::FromStr;

type Err = ();

#[derive(Clone)]
pub struct IntCode {
    start_ops: Vec<isize>,
    pub ops: Vec<isize>,
    idx: usize,
    pub output: Vec<isize>,
}

impl IntCode {
    pub fn compute(&mut self, inputs: &mut Vec<isize>) {
        inputs.reverse();
        loop {
            let res: Result<(), Err> = self.one_op(inputs);
            if res.is_err() {
                break;
            }
        }
    }

    fn get_value(&self, offset: usize, params: &[bool]) -> Result<isize, Err> {
        let param: bool = params.get(offset - 1).copied().unwrap_or(false);
        if param {
            Ok(self.ops[self.idx + offset])
        } else {
            let index: usize = self.ops[self.idx + offset] as usize;
            if index >= self.ops.len() {
                Err(())
            } else {
                Ok(self.ops[index])
            }
        }
    }

    fn write_value(&mut self, offset: usize, value: isize) -> Result<(), Err> {
        let index: usize = self.ops[self.idx + offset] as usize;
        if index >= self.ops.len() {
            Err(())
        } else {
            self.ops[index] = value;
            Ok(())
        }
    }

    fn one_op(&mut self, inputs: &mut Vec<isize>) -> Result<(), Err> {
        let i: usize = self.idx;
        let (op, params) = self.op_and_params(i);
        match op {
            1 => {
                //Add
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, a + b)?;
                self.idx += 4;
                Ok(())
            }
            2 => {
                //Mult
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, a * b)?;
                self.idx += 4;
                Ok(())
            }
            3 => {
                //Read input
                self.write_value(1, inputs.pop().ok_or(())?)?;
                self.idx += 2;
                Ok(())
            }
            4 => {
                //Write output
                let a: isize = self.get_value(1, &params)?;
                self.idx += 2;
                self.output.push(a);
                Ok(())
            }
            5 => {
                //Jump if true
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.idx = if a != 0 { b as usize } else { self.idx + 3 };
                Ok(())
            }
            6 => {
                //Jump if false
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.idx = if a == 0 { b as usize } else { self.idx + 3 };
                Ok(())
            }
            7 => {
                //Is lower
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, (a < b) as isize)?;
                self.idx += 4;
                Ok(())
            }
            8 => {
                //Is equal
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, (a == b) as isize)?;
                self.idx += 4;
                Ok(())
            }
            _ => Err(()), //End
        }
    }

    fn op_and_params(&self, n: usize) -> (isize, Vec<bool>) {
        let mut v: isize = self.ops[n];
        let op: isize = v % 100;
        v /= 100;
        let mut params: Vec<bool> = Vec::new();
        while v > 0 {
            let p: bool = v % 10 > 0;
            params.push(p);
            v /= 10;
        }
        (op, params)
    }

    pub fn set(&mut self, pos: usize, n: isize) {
        self.ops[pos] = n
    }

    pub fn pos(&self, n: usize) -> isize {
        self.ops[n]
    }

    pub fn reset(&mut self) {
        self.ops = self.start_ops.clone();
        self.idx = 0;
        self.output.clear();
    }
}

impl FromStr for IntCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<isize> = s.split(',').map(|n| n.parse::<isize>().unwrap()).collect();
        Ok(IntCode {
            start_ops: ops.clone(),
            ops,
            idx: 0,
            output: Vec::new(),
        })
    }
}
