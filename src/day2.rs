#![allow(dead_code)]

use crate::intcode::run_intcode_noun_verb;

pub fn solve_day_2() -> i32 {
    run_intcode_noun_verb(&"src/day2_input.txt".to_string(), 12, 2)
}

pub fn solve_day_2_part2() -> (i32, i32) {
    let input_path = "src/day2_input.txt".to_string();
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_intcode_noun_verb(&input_path, noun, verb) == 19690720 {
                return (noun, verb);
            }
        }
    }
    return (0, 0);
}
