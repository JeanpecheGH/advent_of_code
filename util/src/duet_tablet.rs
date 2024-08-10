use crate::basic_parser::parse_isize;
use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
enum DuetOp {
    SendVal(isize),
    SendReg(char),
    ReceiveVal(isize),
    ReceiveReg(char),
    SetVal(char, isize),
    SetReg(char, char),
    AddVal(char, isize),
    AddReg(char, char),
    MulVal(char, isize),
    MulReg(char, char),
    ModVal(char, isize),
    ModReg(char, char),
    JumpGTZValVal(isize, isize),
    JumpGTZRegVal(char, isize),
    JumpGTZRegReg(char, char),
}

impl FromStr for DuetOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_send_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, v) = preceded(tag("snd "), parse_isize)(s)?;
            Ok((s, DuetOp::SendVal(v)))
        }
        fn parse_send_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, r) = preceded(tag("snd "), anychar)(s)?;
            Ok((s, DuetOp::SendReg(r)))
        }
        fn parse_receive_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, v) = preceded(tag("rcv "), parse_isize)(s)?;
            Ok((s, DuetOp::ReceiveVal(v)))
        }
        fn parse_receive_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, r) = preceded(tag("rcv "), anychar)(s)?;
            Ok((s, DuetOp::ReceiveReg(r)))
        }
        fn parse_set_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r, v)) =
                preceded(tag("set "), separated_pair(anychar, char(' '), parse_isize))(s)?;
            Ok((s, DuetOp::SetVal(r, v)))
        }
        fn parse_set_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r1, r2)) =
                preceded(tag("set "), separated_pair(anychar, char(' '), anychar))(s)?;
            Ok((s, DuetOp::SetReg(r1, r2)))
        }
        fn parse_add_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r, v)) =
                preceded(tag("add "), separated_pair(anychar, char(' '), parse_isize))(s)?;
            Ok((s, DuetOp::AddVal(r, v)))
        }
        fn parse_add_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r1, r2)) =
                preceded(tag("add "), separated_pair(anychar, char(' '), anychar))(s)?;
            Ok((s, DuetOp::AddReg(r1, r2)))
        }
        fn parse_mul_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r, v)) =
                preceded(tag("mul "), separated_pair(anychar, char(' '), parse_isize))(s)?;
            Ok((s, DuetOp::MulVal(r, v)))
        }
        fn parse_mul_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r1, r2)) =
                preceded(tag("mul "), separated_pair(anychar, char(' '), anychar))(s)?;
            Ok((s, DuetOp::MulReg(r1, r2)))
        }
        fn parse_mod_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r, v)) =
                preceded(tag("mod "), separated_pair(anychar, char(' '), parse_isize))(s)?;
            Ok((s, DuetOp::ModVal(r, v)))
        }
        fn parse_mod_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r1, r2)) =
                preceded(tag("mod "), separated_pair(anychar, char(' '), anychar))(s)?;
            Ok((s, DuetOp::ModReg(r1, r2)))
        }
        fn parse_jum_gtz_val_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (v1, v2)) = preceded(
                tag("jgz "),
                separated_pair(parse_isize, char(' '), parse_isize),
            )(s)?;
            Ok((s, DuetOp::JumpGTZValVal(v1, v2)))
        }
        fn parse_jum_gtz_reg_val(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r, v)) =
                preceded(tag("jgz "), separated_pair(anychar, char(' '), parse_isize))(s)?;
            Ok((s, DuetOp::JumpGTZRegVal(r, v)))
        }
        fn parse_jum_gtz_reg_reg(s: &str) -> IResult<&str, DuetOp> {
            let (s, (r1, r2)) =
                preceded(tag("jgz "), separated_pair(anychar, char(' '), anychar))(s)?;
            Ok((s, DuetOp::JumpGTZRegReg(r1, r2)))
        }

        Ok(alt((
            parse_send_val,
            parse_send_reg,
            parse_receive_val,
            parse_receive_reg,
            parse_set_val,
            parse_set_reg,
            parse_add_val,
            parse_add_reg,
            parse_mul_val,
            parse_mul_reg,
            parse_mod_val,
            parse_mod_reg,
            parse_jum_gtz_val_val,
            parse_jum_gtz_reg_val,
            parse_jum_gtz_reg_reg,
        ))(s)
        .unwrap()
        .1)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum OpResult {
    Nothing,
    SentData,
    Waiting,
}

#[derive(Debug)]
pub struct DuetTablet {
    ops: Vec<DuetOp>,
}

impl DuetTablet {
    pub fn play_solo(&self) -> isize {
        let mut regs: FxHashMap<char, isize> = FxHashMap::default();
        let mut op: isize = 0;
        let mut sound: isize = 0;

        while (op as usize) < self.ops.len() {
            match self.ops[op as usize] {
                //For solo play, "snd" is the "Sound" instruction
                DuetOp::SendVal(n) => sound = n,
                DuetOp::SendReg(r) => sound = *regs.entry(r).or_insert(0),
                //For solo play, "rcv" is the "Recover" instruction
                DuetOp::ReceiveVal(n) => {
                    if n != 0 {
                        return sound;
                    }
                }
                DuetOp::ReceiveReg(r) => {
                    if *regs.entry(r).or_insert(0) != 0 {
                        return sound;
                    }
                }
                DuetOp::SetVal(r, v) => {
                    let _ = regs.insert(r, v);
                }
                DuetOp::SetReg(r1, r2) => {
                    let r2_v: isize = *regs.entry(r2).or_insert(0);
                    let _ = regs.insert(r1, r2_v);
                }
                DuetOp::AddVal(r, v) => *regs.entry(r).or_insert(0) += v,
                DuetOp::AddReg(r1, r2) => {
                    *regs.entry(r1).or_insert(0) += *regs.entry(r2).or_insert(0)
                }
                DuetOp::MulVal(r, v) => *regs.entry(r).or_insert(0) *= v,
                DuetOp::MulReg(r1, r2) => {
                    *regs.entry(r1).or_insert(0) *= *regs.entry(r2).or_insert(0)
                }
                DuetOp::ModVal(r, v) => *regs.entry(r).or_insert(0) %= v,
                DuetOp::ModReg(r1, r2) => {
                    *regs.entry(r1).or_insert(0) %= *regs.entry(r2).or_insert(0)
                }
                DuetOp::JumpGTZValVal(v1, v2) => {
                    if v1 > 0 {
                        op += v2 - 1;
                    }
                }
                DuetOp::JumpGTZRegVal(r, v) => {
                    if *regs.entry(r).or_insert(0) > 0 {
                        op += v - 1;
                    }
                }
                DuetOp::JumpGTZRegReg(r1, r2) => {
                    if *regs.entry(r1).or_insert(0) > 0 {
                        op += *regs.entry(r2).or_insert(0) - 1;
                    }
                }
            }
            op += 1;
        }
        0
    }

    fn apply_op(
        &self,
        regs: &mut FxHashMap<char, isize>,
        read_queue: &mut VecDeque<isize>,
        write_queue: &mut VecDeque<isize>,
        op: &mut isize,
    ) -> OpResult {
        if *op < 0 || (*op as usize) >= self.ops.len() {
            return OpResult::Waiting;
        }

        match self.ops[*op as usize] {
            DuetOp::SendVal(n) => {
                write_queue.push_back(n);
                *op += 1;
                return OpResult::SentData;
            }
            DuetOp::SendReg(r) => {
                write_queue.push_back(*regs.entry(r).or_insert(0));
                *op += 1;
                return OpResult::SentData;
            }
            DuetOp::ReceiveVal(_) => (), //Should not happen
            DuetOp::ReceiveReg(r) => {
                if let Some(v) = read_queue.pop_front() {
                    *regs.entry(r).or_insert(0) = v;
                } else {
                    return OpResult::Waiting;
                }
            }
            DuetOp::SetVal(r, v) => {
                let _ = regs.insert(r, v);
            }
            DuetOp::SetReg(r1, r2) => {
                let r2_v: isize = *regs.entry(r2).or_insert(0);
                let _ = regs.insert(r1, r2_v);
            }
            DuetOp::AddVal(r, v) => *regs.entry(r).or_insert(0) += v,
            DuetOp::AddReg(r1, r2) => *regs.entry(r1).or_insert(0) += *regs.entry(r2).or_insert(0),
            DuetOp::MulVal(r, v) => *regs.entry(r).or_insert(0) *= v,
            DuetOp::MulReg(r1, r2) => *regs.entry(r1).or_insert(0) *= *regs.entry(r2).or_insert(0),
            DuetOp::ModVal(r, v) => *regs.entry(r).or_insert(0) %= v,
            DuetOp::ModReg(r1, r2) => *regs.entry(r1).or_insert(0) %= *regs.entry(r2).or_insert(0),
            DuetOp::JumpGTZValVal(v1, v2) => {
                if v1 > 0 {
                    *op += v2 - 1;
                }
            }
            DuetOp::JumpGTZRegVal(r, v) => {
                if *regs.entry(r).or_insert(0) > 0 {
                    *op += v - 1;
                }
            }
            DuetOp::JumpGTZRegReg(r1, r2) => {
                if *regs.entry(r1).or_insert(0) > 0 {
                    *op += *regs.entry(r2).or_insert(0) - 1;
                }
            }
        }

        *op += 1;
        OpResult::Nothing
    }

    pub fn play_duo(&self) -> isize {
        let mut regs_zero: FxHashMap<char, isize> = FxHashMap::default();
        regs_zero.insert('p', 0);
        let mut regs_one: FxHashMap<char, isize> = FxHashMap::default();
        regs_one.insert('p', 1);

        let mut queue_zero: VecDeque<isize> = VecDeque::new();
        let mut queue_one: VecDeque<isize> = VecDeque::new();

        let mut op_zero: isize = 0;
        let mut op_one: isize = 0;

        let mut nb_send_one: isize = 0;

        //If both program stopped or stuck in "rcv", stop process
        loop {
            //apply zero
            let ret_zero: OpResult = self.apply_op(
                &mut regs_zero,
                &mut queue_zero,
                &mut queue_one,
                &mut op_zero,
            );
            //apply one
            let ret_one: OpResult =
                self.apply_op(&mut regs_one, &mut queue_one, &mut queue_zero, &mut op_one);

            if ret_one == OpResult::SentData {
                nb_send_one += 1;
            }

            if ret_zero == OpResult::Waiting && ret_one == OpResult::Waiting {
                return nb_send_one;
            };
        }
    }
}

impl FromStr for DuetTablet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<DuetOp> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(DuetTablet { ops })
    }
}
