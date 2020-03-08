pub fn solve_day_4() -> u32 {
    let start = 271973;
    let end = 785961;
    let mut count = 0;
    for num in start..=end {
        if is_valid_pass(num) {
            count += 1;
        }
    }
    count
}

fn is_valid_pass(num: u32) -> bool {
    return has_double_digits(num) && has_rising_digits(num);
}

fn has_double_digits(num: u32) -> bool {
    true
}

fn has_rising_digits(num: u32) -> bool {
    true
}
