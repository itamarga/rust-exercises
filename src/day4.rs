#![allow(dead_code)]
#[allow(clippy::unreadable_literal)]

pub fn solve_day_4_pt1() -> (u32, u32) {
    let start = 271973;
    let end = 785961;
    let mut count = 0;
    let mut count2 = 0;
    for num in start..=end {
        if is_valid_pass_pt1(num) {
            count += 1;
        }
        if is_valid_pass_pt2(num) {
            count2 += 1;
        }
    }
    (count, count2)
}

fn is_valid_pass_pt1(num: u32) -> bool {
    has_double_digits(num) && has_rising_digits(num)
}

fn is_valid_pass_pt2(num: u32) -> bool {
    has_double_digits_isolated(num) && has_rising_digits(num)
}

fn has_double_digits(mut num: u32) -> bool {
    let mut last_digit = 10;
    while num > 0 {
        let digit = num % 10;
        if digit == last_digit {
            return true;
        }
        num /= 10;
        last_digit = digit
    }
    false
}

fn has_double_digits_isolated(mut num: u32) -> bool {
    let mut last_digit = 10;
    while num > 0 {
        let digit = num % 10;
        if digit == last_digit {
            if digit != (num / 10) % 10 {
                return true;
            } else {
                while num % 10 == digit {
                    num /= 10;
                }
                continue;
            }
        }
        num /= 10;
        last_digit = digit;
    }
    false
}

fn has_rising_digits(mut num: u32) -> bool {
    let mut last_digit = 10;
    while num > 0 {
        let digit = num % 10;
        if digit > last_digit {
            return false;
        }
        num /= 10;
        last_digit = digit;
    }
    true
}
