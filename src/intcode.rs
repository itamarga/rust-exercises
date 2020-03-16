#![allow(dead_code, unused_imports, clippy::unreadable_literal)]

const STEP_LIMIT: u32 = 1000000;
pub struct IntcodeMachine {
    intcodes: Vec<i64>,
    pos: usize,
    rel: i64,
}

#[derive(PartialEq)]
pub enum IntcodeReturns {
    Halt,
    NoVal,
    Val(i64),
    RequestIn,
}

impl IntcodeMachine {
    pub fn step(&mut self, arg: Option<i64>) -> IntcodeReturns {
        use IntcodeReturns::*;
        let opcode = self.intcodes[self.pos];
        let action = opcode % 100;
        match action {
            1 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10);
                self.intcodes[target_pos] = self.intcodes[src1_pos] + self.intcodes[src2_pos];
                self.pos += 4;
                NoVal
            }
            2 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10);
                self.intcodes[target_pos] = self.intcodes[src1_pos] * self.intcodes[src2_pos];
                self.pos += 4;
                NoVal
            }
            3 => {
                if let Some(param) = arg {
                    let target_pos = self.get_pos(1, opcode / 100);
                    self.intcodes[target_pos] = param;
                    self.pos += 2;
                    NoVal
                } else {
                    RequestIn
                }
            }
            4 => {
                let src_pos = self.get_pos(1, opcode / 100);
                self.pos += 2;
                Val(self.intcodes[src_pos])
            }
            5 => {
                let bool_pos = self.get_pos(1, (opcode / 100) % 10);
                let target_pos = self.get_pos(2, (opcode / 1000) % 10);
                if self.intcodes[bool_pos] != 0 {
                    self.pos = self.intcodes[target_pos] as usize;
                } else {
                    self.pos += 3;
                }
                NoVal
            }
            6 => {
                let bool_pos = self.get_pos(1, (opcode / 100) % 10);
                let target_pos = self.get_pos(2, (opcode / 1000) % 10);
                if self.intcodes[bool_pos] == 0 {
                    self.pos = self.intcodes[target_pos] as usize;
                } else {
                    self.pos += 3;
                }
                NoVal
            }
            7 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10);
                if self.intcodes[src1_pos] < self.intcodes[src2_pos] {
                    self.intcodes[target_pos] = 1;
                } else {
                    self.intcodes[target_pos] = 0;
                }
                self.pos += 4;
                NoVal
            }
            8 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10);
                if self.intcodes[src1_pos] == self.intcodes[src2_pos] {
                    self.intcodes[target_pos] = 1;
                } else {
                    self.intcodes[target_pos] = 0;
                }
                self.pos += 4;
                NoVal
            }
            9 => {
                let src_pos = self.get_pos(1, (opcode / 100) % 10);
                self.rel += self.intcodes[src_pos];
                self.pos += 2;
                NoVal
            }
            99 => Halt,
            _ => panic!("Bad code!"),
        }
    }

    fn get_pos(&mut self, arg_num: usize, mode: i64) -> usize {
        let idx = match mode {
            0 => self.intcodes[self.pos + arg_num] as usize,
            1 => self.pos + arg_num as usize,
            2 => (self.rel + self.intcodes[self.pos + arg_num]) as usize,
            _ => panic!("Bad code!"),
        };
        if idx >= self.intcodes.len() {
            self.intcodes.resize(idx + 1, 0);
        }
        idx
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.rel = 0;
    }

    pub fn run(&mut self, opt: Option<i64>) -> IntcodeReturns {
        for _ in 0..STEP_LIMIT {
            let ret = self.step(opt);
            match ret {
                IntcodeReturns::NoVal => continue,
                x => return x,
            }
        }
        panic!("Step limit reached!");
    }
}

impl From<Vec<i32>> for IntcodeMachine {
    fn from(vec: Vec<i32>) -> Self {
        IntcodeMachine {
            intcodes: vec.iter().map(|m| *m as i64).collect(),
            pos: 0,
            rel: 0,
        }
    }
}

impl From<&String> for IntcodeMachine {
    fn from(intcodes_str: &String) -> Self {
        IntcodeMachine {
            intcodes: parse_intcode(intcodes_str),
            pos: 0,
            rel: 0,
        }
    }
}

impl From<&str> for IntcodeMachine {
    fn from(intcodes_str: &str) -> Self {
        IntcodeMachine {
            intcodes: parse_intcode(intcodes_str),
            pos: 0,
            rel: 0,
        }
    }
}

pub fn run_intcode_noun_verb(intcode_path: &str, noun: i64, verb: i64) -> i64 {
    let input = std::fs::read_to_string(intcode_path).unwrap();
    let mut intcode = parse_intcode(&input);
    intcode[1] = noun;
    intcode[2] = verb;
    process_intcode(&mut intcode);
    intcode[0]
}

fn parse_intcode(intcode_str: &str) -> Vec<i64> {
    intcode_str
        .split(',')
        .map(|int| int.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn process_intcode(intcode: &mut Vec<i64>) {
    let mut idx = 0;
    while idx < intcode.len() {
        idx = match intcode[idx] {
            1 => add(intcode, idx),
            2 => mult(intcode, idx),
            99 => return,
            _ => panic!("Bad code!"),
        };
    }
}

fn add(intcode: &mut Vec<i64>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] + intcode[b_idx];
    idx + 4
}
fn mult(intcode: &mut Vec<i64>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] * intcode[b_idx];
    idx + 4
}

mod tests {
    use super::*;
    #[test]
    fn is_eight() {
        {
            let mut machine = IntcodeMachine::from("3,9,8,9,10,9,4,9,99,-1,8");
            if let IntcodeReturns::Val(result) = machine.run(Some(8)) {
                assert_eq!(result, 1);
            } else {
                panic!("Unexpected IntcodeResult");
            }
        }
        {
            let mut machine = IntcodeMachine::from("3,9,8,9,10,9,4,9,99,-1,8");
            if let IntcodeReturns::Val(result) = machine.run(Some(5)) {
                assert_eq!(result, 0);
            } else {
                panic!("Unexpected IntcodeResult");
            }
        }
    }
    #[test]
    fn quine() {
        let mut machine =
            IntcodeMachine::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        if let IntcodeReturns::Val(result) = machine.run(None) {
            println!("{}", result);
            assert_eq!(result, 109);
        }
    }
}
