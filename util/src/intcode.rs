use std::collections::VecDeque;
use std::str::FromStr;

const MEMSIZE: usize = 20000;

type Err = ();

#[derive(Clone)]
pub struct IntCode {
    start_ops: Vec<isize>,
    pub ops: Vec<isize>,
    idx: usize,
    relative_base: isize,
    input: VecDeque<isize>,
    pub output: Vec<isize>,
    infinite: bool,
}

impl IntCode {
    pub fn compute(&mut self, inputs: Vec<isize>) {
        self.add_input(inputs);
        loop {
            let res: Result<(), Err> = self.one_op();
            if res.is_err() {
                break;
            }
        }
    }

    pub fn add_input(&mut self, input: Vec<isize>) {
        input.into_iter().for_each(|i| {
            self.input.push_front(i);
        })
    }

    pub fn compute_one(&mut self) {
        let _ = self.one_op();
    }

    fn get_value(&self, offset: usize, params: &[isize]) -> Result<isize, Err> {
        let param: isize = params.get(offset - 1).copied().unwrap_or(0);
        let index: usize = match param {
            1 => self.idx + offset,
            2 => {
                let base_offset: isize = self.ops[self.idx + offset];
                (self.relative_base + base_offset) as usize
            }
            _ => self.ops[self.idx + offset] as usize,
        };
        if index >= self.ops.len() {
            Err(())
        } else {
            Ok(self.ops[index])
        }
    }

    fn write_value(&mut self, offset: usize, value: isize, params: &[isize]) -> Result<(), Err> {
        let param: isize = params.get(offset - 1).copied().unwrap_or(0);
        let index: usize = match param {
            2 => {
                let base_offset: isize = self.ops[self.idx + offset];
                (self.relative_base + base_offset) as usize
            }
            _ => self.ops[self.idx + offset] as usize,
        };
        if index >= self.ops.len() {
            Err(())
        } else {
            self.ops[index] = value;
            Ok(())
        }
    }

    fn get_input(&mut self) -> Result<isize, Err> {
        if self.infinite {
            Ok(self.input.pop_back().unwrap_or(-1))
        } else {
            self.input.pop_back().ok_or(())
        }
    }

    fn one_op(&mut self) -> Result<(), Err> {
        let i: usize = self.idx;
        let (op, params) = self.op_and_params(i);
        match op {
            1 => {
                //Add
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, a + b, &params)?;
                self.idx += 4;
                Ok(())
            }
            2 => {
                //Mult
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, a * b, &params)?;
                self.idx += 4;
                Ok(())
            }
            3 => {
                //Read input
                let input: isize = self.get_input()?;
                self.write_value(1, input, &params)?;
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
                self.write_value(3, (a < b) as isize, &params)?;
                self.idx += 4;
                Ok(())
            }
            8 => {
                //Is equal
                let a: isize = self.get_value(1, &params)?;
                let b: isize = self.get_value(2, &params)?;
                self.write_value(3, (a == b) as isize, &params)?;
                self.idx += 4;
                Ok(())
            }
            9 => {
                //Adjust relative base
                let a: isize = self.get_value(1, &params)?;
                self.relative_base += a;
                self.idx += 2;
                Ok(())
            }
            _ => Err(()), //End
        }
    }

    fn op_and_params(&self, n: usize) -> (isize, Vec<isize>) {
        let mut v: isize = self.ops[n];
        let op: isize = v % 100;
        v /= 100;
        let mut params: Vec<isize> = Vec::new();
        while v > 0 {
            let p: isize = v % 10;
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

    pub fn write_cmd(&mut self, cmd: &str) {
        let mut input: Vec<isize> = cmd.as_bytes().iter().map(|&c| c as isize).collect();
        input.push(10);
        self.compute(input);
    }

    pub fn read_prompt(&mut self) -> Option<String> {
        if let Some(10) = self.output.last() {
            let chars: Vec<u8> = self.output.iter().map(|&n| n as u8).collect();
            let prompt: Option<String> = String::from_utf8(chars).ok();
            self.output.clear();
            prompt
        } else {
            None
        }
    }

    pub fn set_inifinite(&mut self) {
        self.infinite = true;
    }

    pub fn reset(&mut self) {
        self.ops.clone_from(&self.start_ops);
        self.idx = 0;
        self.relative_base = 0;
        self.input.clear();
        self.output.clear();
    }
}

impl FromStr for IntCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ops: Vec<isize> = s.split(',').map(|n| n.parse::<isize>().unwrap()).collect();
        let size: usize = MEMSIZE - ops.len();
        ops.extend(vec![0; size].iter());
        Ok(IntCode {
            start_ops: ops.clone(),
            ops,
            idx: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            infinite: false,
        })
    }
}
