#[allow(clippy::unreadable_literal)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
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
    let (first, second) = day4::solve_day_4_pt1();
    println!("Day 4 pt1 solution is: {}. Pt2 is: {}", first, second);
    println!("Day 5 pt1 solution is: {}.", day5::solve_day_5_pt1());
    println!("Day 5 pt2 solution is: {}.", day5::solve_day_5_pt2());
    println!("Day 6 pt 1 solution is: {}.", day6::solve_day_6_pt_1());
    println!("Day 6 pt 2 solution is: {}.", day6::solve_day_6_pt_2());
}
