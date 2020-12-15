use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::VecDeque;

const PREAMBLE: usize = 25;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(numbers: &Vec<i64>) -> i64 {
    let p1 = match problem1(PREAMBLE, numbers) {
        Some(n) => n,
        None => i64::MIN,
    };
    p1
}

#[aoc(day9, part2)]
fn part2(numbers: &Vec<i64>) -> i64 {
    let p1 = match problem1(PREAMBLE, numbers) {
        Some(n) => n,
        None => panic!("Cant be here"),
    };

    let p2 = match problem2(numbers, p1) {
        Some(n) => n.0 + n.1,
        None => i64::MIN,
    };
    p2
}

fn problem2(numbers: &Vec<i64>, number_to_find: i64) -> Option<(i64, i64)> {
    let mut nums_in_play: VecDeque<i64> = VecDeque::new();
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        match sum.cmp(&number_to_find) {
            Ordering::Less => {
                nums_in_play.push_back(numbers[index]);
                sum = sum + numbers[index];
                index = index + 1;
            }
            Ordering::Greater => {
                sum = sum - nums_in_play.pop_front().unwrap();
            }
            Ordering::Equal => {
                let min = nums_in_play.iter().map(|n| *n).min().unwrap();
                let max = nums_in_play.iter().map(|n| *n).max().unwrap();
                return Some((min, max));
            }
        }
    }
    None
}

fn problem1(preamble_length: usize, numbers: &Vec<i64>) -> Option<i64> {
    //What is the first number that does cannot be represented by a sum of two numbers in the previous 'preamble' numbers?

    //Build to preamble. Rust annoys me a bit here because I could just use a slice
    let mut preamble: VecDeque<i64> = VecDeque::with_capacity(preamble_length);
    numbers
        .iter()
        .take(preamble_length)
        .for_each(|n| preamble.push_back(*n));

    //println!("Preamble {:?}-{:?}",preamble.len(),preamble);

    for n in numbers.iter().skip(preamble_length) {
        let found = preamble.iter().any(|a| preamble.contains(&(n - a)));
        if !found {
            return Some(*n);
        }
        //Take off the Preamble and put on the preamble
        preamble.pop_front();
        preamble.push_back(*n);
    }
    return None;
}
