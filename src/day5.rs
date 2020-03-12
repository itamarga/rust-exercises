#![allow(dead_code)]

use crate::intcode::*;
const INTCODE_PATH: &str = "src/day5_input.txt";

pub fn solve_day_5_pt1() -> i32 {
    let input = std::fs::read_to_string(INTCODE_PATH).unwrap();
    let mut machine = IntcodeMachine::from(&input);
    let mut ret = machine.step(Some(1));
    //let mut count = 1;
    let mut final_result = 0;
    while ret != IntcodeReturns::Halt {
        ret = machine.step(None);
        if let IntcodeReturns::Val(result) = ret {
            //println!("Test {} returned {}", count, result);
            final_result = result;
            //count += 1;
        }
    }
    final_result
}

pub fn solve_day_5_pt2() -> i32 {
    let input = std::fs::read_to_string(INTCODE_PATH).unwrap();
    let mut machine = IntcodeMachine::from(&input);
    let mut ret = machine.step(Some(5));
    // let mut count = 1;
    let mut final_result = 0;
    while ret != IntcodeReturns::Halt {
        ret = machine.step(None);
        if let IntcodeReturns::Val(result) = ret {
            // println!("Test {} returned {}", count, result);
            final_result = result;
            // count += 1;
        }
    }
    final_result
}
