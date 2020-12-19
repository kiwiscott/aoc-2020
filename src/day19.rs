#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day19, part1)]
fn part1(_: &Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_test() {
        assert_eq!(true, false);
    }
}
