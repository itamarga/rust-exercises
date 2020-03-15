use image::*;
const INTPUT_PATH: &str = "src/day8_input.txt";
const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;

pub fn solve_day_8_pt_1() -> u32 {
    let input = std::fs::read_to_string(INTPUT_PATH).unwrap();
    let mut idx = 0;
    let mut final_idx = 0;
    let mut min_zeros = std::u32::MAX;
    while idx < input.len() {
        let zeros = count_char(&input[idx..idx + IMG_WIDTH * IMG_HEIGHT], '0');
        if zeros < min_zeros {
            min_zeros = zeros;
            final_idx = idx;
        }
        idx += IMG_WIDTH * IMG_HEIGHT;
    }
    let ones = count_char(&input[final_idx..final_idx + IMG_WIDTH * IMG_HEIGHT], '1');
    let twos = count_char(&input[final_idx..final_idx + IMG_WIDTH * IMG_HEIGHT], '2');
    ones * twos
}

fn count_char(segment: &str, c: char) -> u32 {
    let mut zero_count = 0;
    for char in segment.chars() {
        if char == c {
            zero_count += 1;
        }
    }
    zero_count
}

pub fn solve_day_8_pt_2() -> u32 {
    0
}
