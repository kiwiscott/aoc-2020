use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut buf = String::new();

    for i in input.lines() {
        match i.len() == 0 {
            true => {
                res.push(buf.trim().to_string());
                buf = String::new();
            }
            false => {
                buf.push_str(i);
                buf.push(' ');
            }
        }
    }
    res.push(buf.trim().to_string());
    res
}

#[aoc(day6, part1)]
fn part1(answers: &Vec<String>) -> usize {
    println!("{:?}", answers);

    answers
        .iter()
        .map(|c| {
            let mut v = HashSet::new();
            c.chars().filter(|p| p != &' ').for_each(|a| {
                v.insert(a);
            });
            if v.len() == 3 {
                println!("{:?} {:?}", v, c);
            }
            v.len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(answers: &Vec<String>) -> usize {
    answers
        .iter()
        .map(|pass| {
            let num_in_group: u8 = pass.split_ascii_whitespace().count().try_into().unwrap();
            (pass, num_in_group)
        })
        .map(|(pass, num_in_group)| {
            pass.chars()
                .filter(|p| p != &' ')
                .fold(HashMap::<char, u8>::new(), |mut acc, a| {
                    *acc.entry(a).or_insert(0) += 1;
                    acc
                })
                .iter()
                .filter(|(_, v)| **v == num_in_group)
                .count()
        })
        .sum()
    //3276
}
