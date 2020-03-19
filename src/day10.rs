extern crate num;
use num::integer::gcd;
use std::collections;
const INPUT: &str = ".###..#######..####..##...#
########.#.###...###.#....#
###..#...#######...#..####.
.##.#.....#....##.#.#.....#
###.#######.###..##......#.
#..###..###.##.#.#####....#
#.##..###....#####...##.##.
####.##..#...#####.#..###.#
#..#....####.####.###.#.###
#..#..#....###...#####..#..
##...####.######....#.####.
####.##...###.####..##....#
#.#..#.###.#.##.####..#...#
..##..##....#.#..##..#.#..#
##.##.#..######.#..#..####.
#.....#####.##........#####
###.#.#######..#.#.##..#..#
###...#..#.#..##.##..#####.
.##.#..#...#####.###.##.##.
...#.#.######.#####.#.####.
#..##..###...###.#.#..#.#.#
.#..#.#......#.###...###..#
#.##.#.#..#.#......#..#..##
.##.##.##.#...##.##.##.#..#
#.###.#.#...##..#####.###.#
#.####.#..#.#.##.######.#..
.#.#####.##...#...#.##...#.";

pub fn solve_day_10_p1() {
    let asteroids = parse_input(INPUT);
    let answer = asteroids
        .iter()
        .max_by_key(|asteroid| calc_seen_asteroids(asteroid, &asteroids))
        .unwrap();
    println!(
        "asteroid {:?} can see the most, seeing {}",
        answer,
        calc_seen_asteroids(answer, &asteroids)
    );
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                ret.push((x as i32, y as i32));
            }
        }
    }
    ret
}

fn calc_seen_asteroids(asteroid: &(i32, i32), asteroids: &Vec<(i32, i32)>) -> usize {
    let &(xa, ya) = asteroid;
    let mut list = asteroids.clone();
    list.sort_by_key(|&(xb, yb)| (xb - xa).abs() + (yb - ya).abs());
    let mut has_seen_above = false;
    let mut has_seen_below = false;
    let mut directions_seen = collections::HashSet::<(i32, i32)>::new();
    let mut seen_list = Vec::<(i32, i32)>::new();
    // abstarget = absasteroid + reltarget
    // reltarget = abstarget - absasteroid
    for (xb, yb) in list {
        if xa == xb {
            if ya == yb {
                continue; // ignore self
            } else if ya < yb && has_seen_above == false {
                seen_list.push((xb, yb));
                has_seen_above = true
            } else if ya > yb && has_seen_below == false {
                seen_list.push((xb, yb));
                has_seen_below = true
            }
        } else {
            let dx = xb - xa;
            let dy = yb - ya;
            let gcd = gcd(dx, dy);
            if directions_seen.contains(&(dx / gcd, dy / gcd)) == false {
                seen_list.push((xb, yb));
                directions_seen.insert((dx / gcd, dy / gcd));
            }
        }
    }
    // println!(
    //     "{:?} can see: {:?} total:{:?}",
    //     (xa, ya),
    //     seen_list,
    //     seen_list.len()
    // );
    seen_list.len()
}
