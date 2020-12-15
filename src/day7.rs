use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> BagMap {
    let b: Vec<String> = input.lines().map(|k| k.to_string()).collect();

    BagMap::new(b)
}

#[aoc(day7, part1)]
fn part1(bags: &BagMap) -> usize {
    let found = bags.all_outer_bags_for_bag("shiny gold");
    found.iter().count()
}

#[aoc(day7, part2)]
fn part2(bags: &BagMap) -> u32 {
    let problem2 = bags.bags_within_bag("shiny gold");

    //Calculation will include this bag which we don't need
    static OUTER_BAG: u32 = 1;
    problem2 - OUTER_BAG
}

#[derive(Debug)]
pub struct BagMap {
    map: HashMap<String, Vec<(u32, String)>>,
}

impl BagMap {
    pub fn new(rules: Vec<String>) -> BagMap {
        let mut bags = HashMap::<String, Vec<(u32, String)>>::new();

        let mut s = String::new();

        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"((\d?)\W?\w*\W\w*)\Wbag").unwrap();
        }

        rules.iter().for_each(|l| {
            let mut firstbag = true;
            for cap in RE.captures_iter(&l) {
                if firstbag {
                    s = String::from(&cap[1]);
                    bags.insert(String::from(&cap[1]), vec![]);
                    firstbag = false;
                } else {
                    let v = &cap[1];
                    if v == " no other" {
                        continue;
                    }
                    let c: Vec<&str> = v.splitn(2, " ").collect();
                    let to_add = (c[0].parse::<u32>().unwrap(), String::from(c[1]));

                    let x = bags.get_mut(&s).unwrap();
                    x.push(to_add);
                }
            }
        });
        BagMap { map: bags }
    }

    pub fn bags_within_bag(&self, bag: &str) -> u32 {
        let v = self.map.get(bag).unwrap();

        let x = v
            .iter()
            .map(|(c, b)| c * self.bags_within_bag(b))
            .sum::<u32>();

        //println!("{:?} {:?} {:?}", x, bag, v.len());

        x + 1 // this bag
    }

    pub fn all_outer_bags_for_bag(&self, bag: &str) -> HashSet<String> {
        let mut found = HashSet::<String>::new();
        let mut to_process = Vec::<String>::new();
        to_process.push(bag.to_string());

        while let Some(i) = to_process.pop() {
            for x in self.outer_bags_for_bag(&i) {
                if !found.contains(&x) {
                    found.insert(x.to_string());
                    to_process.push(x.to_string());
                }
            }
        }
        found
    }

    fn outer_bags_for_bag(&self, bag: &str) -> Vec<String> {
        self.map
            .iter()
            .filter(|(_ob, rules)| rules.iter().any(|(_c, b)| b == bag))
            .map(|(ob, _rules)| ob.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example() {
        let l = test_lines();
        let bg = BagMap::new(l);
        let found = bg.all_outer_bags_for_bag("shiny gold");

        assert_eq!(found.len(), 4, "{:?}", found);
    }

    /*
    --- Part Two ---
    It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

    Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
    So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
    */
    #[test]
    fn how_many_bags_can_i_contain() {
        let l = test_lines();
        let bg = BagMap::new(l);
        let found = bg.bags_within_bag("shiny gold");
        assert_eq!(found-1, 32, "{:?}", found-1);
    }

    fn test_lines() -> Vec<String> {
        let s = [
            "bright red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        return s.iter().map(|s| String::from(*s)).collect();
    }
}
