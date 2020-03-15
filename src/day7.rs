use crate::intcode::*;
use rayon::prelude::*;

const INPUT: &str = "3,8,1001,8,10,8,105,1,0,0,21,38,47,72,97,122,203,284,365,446,99999,3,9,1001,9,3,9,1002,9,5,9,1001,9,4,9,4,9,99,3,9,102,3,9,9,4,9,99,3,9,1001,9,2,9,102,5,9,9,101,3,9,9,1002,9,5,9,101,4,9,9,4,9,99,3,9,101,5,9,9,1002,9,3,9,101,2,9,9,102,3,9,9,1001,9,2,9,4,9,99,3,9,101,3,9,9,102,2,9,9,1001,9,4,9,1002,9,2,9,101,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,99";

pub fn solve_day_7_pt_1() -> i64 {
    permutations([0, 1, 2, 3, 4])
        .par_iter()
        .map(|permutation| run_amplifier(permutation))
        .max()
        .unwrap()
}

fn run_amplifier(phase_settings: &[u8]) -> i64 {
    let mut machine = IntcodeMachine::from(INPUT);
    let mut ret = 0;
    for phase in phase_settings.iter() {
        machine.step(Some(*phase as i64));

        loop {
            if let IntcodeReturns::Val(result) = machine.step(Some(ret)) {
                {
                    ret = result;
                    machine.reset();
                    break;
                };
            }
        }
    }
    ret
}

pub fn solve_day_7_pt_2() -> i64 {
    permutations([5, 6, 7, 8, 9])
        .par_iter()
        .map(|permutation| run_amplifier_feedback(permutation))
        .max()
        .unwrap()
}

fn run_amplifier_feedback(phase_settings: &[u8]) -> i64 {
    let mut machines = [
        IntcodeMachine::from(INPUT),
        IntcodeMachine::from(INPUT),
        IntcodeMachine::from(INPUT),
        IntcodeMachine::from(INPUT),
        IntcodeMachine::from(INPUT),
    ];
    for (idx, phase) in phase_settings.iter().enumerate() {
        machines[idx].step(Some(*phase as i64));
    }
    let mut ret = 0;
    loop {
        for idx in 0..5 {
            'inner: loop {
                match machines[idx].step(Some(ret)) {
                    IntcodeReturns::Val(result) => {
                        ret = result;
                        break 'inner;
                    }
                    IntcodeReturns::Halt => {
                        return ret;
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn permutations(a: [u8; 5]) -> Vec<Vec<u8>> {
    let mut partial = vec![];
    partial.push(vec![a[0]]);
    for mem in a.iter().skip(1) {
        for j in (0..partial.len()).rev() {
            let curr = partial.remove(j);
            for k in 0..=curr.len() {
                let mut curr = curr.clone();
                curr.insert(k, *mem);
                partial.push(curr);
            }
        }
    }
    partial
}
