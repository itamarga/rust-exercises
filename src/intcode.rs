pub fn run_intcode(intcode_path: &String, noun: i32, verb: i32) -> i32 {
    let input = std::fs::read_to_string(intcode_path).unwrap();
    let mut intcode = parse_intcode(&input);
    intcode[1] = noun;
    intcode[2] = verb;
    process_intcode(&mut intcode);
    intcode[0]
}

fn parse_intcode(intcode_str: &String) -> Vec<i32> {
    intcode_str
        .split(",")
        .map(|int| int.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn process_intcode(intcode: &mut Vec<i32>) {
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

fn add(intcode: &mut Vec<i32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] + intcode[b_idx];
    return idx + 4;
}

fn mult(intcode: &mut Vec<i32>, idx: usize) -> usize {
    let a_idx = intcode[idx + 1] as usize;
    let b_idx = intcode[idx + 2] as usize;
    let target_idx = intcode[idx + 3] as usize;
    intcode[target_idx] = intcode[a_idx] * intcode[b_idx];
    return idx + 4;
}
