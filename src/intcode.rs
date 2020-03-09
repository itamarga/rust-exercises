pub struct IntcodeMachine {
    intcodes: Vec<i32>,
    pos: usize,
}

#[derive(PartialEq)]
pub enum IntcodeReturns {
    Halt,
    NoVal,
    Val(i32),
}

impl IntcodeMachine {
    pub fn step(&mut self, arg: Option<i32>) -> IntcodeReturns {
        use IntcodeReturns::*;
        let opcode = self.intcodes[self.pos];
        let action = opcode % 100;
        match action {
            1 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10 == 0);
                self.intcodes[target_pos] = self.intcodes[src1_pos] + self.intcodes[src2_pos];
                self.pos += 4;
                return NoVal;
            }
            2 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10 == 0);
                self.intcodes[target_pos] = self.intcodes[src1_pos] * self.intcodes[src2_pos];
                self.pos += 4;
                return NoVal;
            }
            3 => {
                let param = arg.unwrap();
                let target_pos = self.get_pos(1, opcode / 100 == 0);
                self.intcodes[target_pos] = param;
                self.pos += 2;
                return NoVal;
            }
            4 => {
                let src_pos = self.get_pos(1, opcode / 100 == 0);
                self.pos += 2;
                return Val(self.intcodes[src_pos]);
            }
            5 => {
                let bool_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let target_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                if self.intcodes[bool_pos] != 0 {
                    self.pos = self.intcodes[target_pos] as usize;
                } else {
                    self.pos += 3;
                }
                return NoVal;
            }
            6 => {
                let bool_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let target_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                if self.intcodes[bool_pos] == 0 {
                    self.pos = self.intcodes[target_pos] as usize;
                } else {
                    self.pos += 3;
                }
                return NoVal;
            }
            7 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10 == 0);
                if self.intcodes[src1_pos] < self.intcodes[src2_pos] {
                    self.intcodes[target_pos] = 1;
                } else {
                    self.intcodes[target_pos] = 0;
                }
                self.pos += 4;
                return NoVal;
            }
            8 => {
                let src1_pos = self.get_pos(1, (opcode / 100) % 10 == 0);
                let src2_pos = self.get_pos(2, (opcode / 1000) % 10 == 0);
                let target_pos = self.get_pos(3, (opcode / 10000) % 10 == 0);
                if self.intcodes[src1_pos] == self.intcodes[src2_pos] {
                    self.intcodes[target_pos] = 1;
                } else {
                    self.intcodes[target_pos] = 0;
                }
                self.pos += 4;
                return NoVal;
            }
            99 => return Halt,
            _ => panic!("Bad code!"),
        }
    }

    fn get_pos(&self, arg_num: usize, indirect: bool) -> usize {
        if indirect {
            self.intcodes[self.pos + arg_num] as usize
        } else {
            self.pos + arg_num as usize
        }
    }
}

impl From<Vec<i32>> for IntcodeMachine {
    fn from(vec: Vec<i32>) -> Self {
        IntcodeMachine {
            intcodes: vec,
            pos: 0,
        }
    }
}

impl From<&String> for IntcodeMachine {
    fn from(intcodes_str: &String) -> Self {
        IntcodeMachine {
            intcodes: parse_intcode(intcodes_str),
            pos: 0,
        }
    }
}

pub fn run_intcode_noun_verb(intcode_path: &String, noun: i32, verb: i32) -> i32 {
    let input = std::fs::read_to_string(intcode_path).unwrap();
    let mut intcode = parse_intcode(&input);
    intcode[1] = noun;
    intcode[2] = verb;
    process_intcode(&mut intcode);
    intcode[0]
}

fn parse_intcode(intcode_str: &String) -> Vec<i32> {
    intcode_str
        .split(",")
        .map(|int| int.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn process_intcode(intcode: &mut Vec<i32>) {
    let mut idx = 0;
    while idx < intcode.len() {
        idx = match intcode[idx] {
            1 => add(intcode, idx),
            2 => mult(intcode, idx),
            99 => return,
            _ => panic!("Bad code!"),
        };
    }
    ()
}

fn add(intcode: &mut Vec<i32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] + intcode[b_idx];
    return idx + 4;
}
fn mult(intcode: &mut Vec<i32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] * intcode[b_idx];
    return idx + 4;
}
