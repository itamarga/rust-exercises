use image::*;
const INTPUT_PATH: &str = "src/day8_input.txt";
const IMG_WIDTH: u32 = 25;
const IMG_HEIGHT: u32 = 6;
const IMG_SIZE: usize = IMG_HEIGHT as usize * IMG_WIDTH as usize;

pub fn solve_day_8_pt_1() -> u32 {
    let input = std::fs::read_to_string(INTPUT_PATH).unwrap();
    let mut idx = 0;
    let mut final_idx = 0;
    let mut min_zeros = std::u32::MAX;
    while idx < input.len() {
        let zeros = count_char(&input[idx..idx + IMG_SIZE], '0');
        if zeros < min_zeros {
            min_zeros = zeros;
            final_idx = idx;
        }
        idx += IMG_SIZE;
    }
    let ones = count_char(&input[final_idx..final_idx + IMG_SIZE], '1');
    let twos = count_char(&input[final_idx..final_idx + IMG_SIZE], '2');
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

pub fn solve_day_8_pt_2() {
    let input = std::fs::read_to_string(INTPUT_PATH).unwrap();
    let mut img = [2; IMG_SIZE];

    for (i, c) in input.chars().enumerate() {
        if img[i % IMG_SIZE] == 2 {
            let mut val = c.to_digit(10).unwrap() as u8;
            if val == 1 {
                val = 255;
            }
            img[i % IMG_SIZE] = val;
        }
    }

    // let encoder = image::bmp::BMPEncoder::new();
    image::save_buffer("day8_pass.png", &img, IMG_WIDTH, IMG_HEIGHT, ColorType::L8);
}
