#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Part {
    Literal(char),
    Rule(i32),
    Or,
    Compound(Vec<Part>),
    RecursiveOne(Vec<Part>),
    RecursiveMatch(Vec<Part>),
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (HashMap<i32, Part>, Vec<String>) {
    lazy_static! {
        static ref SINGLE_CHAR: Regex = Regex::new(r#"^"([a-z])"$"#).unwrap();
    }
    let mut processed = HashMap::<i32, Part>::new();

    for line in input.lines().take_while(|l| l.trim().len() > 0) {
        let v: Vec<&str> = line.trim().splitn(2, ':').collect();
        let index: i32 = v[0].parse().unwrap();
        let second_part = v[1].trim().to_string();

        if SINGLE_CHAR.is_match(&second_part) {
            let ch = second_part.chars().nth(1).unwrap();
            processed.insert(index, Part::Literal(ch));
        } else {
            let com_parts: Vec<Part> = second_part
                .split_ascii_whitespace()
                .map(|t| match t {
                    "|" => Part::Or,
                    _ => Part::Rule(t.parse::<i32>().unwrap()),
                })
                .collect();
            processed.insert(index, Part::Compound(com_parts));
        }
    }
    let mut tests = vec![];

    for line in input.lines().skip_while(|l| l.trim().len() != 0) {
        if line.trim().len() == 0 {
            continue;
        }
        tests.push(line.trim().to_string());
    }

    (processed, tests)
}

#[aoc(day19, part1)]
fn part1((rules, tests): &(HashMap<i32, Part>, Vec<String>)) -> usize {
    let mut rule_built = String::from("^");
    rule_built.push_str(&process_rule(rules.get(&0).unwrap(), &rules));
    rule_built.push('$');

    let re = Regex::new(&rule_built).unwrap();

    tests.iter().filter(|r| re.is_match(r)).count()
}

#[aoc(day19, part2)]
fn part2((rules, tests): &(HashMap<i32, Part>, Vec<String>)) -> usize {
    let mut new_rules = HashMap::<i32, Part>::new();
    for (k, v) in rules {
        new_rules.insert(*k, v.clone());
    }
    //8: 42 | 42 8
    //11: 42 31 | 42 11 31

    //8: 42 | 42 42 | 42 42 42
    //11: 42 31 | 42 (42 (42 (42 (42 31) 31) 31) 31) 31
    //The problem is the inners must match !!!!!

    new_rules.insert(8, Part::RecursiveOne(vec![Part::Rule(42)]));

    new_rules.insert(
        11,
        Part::RecursiveMatch(vec![Part::Rule(42), Part::Rule(31)]),
    );

    let mut rule_built = String::from("^");
    rule_built.push_str(&process_rule(new_rules.get(&0).unwrap(), &new_rules));
    rule_built.push('$');

    let re = Regex::new(&rule_built).unwrap();

    tests.iter().filter(|r| re.is_match(r)).count()
}

fn process_rule(rule: &Part, rule_set: &HashMap<i32, Part>) -> String {
    let mut s = String::new();

    match rule {
        Part::Literal(l) => s.push(*l),
        Part::Or => s.push('|'),
        Part::Rule(i) => {
            let ss = process_rule(rule_set.get(&i).unwrap(), rule_set);
            s.push_str(&ss);
        }
        Part::Compound(set) => {
            s.push_str("(?:");
            for part in set {
                let ss = process_rule(part, rule_set);
                s.push_str(&ss);
            }
            s.push(')');
        }
        Part::RecursiveOne(set) => {
            s.push_str("(?:");
            for part in set {
                let ss = process_rule(part, rule_set);
                s.push_str(&ss);
            }
            s.push(')');
            s.push_str("+");
        }
        Part::RecursiveMatch(set) => {
            //We must match this recursion one or more times.
            //The problem with this appraoch is that we have to match to a set number of times (e.g. 10 below because both sides must match)
            // os we nned a match something like ( (a[1]b[1]) | (a[2]b[2]) || ... )
            //but we can't match ( (a[1]b[2]) | (a[2]b[1]) || ... )

            s.push_str("(?:");
            for i in 1..10 {
                if i != 1 {
                    s.push('|');
                }
                s.push_str("(?:");
                for part in set {
                    s.push_str("(?:");
                    let ss = process_rule(part, rule_set);
                    s.push_str(&ss);
                    s.push_str(&format!("{{{:?}}}", i));
                    s.push(')');
                }
                s.push(')');
            }
            //s.push_str("*");
            s.push(')');
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_first_part() {
        let s = "0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: \"a\"
            5: \"b\"";

        let (rules, tests) = parse_input(s);
        assert_eq!(6, rules.len());
        assert_eq!(0, tests.len());
    }

    #[test]
    fn first_part_to_string() {
        let rule1 = "(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)";

        let s = "0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: \"a\"
            5: \"b\"";

        let (rules, _) = parse_input(s);
        assert_eq!(rule1, process_rule(rules.get(&0).unwrap(), &rules));
    }
    #[test]
    fn first_examples() {
        let s = "0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: \"a\"
            5: \"b\"";
        let (rules, _) = parse_input(s);

        let mut rule_built = String::from("^");
        rule_built.push_str(&process_rule(rules.get(&0).unwrap(), &rules));
        rule_built.push('$');

        let re = Regex::new(&rule_built).unwrap();
        assert_eq!(true, re.is_match("ababbb"));
        assert_eq!(false, re.is_match("bababa"));
        assert_eq!(true, re.is_match("abbbab"));
        assert_eq!(false, re.is_match("aaabbb"));
        assert_eq!(false, re.is_match("aaaabbb"));
    }
    #[test]
    fn full_parse_example() {
        let s = "0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: \"a\"
            5: \"b\"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb";
        let (rules, tests) = parse_input(s);

        println!("{:?} {:?}", rules, tests);
        assert_eq!(6, rules.len());
        assert_eq!(5, tests.len());
    }

    #[test]
    fn part2_example_b4_changes() {
        let s = r#"42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let (rules, tests) = parse_input(s);

        let mut rule_built = String::from("^");
        rule_built.push_str(&process_rule(rules.get(&0).unwrap(), &rules));
        rule_built.push('$');

        let re = Regex::new(&rule_built).unwrap();
        assert_eq!(3, tests.iter().filter(|r| re.is_match(r)).count());
    }
    #[test]
    fn part2_example_with_changes() {
        let s = r#"42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        let (mut rules, tests) = parse_input(s);
        //8: 42 | 42 8
        //11: 42 31 | 42 11 31

        rules.insert(8, Part::RecursiveOne(vec![Part::Rule(42)]));
        rules.insert(
            11,
            Part::Compound(vec![
                Part::RecursiveOne(vec![Part::Rule(42)]),
                Part::RecursiveOne(vec![Part::Rule(31)]),
            ]),
        );

        let mut rule_built = String::from("^");
        rule_built.push_str(&process_rule(rules.get(&0).unwrap(), &rules));
        rule_built.push('$');

        let re = Regex::new(&rule_built).unwrap();
        assert_eq!(12, tests.iter().filter(|r| re.is_match(r)).count());
    }
}
