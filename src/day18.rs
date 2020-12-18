#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Vec<Op>> {
    input.lines().map(|s| to_vec_char(s.to_string())).collect()
}

#[aoc(day18, part1)]
fn part1(problems: &Vec<Vec<Op>>) -> i64 {
    let solved: Vec<i64> = problems
        .iter()
        .map(|p| rec_math_it(p, &left_first_precedence))
        .collect();
    solved.iter().sum()
}

#[aoc(day18, part2)]
fn part2(problems: &Vec<Vec<Op>>) -> i64 {
    let solved: Vec<i64> = problems
        .iter()
        .map(|p| rec_math_it(p, &add_first_precedence))
        .collect();
    solved.iter().sum()
}

fn to_vec_char(s: String) -> Vec<Op> {
    s.chars()
        .filter(|s| s != &' ')
        .map(|s| match s {
            '+' => Op::Add,
            '*' => Op::Mul,
            '(' => Op::LParen,
            ')' => Op::RParen,
            _ => {
                let v = s.to_string().parse::<i64>().unwrap();
                Op::Value(v)
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    None,
    Add,
    Mul,
    LParen,
    RParen,
    Value(i64),
}

fn rec_math_it(values: &Vec<Op>, strategy: &dyn Fn(&Vec<Op>) -> i64) -> i64 {
    //find deepest left paren
    let deep = values
        .iter()
        .enumerate()
        .filter(|(_, op)| **op == Op::LParen)
        .map(|(i, _)| i)
        .max();

    if let Some(n) = deep {
        let rparen = values
            .iter()
            .skip(n)
            .enumerate()
            .find(|(_, op)| **op == Op::RParen);
        match rparen {
            None => panic!("Bad String"),
            Some((i, _)) => {
                let (start, right) = values.split_at(n);
                let (sub, end) = right.split_at(i + 1);

                let x = strategy(&sub.to_vec());
                let mut new_vec = start.to_vec();
                new_vec.push(Op::Value(x));
                for a in end {
                    new_vec.push(*a);
                }
                return rec_math_it(&new_vec, &strategy);
            }
        };
    } else {
        strategy(values)
    }
}
fn left_first_precedence(values: &Vec<Op>) -> i64 {
    let mut op = Op::None;
    values.iter().fold(0, |mut acc, v| {
        match v {
            Op::Add => op = Op::Add,
            Op::Mul => op = Op::Mul,
            Op::Value(n) => match op {
                Op::None => acc = *n,
                Op::Add => acc = acc + *n,
                Op::Mul => acc = acc * *n,
                _ => (),
            },
            _ => (),
        }

        acc
    })
}

fn add_first_precedence(values: &Vec<Op>) -> i64 {
    let mut new_vals = values.clone();

    while new_vals.iter().any(|op| *op == Op::Add) {
        for i in 0..values.len() {
            if new_vals[i] != Op::Add {
                continue;
            }
            let x = match new_vals[i - 1] {
                Op::Value(v) => v,
                _ => 0,
            };
            let y = match new_vals[i + 1] {
                Op::Value(v) => v,
                _ => 0,
            };

            let p = x + y;

            //There's three elements so remove 2 and write one
            new_vals.remove(i - 1);
            new_vals.remove(i - 1);
            new_vals[i - 1] = Op::Value(p);
            break;
        }
    }

    //Only the 'sum' values are relevant now
    let mut y = 1;
    for op in new_vals.iter() {
        match op {
            Op::Value(n) => y = y * n,
            _ => (),
        }
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_mul_first() {
        let vc = to_vec_char("1 + 2 * 3 + 4 * 5 + 6".to_string());
        assert_eq!(231, add_first_precedence(&vc));
    }

    #[test]
    fn part2_examples() {
        let vc = to_vec_char("1 + (2 * 3) + (4 * (5 + 6))".to_string());
        assert_eq!(51, rec_math_it(&vc, &add_first_precedence));

        let vc = to_vec_char("2 * 3 + (4 * 5)".to_string());
        assert_eq!(46, rec_math_it(&vc, &add_first_precedence));

        let vc = to_vec_char("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string());
        assert_eq!(669060, rec_math_it(&vc, &add_first_precedence));

        let vc = to_vec_char("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string());
        assert_eq!(23340, rec_math_it(&vc, &add_first_precedence));
    }

    #[test]
    fn result_simple() {
        let vc = to_vec_char("1 + 2 * 3 + 4 * 5 + 6".to_string());
        assert_eq!(71, rec_math_it(&vc, &left_first_precedence));
    }

    #[test]
    fn result_paren() {
        let vc = to_vec_char("1 + (2 * 3) + (4 * (5 + 6))".to_string());
        assert_eq!(51, rec_math_it(&vc, &left_first_precedence));
    }

    #[test]
    fn part1_examples() {
        let vc = to_vec_char("2 * 3 + (4 * 5)".to_string());
        assert_eq!(26, rec_math_it(&vc, &left_first_precedence));
        let vc = to_vec_char("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string());
        assert_eq!(437, rec_math_it(&vc, &left_first_precedence));
        let vc = to_vec_char("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string());
        assert_eq!(12240, rec_math_it(&vc, &left_first_precedence));
        let vc = to_vec_char("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string());
        assert_eq!(13632, rec_math_it(&vc, &left_first_precedence));
    }
}
