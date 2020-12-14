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
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let mut memory = HashMap::<usize, usize>::new();

    for o in ops {
        match o {
            Ops::Mask(n) => {
                mask = n.to_string();
            }
            Ops::Mem(slot, value) => {
                memory.insert(*slot, apply_mask(value, &mask));
            }
        }
    }
    let x: usize = memory.iter().fold(0, |acc, (_k, v)| acc + v);
    x
}
#[aoc(day14, part2)]
fn part2(ops: &[Ops]) -> usize {
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let mut memory = HashMap::<usize, usize>::new();

    for o in ops {
        match o {
            Ops::Mask(n) => {
                mask = n.to_string();
            }
            Ops::Mem(slot, value) => {
                for mem_loc in apply_mask_v2(slot, &mask) {
                    memory.insert(mem_loc, *value);
                }
            }
        }
    }

    let x: usize = memory.iter().fold(0, |acc, (_m, v)| acc + v);
    x
}

fn apply_mask(value: &usize, mask: &String) -> usize {
    let frm = format!("{:036b}", value);
    let maskarr: Vec<char> = mask.chars().collect();

    let mut tor = String::with_capacity(frm.capacity());

    for (i, c) in frm.chars().enumerate() {
        match maskarr[i] {
            '1' => tor.push('1'),
            '0' => tor.push('0'),
            _ => tor.push(c),
        }
    }

    usize::from_str_radix(&tor, 2).unwrap()
}

fn apply_mask_v2(value: &usize, mask: &String) -> Vec<usize> {
    //We need to write the value to many addresses
    let frm = format!("{:036b}", value);
    let maskarr: Vec<char> = mask.chars().collect();

    let mut new_mask = String::with_capacity(frm.capacity());

    for (i, c) in frm.chars().enumerate() {
        match maskarr[i] {
            '1' => new_mask.push('1'),
            'X' => new_mask.push('X'),
            _ => new_mask.push(c),
        }
    }

    let mut memory_locations: Vec<String> = Vec::new();
    memory_locations.push("".to_string());

    for x in new_mask.chars() {
        match x {
            'X' => {
                let mut new_ml: Vec<String> = Vec::new();

                for s in memory_locations {
                    let s0 = s.to_string() + "0";
                    new_ml.push(s0);
                    let s1 = s.to_string() + "1";
                    new_ml.push(s1);
                }
                memory_locations = new_ml.clone();
            }
            _ => {
                for i in 0..memory_locations.len() {
                    if let Some(elem) = memory_locations.get_mut(i) {
                        elem.push(x);
                    }
                }
            }
        }
    }

    memory_locations
        .iter()
        .map(|m| usize::from_str_radix(m, 2).unwrap())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_example_1() {
        let mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(73, apply_mask(&11_usize, &mask));
    }
    #[test]
    fn mask_example_2() {
        let mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(101, apply_mask(&101_usize, &mask));
    }
    #[test]
    fn mask_example_3() {
        let mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(64, apply_mask(&0_usize, &mask));
    }

    #[test]
    fn mask2_example1() {
        let mask = String::from("000000000000000000000000000000X1001X");
        let res = apply_mask_v2(&42_usize, &mask);

        assert_eq!(res, [26, 27, 58, 59]);
    }

    #[test]
    fn mask2_example2() {
        let mask = String::from("00000000000000000000000000000000X0XX");
        let res = apply_mask_v2(&26_usize, &mask);

        assert_eq!(res, [16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
