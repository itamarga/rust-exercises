#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_day_1() -> Result<u32, std::io::Error> {
    let input = File::open("src/day1_input.txt")?;
    let mut total_fuel = 0;

    let reader = BufReader::new(input);
    for line in reader.lines() {
        let line = line.unwrap();
        let mass = line.parse::<u32>().unwrap();
        total_fuel += calc_fuel(mass);
    }
    Ok(total_fuel)
}

pub fn calc_fuel(mass: u32) -> u32 {
    let mut fuel = mass as i32 / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel += calc_fuel(fuel as u32) as i32;
        fuel as u32
    }
}
