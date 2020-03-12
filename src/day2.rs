#![allow(dead_code, clippy::unreadable_literal)]

use crate::intcode::run_intcode_noun_verb;
const INPUT_PATH: &str = "src/day2_input.txt";

pub fn solve_day_2() -> i32 {
    run_intcode_noun_verb(INPUT_PATH, 12, 2)
}

pub fn solve_day_2_part2() -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_intcode_noun_verb(INPUT_PATH, noun, verb) == 19690720 {
                return (noun, verb);
            }
        }
    }
    (0, 0)
}
