use aoc_runner_derive::{aoc, aoc_generator};
use regex::RegexSet;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut next_pass = String::new();
    let mut _l = String::new();

    for line in input.lines() {
        _l = line.to_string();
        if _l == "" {
            v.push(next_pass);
            next_pass = String::new();
        } else {
            _l.push('\n');
            next_pass += &_l;
        }
    }
    v.push(next_pass);
    return v;
}

#[aoc(day4, part1)]
fn part1(passports: &Vec<String>) -> usize {
    passports.iter().filter(|p| valid_passport(p)).count()
}
#[aoc(day4, part2)]
fn part2(passports: &Vec<String>) -> usize {
    passports.iter().filter(|p| valid_passport_2(p)).count()
}

fn valid_passport(data: &str) -> bool {
    lazy_static! {
        static ref RESET: RegexSet = RegexSet::new(&[
            r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:", r"cid:"
        ])
        .unwrap();
    }

    let matches = RESET.matches(data);

    return (0..7).all(|a| matches.matched(a));
}

/*
0:byr (Birth Year) - four digits; at least 1920 and at most 2002.
1:iyr (Issue Year) - four digits; at least 2010 and at most 2020.
2:eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
3:hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
4:hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
5:ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
6:pid (Passport ID) - a nine-digit number, including leading zeroes.
7:cid (Country ID) - ignored, missing or not.
*/

lazy_static! {
    static ref PASSPORT_RULES_2: RegexSet = RegexSet::new(&[
        r"byr:(19[2-9][0-9]|200[0-2])\b",
        r"iyr:(201[0-9]|2020)\b",
        r"eyr:(202[0-9]|2030)\b",
        r"hgt:((1([5-8][0-9]|[9][0-3])cm)|((59|6[0-9]|7[0-6])in))\b",
        r"hcl:#[0-9a-f]{6}\b",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"pid:[0-9]{9}\b"
    ])
    .unwrap();
}

fn valid_passport_2(data: &str) -> bool {
    let matches = PASSPORT_RULES_2.matches(data);
    return (0..7).all(|a| matches.matched(a));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pid() {
        /*
        6:pid (Passport ID) - a nine-digit number, including leading zeroes.
        */

        [1, 2, 3, 5667, 667788, 999999999].iter().for_each(|i| {
            let s = format!("pid:{:09}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(6), "expected:{:?}", s);
        });
    }

    #[test]
    fn test_ecl() {
        /*
        5:ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
         */

        ["amx", "blz", "zrn", "grz", "zrn", "nzl", "2025"]
            .iter()
            .for_each(|i| {
                let s = format!("ecl:{}", i);
                let matches = PASSPORT_RULES_2.matches(&s.to_string());
                assert!(!matches.matched(5), "expected:{:?}", s);
            });

        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .for_each(|i| {
                let s = format!("ecl:{}", i);
                let matches = PASSPORT_RULES_2.matches(&s.to_string());
                assert!(matches.matched(5), "expected:{:?}", s);
            });
    }

    #[test]
    fn test_hcl() {
        /*
        hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
         */

        ["#cfa07f", "#aaaaaa", "#cfa07d"].iter().for_each(|i| {
            let s = format!("hcl:{}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(4), "expected:{:?}", s);
        });
    }

    #[test]
    fn test_hgt() {
        /*
        hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
        */
        (150..193).for_each(|i| {
            let s = format!("hgt:{:?}cm", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(3), "expected:{:?}", s);
        });

        (59..76).for_each(|i| {
            let s = format!("hgt:{:?}in", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(3), "expected:{:?}", s);
        });

        ["149cm", "194cm", "58in", "77in"].iter().for_each(|i| {
            let s = format!("hgt:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(!matches.matched(3), "expected:{:?}", s);
        });
    }

    #[test]
    fn test_eyr() {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
        (2020..2030).for_each(|i| {
            let s = format!("eyr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(2), "expected:{:?}", s);
        });

        [2000, 2031].iter().for_each(|i| {
            let s = format!("eyr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(!matches.matched(2), "expected:{:?}", s);
        });
    }

    #[test]
    fn test_iyr() {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
        (2010..2020).for_each(|i| {
            let s = format!("iyr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(1), "expected:{:?}", s);
        });

        [2000, 2021].iter().for_each(|i| {
            let s = format!("iyr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(!matches.matched(1), "expected:{:?}", s);
        });
    }

    #[test]
    fn test_byr() {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
        (1920..2002).for_each(|i| {
            let s = format!("byr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(matches.matched(0), "expected:{:?}", s);
        });

        [1919, 2003].iter().for_each(|i| {
            let s = format!("byr:{:?}", i);
            let matches = PASSPORT_RULES_2.matches(&s.to_string());
            assert!(!matches.matched(0), "expected:{:?}", s);
        });
    }

    //Extract Multiline strings
    #[test]
    fn can_take_string_to_password() {
        let mut tests = std::collections::HashMap::new();
        tests.insert(
            true,
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        );
        tests.insert(
            false,
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
        );
        tests.insert(
            true,
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
        );
        tests.insert(
            false,
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in",
        );

        for (valid, data) in &tests {
            assert_eq!(
                valid,
                &valid_passport(data),
                "expected:{:?} data:{:?}",
                valid,
                data
            );
        }
    }
}
