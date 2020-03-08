mod day1;
mod day2;
mod day3;
mod intcode;

fn main() {
    println!("Day 1 solution is: {}", day1::solve_day_1().unwrap());
    println!("Day 2 solution is: {}", day2::solve_day_2());
    println!("Day 2 part 2 solution is: {:?}", day2::solve_day_2_part2());
    println!("Day 3 solution is: {:?}", day3::find_nearest_intersection());
    println!(
        "Day 3 part 2 solution is: {:?}",
        day3::find_shortest_path_to_intersection()
    );
}
