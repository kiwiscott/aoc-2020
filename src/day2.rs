use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

use regex::Regex;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Password> {
    input.lines().map(|s| string_password(s)).collect()
}

#[aoc(day2, part1)]
fn part1(passwords: &Vec<Password>) -> usize {
    passwords
        .iter()
        .map(|p| valid_password_policy_1(&p))
        .filter(|x| *x)
        .count()
}

#[aoc(day2, part2)]
fn part2(passports: &Vec<Password>) -> usize {
    passports
        .iter()
        .map(|p| valid_password_policy_2(&p))
        .filter(|x| *x)
        .count()
}

//8-9 f: fffffffxx
#[derive(Debug)]
pub struct Password {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

pub fn valid_password_policy_2(p: &Password) -> bool {
    /*
    The character p.letter must appear in either the
    p.min and p.max position but not both.
    */
    let char_vec: Vec<char> = p.password.chars().collect();

    let m = (p.min - 1) as usize;
    let mm = (p.max - 1) as usize;

    //Both cannot be true
    if char_vec[m] == p.letter && char_vec[mm] == p.letter {
        return false;
    }
    return char_vec[m] == p.letter || char_vec[mm] == p.letter;
}

pub fn valid_password_policy_1(p: &Password) -> bool {
    /*
    The character p.letter must only appear between p.min and p.max times
    */
    let mut char_map: HashMap<char, i32> = HashMap::new();

    for c in p.password.chars() {
        if char_map.contains_key(&c) {
            let i = 1 + char_map[&c];
            char_map.remove(&c);
            char_map.insert(c, i);
        } else {
            char_map.insert(c, 1);
        }
    }
    return char_map.contains_key(&p.letter)
        && (p.min <= char_map[&p.letter])
        && char_map[&p.letter] <= p.max;
}

pub fn string_password(s: &str) -> Password {
    let re = Regex::new(r"^([\d]*)-([\d]*)\W([a-zA-Z]):\W([a-zA-Z]*)$").unwrap();

    let caps = re.captures(s).unwrap();

    return Password {
        min: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        max: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        letter: caps.get(3).unwrap().as_str().parse::<char>().unwrap(),
        password: caps.get(4).unwrap().as_str().to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    //String to Password
    #[test]
    fn can_take_string_to_password() {
        let p = string_password("1-3 a: abcde");
        assert_eq!(1, p.min);
        assert_eq!(3, p.max);
        assert_eq!('a', p.letter);
        assert_eq!("abcde", p.password);
    }

    #[test]
    fn can_take_string_to_diff_password() {
        let p = string_password("2-4 z: zzzzz");
        assert_eq!(2, p.min);
        assert_eq!(4, p.max);
        assert_eq!('z', p.letter);
        assert_eq!("zzzzz", p.password);
    }

    #[test]
    fn can_take_string_to_diff_password2() {
        let p = string_password("14-15 d: dzjgbdwdkdhdddh");
        assert_eq!(14, p.min);
        assert_eq!(15, p.max);
        assert_eq!('d', p.letter);
        assert_eq!("dzjgbdwdkdhdddh", p.password);
    }

    #[test]
    fn policy_2() {
        let p = Password {
            min: 1,
            max: 3,
            letter: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(true, valid_password_policy_2(&p));
    }

    #[test]
    fn policy_2_both_cannot_be_the_same() {
        let p = Password {
            min: 1,
            max: 3,
            letter: 'a',
            password: "abade".to_string(),
        };
        assert_eq!(false, valid_password_policy_2(&p));
    }

    #[test]
    fn policy_2_valid_in_second_position() {
        let p = Password {
            min: 1,
            max: 3,
            letter: 'a',
            password: "zbade".to_string(),
        };
        assert_eq!(true, valid_password_policy_2(&p));
    }
}
