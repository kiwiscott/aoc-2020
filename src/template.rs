/*
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Ops {
    Mask(String),
    Mem(usize, usize),
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Ops> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<op>\w+)(\[(?P<addr>\d+)\])? = (?P<value>[\dX]+)$").unwrap();
    }

    input
        .lines()
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            match captures.name("op").unwrap().as_str() {
                "mask" => Ops::Mask(captures.name("value").unwrap().as_str().to_string()),
                "mem" => Ops::Mem(
                    captures.name("addr").unwrap().as_str().parse().unwrap(),
                    captures.name("value").unwrap().as_str().parse().unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(ops: &[Ops]) -> usize {
    0
}
#[aoc(day14, part2)]
fn part2(ops: &[Ops]) -> usize {
    0
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input("").unwrap()[..]), 58);
    }

}

*/
