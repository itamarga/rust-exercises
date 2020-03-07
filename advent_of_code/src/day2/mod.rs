pub fn solve_day_2() -> u32 {
    let example = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut intcode = parse_intcode(example);
    process_intcode(&mut intcode);
    intcode[0]
}

fn parse_intcode(intcode_str: &'static str) -> Vec<u32> {
    intcode_str
        .split(",")
        .map(|int| int.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn process_intcode(intcode: &mut Vec<u32>) {
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

fn add(intcode: &mut Vec<u32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] + intcode[b_idx];
    return idx + 4;
}

fn mult(intcode: &mut Vec<u32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] * intcode[b_idx];
    return idx + 4;
}
