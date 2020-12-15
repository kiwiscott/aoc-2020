use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

const DEVICE_TO_HIGHEST_JOLTAGE_GAP: i32 = 3;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<i64> {
    let mut jolt_adapters: Vec<i64> = input.lines().map(|l| l.parse::<i64>().unwrap()).collect();

    //add zero to the start
    jolt_adapters.push(0);
    jolt_adapters.sort();

    jolt_adapters
}

#[aoc(day10, part1)]
fn part1(jolt_adapters: &Vec<i64>) -> i64 {
    let p1: HashMap<i32, i32> = jolt_adapters
        .iter()
        .enumerate()
        //Find the difference to the next item
        .map(|(index, joltage)| match jolt_adapters.get(index + 1) {
            Some(next_joltage) => (next_joltage - joltage) as i32,
            None => DEVICE_TO_HIGHEST_JOLTAGE_GAP, //Last Row is always + 3,
        })
        //HashMap of diffrences and the count of those differences
        .fold(HashMap::<i32, i32>::new(), |mut acc, a| {
            *acc.entry(a).or_insert(0) += 1;
            acc
        });

    println!(
        "p1. Product of one volt jumps [{:?}], three volts jumps [{:?}] is [{:?}]",
        p1.get(&1).unwrap(),
        p1.get(&3).unwrap(),
        p1.get(&1).unwrap() * p1.get(&3).unwrap()
    );
    (p1.get(&1).unwrap() * p1.get(&3).unwrap()) as i64
}
#[aoc(day10, part2)]
fn part2(jolt_adapters: &Vec<i64>) -> i64 {
    let mut jolt_adapters = jolt_adapters.clone();
    jolt_adapters.sort();
    //Get the last value
    let device_joltage = (*jolt_adapters.last().unwrap()) + (DEVICE_TO_HIGHEST_JOLTAGE_GAP as i64);
    //Reverse for Bottom Up Walk
    jolt_adapters.reverse();

    //74049191673856
    let x = match bottom_up_tree_walk(jolt_adapters, device_joltage) {
        Some(n) => {
            println!("p2. All Possible combinations [{:?}]", n);
            n
        }
        None => {
            println!("p2, Not Solved!");
            0
        }
    };
    x
}

fn bottom_up_tree_walk(jolt_adapters: Vec<i64>, device_joltage: i64) -> Option<i64> {
    let mut cache = HashMap::<i64, i64>::new();
    let possible_jolt_upgrades = [1, 2, 3];

    //There is only one end device
    cache.insert(device_joltage, 1);
    //Reverse for Caching

    //We walk the numbers in reverse and create a cache for each of the sub paths.
    //We do that so we can lookup the cache to sum the nodes of the sub paths.
    for adapter_joltage in jolt_adapters {
        let mut sub_paths = 0;

        for i in possible_jolt_upgrades.iter() {
            match cache.get(&(adapter_joltage + i)) {
                Some(n) => sub_paths = sub_paths + n,
                None => (),
            }
        }
        cache.insert(adapter_joltage, sub_paths);
        //println!("{:?}-{:?}", number, sub_paths);
    }
    match cache.get(&0) {
        Some(n) => Some(*n),
        None => None,
    }
}
