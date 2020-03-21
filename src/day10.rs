extern crate num;
use num::integer::gcd;
use std::cmp::Ordering;
use std::collections;
// const INPUT: &str = ".#....#####...#..
// ##...##.#####..##
// ##...#...#.#####.
// ..#.....#...###..
// ..#.#.....#....##";

// Example
// const INPUT: &str = ".#..##.###...#######
// ##.############..##.
// .#.######.########.#
// .###.#######.####.#.
// #####.##.#.##.###.##
// ..#####..#.#########
// ####################
// #.####....###.#.#.##
// ##.#################
// #####.##.###..####..
// ..######..##.#######
// ####.##.####...##..#
// .#####..#.######.###
// ##...#.##########...
// #.##########.#######
// .####.#.###.###.#.##
// ....##.##.###..#####
// .#.#.###########.###
// #.#.#.#####.####.###
// ###.##.####.##.#..##";

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
.#.#####.##...#...#.##...#.
";

pub fn solve_day_10_pt1() {
    let asteroids = parse_input(INPUT);
    let answer = find_station_location(&asteroids);
    println!(
        "asteroid {:?} can see the most, seeing {}",
        answer.0,
        answer.1.len()
    );
}

fn find_station_location(asteroids: &Vec<(i32, i32)>) -> ((i32, i32), Vec<(i32, i32)>) {
    let answer = asteroids
        .iter()
        .map(|asteroid| (asteroid, calc_seen_asteroids(asteroid, &asteroids)))
        .max_by(|(_, m1), (_, m2)| m1.len().cmp(&m2.len()))
        .unwrap();

    (answer.0.clone(), answer.1)
}

pub fn solve_day_10_pt2() {
    let mut asteroids = parse_input(INPUT);
    let total_asteroids = asteroids.len();
    let (station, mut seen) = find_station_location(&asteroids);
    // println!("Station at {:?}", station);
    let sort_fn = |asteroid1: &(i32, i32), asteroid2: &(i32, i32)| {
        let dx1 = asteroid1.0 - station.0;
        let dy1 = asteroid1.1 - station.1;
        let dx2 = asteroid2.0 - station.0;
        let dy2 = asteroid2.1 - station.1;
        if dx1 == 0 {
            if dy1 < 0 {
                Ordering::Greater
            } else {
                if dx2 <= 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        } else if dx2 == 0 {
            if dy2 < 0 {
                Ordering::Less
            } else {
                if dx1 <= 0 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
        // nither is zero
        else {
            let tan1 = dy1 as f32 / dx1 as f32;
            let tan2 = dy2 as f32 / dx2 as f32;

            if dx1 < 0 {
                if dx2 > 0 {
                    Ordering::Less
                } else {
                    tan2.partial_cmp(&tan1).unwrap_or(Ordering::Equal)
                }
            } else {
                //dx1 > 0
                if dx2 < 0 {
                    Ordering::Greater
                } else {
                    tan2.partial_cmp(&tan1).unwrap_or(Ordering::Equal)
                }
            }
        }
    };
    seen.sort_by(sort_fn);
    asteroids = remove_seen(asteroids, &seen);

    for i in 1..total_asteroids {
        let asteroid = seen.pop().unwrap();
        let tan = (asteroid.1 - station.1) as f32 / (asteroid.0 - station.0) as f32;
        if i == 200 {
            println!("{}: {:?} with tan {}", i, asteroid, tan);
        }
        if seen.len() == 0 {
            // println!("++++++++++++++++recalc seen+++++++++++++++++");
            seen = calc_seen_asteroids(&station, &asteroids);
            seen.sort_by(sort_fn);
            asteroids = remove_seen(asteroids, &seen);
        }
    }
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

fn remove_seen(asteroids: Vec<(i32, i32)>, seen: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    asteroids
        .into_iter()
        .filter(|asteroid| false == seen.contains(asteroid))
        .collect()
}

fn calc_seen_asteroids(asteroid: &(i32, i32), asteroids: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
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
    seen_list
}

#[test]
fn test_deg() {
    let mut asteroids = parse_input(INPUT);
    let (station, mut seen) = find_station_location(&asteroids);
    let mut asteroids: Vec<((i32, i32), f32)> = asteroids
        .into_iter()
        .map(|asteroid| {
            let dx = asteroid.0 - station.0;
            let dy = asteroid.1 - station.1;
            let deg = (dy as f32 / dx as f32).atan();
            (asteroid, deg)
        })
        .collect();
    asteroids.sort_by(|(_, deg1), (_, deg2)| deg1.partial_cmp(deg2).unwrap_or(Ordering::Equal));
    println!("Station at {:?}", station);
    for (asteroid, deg) in asteroids {
        println!("{:?} : {}", asteroid, deg);
    }
}
