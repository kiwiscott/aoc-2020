use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

const N: i32 = 0;
const E: i32 = 90;
const S: i32 = 180;
const W: i32 = 270;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let (d, v) = l.split_at(1);
            (char::from_str(d).unwrap(), v.parse::<i32>().unwrap())
        })
        .collect()
}

fn direction_from_str(s: char) -> i32 {
    match s {
        'N' => N,
        'E' => E,
        'S' => S,
        _ => W,
    }
}

#[aoc(day12, part1)]
fn part1(data: &Vec<(char, i32)>) -> i32 {
    let mut movements = HashMap::new();
    movements.insert(N, 0_i32);
    movements.insert(E, 0_i32);
    movements.insert(S, 0_i32);
    movements.insert(W, 0_i32);

    let mut current_direction = E;

    for (direction, value) in data {
        match direction {
            'F' => {
                if let Some(x) = movements.get_mut(&current_direction) {
                    *x += value;
                }
            }
            'N' | 'E' | 'W' | 'S' => {
                let d = direction_from_str(*direction);

                if let Some(x) = movements.get_mut(&d) {
                    *x += value;
                }
            }
            'R' => {
                current_direction = (current_direction + value) % 360;
            }
            'L' => {
                current_direction = (current_direction + (360 - value)) % 360;
            }
            _ => {
                panic!("Opps")
            }
        }
    }

    let ns: i32 = movements[&N] - movements[&S];
    let ew: i32 = movements[&E] - movements[&W];

    println!(
        "Problem 1. North-South [{:?}] East-West [{:?}] Manhattan Distance [{:?}]",
        ns,
        ew,
        ns.abs() + ew.abs()
    );
    ns.abs() + ew.abs()
}
#[aoc(day12, part2)]
fn part2(data: &Vec<(char, i32)>) -> i32 {
    //10 units east and 1 unit north
    let mut waypoint = (1_i32, 10_i32, 0_i32, 0_i32);
    let mut ship = (0_i32, 0_i32);

    for (direction, value) in data {
        match direction {
            'N' => waypoint.0 += value,
            'E' => waypoint.1 += value,
            'S' => waypoint.2 += value,
            'W' => waypoint.3 += value,
            'R' => {
                let mut v = *value;
                while v > 0 {
                    waypoint = (waypoint.3, waypoint.0, waypoint.1, waypoint.2);
                    v += -90;
                }
            }
            'L' => {
                let mut v = *value;
                while v > 0 {
                    waypoint = (waypoint.1, waypoint.2, waypoint.3, waypoint.0);
                    v += -90;
                }
            }
            'F' => {
                ship.0 += value * (waypoint.0 - waypoint.2);
                ship.1 += value * (waypoint.1 - waypoint.3);
            }
            _ => {
                panic!("Opps")
            }
        }
        //println!("[{:?}:{:?}]Waypoint : {:?} Ship  : {:?}",direction,value, waypoint, ship);
    }

    println!(
        "Problem 2. Waypoint:[{:?}] Ship:[{:?}] Result:{:?}",
        waypoint,
        ship,
        ship.0.abs() + ship.1.abs()
    );

    ship.0.abs() + ship.1.abs()
}
