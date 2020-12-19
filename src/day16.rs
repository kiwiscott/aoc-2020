use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}
impl Rule {
    fn is_in_range(&self, value: &i32) -> bool {
        self.ranges
            .iter()
            .any(|(lower, upper)| lower <= value && value <= upper)
    }
}

type Rules = Vec<Rule>;
type Tickets = Vec<Ticket>;
type Ticket = Vec<i32>;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> (Rules, Ticket, Tickets) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(?P<field>[\w\s]*?):[\W*](?P<l1>\d*)-(?P<l2>\d*) or (?P<v1>\d*)-(?P<v2>\d*)$"
        )
        .unwrap();
    }
    let mut processing = "rules";
    let mut rules = Rules::new();
    let mut tickets = Tickets::new();
    let mut your_ticket = Ticket::new();

    for line in input.lines().filter(|p| p.len() > 0) {
        if line == "your ticket:" {
            processing = "your ticket";
            continue;
        } else if line == "nearby tickets:" {
            processing = "nearby tickets";
            continue;
        }

        match processing {
            "rules" => {
                let captures = RE.captures(line).unwrap();
                let r = Rule {
                    name: captures.name("field").unwrap().as_str().to_string(),
                    ranges: vec![
                        (
                            captures.name("l1").unwrap().as_str().parse().unwrap(),
                            captures.name("l2").unwrap().as_str().parse().unwrap(),
                        ),
                        (
                            captures.name("v1").unwrap().as_str().parse().unwrap(),
                            captures.name("v2").unwrap().as_str().parse().unwrap(),
                        ),
                    ],
                };
                rules.push(r);
            }
            "your ticket" => {
                your_ticket = line
                    .split(',')
                    .map(|l| l.to_string().parse::<i32>().unwrap())
                    .collect();
            }
            "nearby tickets" => {
                let ticket = line
                    .split(',')
                    .map(|l| l.to_string().parse::<i32>().unwrap())
                    .collect();
                tickets.push(ticket);
            }
            _ => (),
        }
    }

    (rules, your_ticket, tickets)
}

#[aoc(day16, part1)]
fn part1((rules, _your_ticket, others_tickets): &(Rules, Ticket, Tickets)) -> i32 {
    let mut unmatched = vec![];

    for ticket in others_tickets {
        for value in ticket {
            match rules.iter().any(|r| r.is_in_range(value)) {
                false => unmatched.push(*value),
                _ => (),
            }
        }
    }
    println!("{:?}", unmatched.len(),);

    unmatched.iter().sum::<i32>()
}

#[aoc(day16, part2)]
fn part2((rules, my_ticket, others_tickets): &(Rules, Ticket, Tickets)) -> i64 {
    //discard the unmatched tickets
    let valid_tickets: Tickets = others_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| rules.iter().any(|r| r.is_in_range(value)))
        })
        .map(|t| t.clone())
        .collect();

    //190 tickets left

    //we need to test against rule 1 to see if there's more
    let mut allocated = HashMap::<String, usize>::new();
    while rules.len() != allocated.len() {
        for i in 0..rules.len() {
            let y: Vec<&Rule> = rules
                .iter()
                .filter(|rule| !allocated.contains_key(&rule.name))
                .filter(|rule| valid_tickets.iter().all(|t| rule.is_in_range(&t[i])))
                .map(|rule| rule.clone())
                .collect();

            if y.iter().len() == 1 {
                allocated.insert(y[0].name.clone(), i);
                break;
            }
        }
    }

    let x: Vec<i64> = allocated
        .iter()
        .filter(|(k, _v)| k.contains("departure"))
        .map(|(_k, v)| *my_ticket.get(*v).unwrap() as i64)
        .collect();

    x.iter().product::<i64>()
}
