#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    let data: Vec<(HashSet<String>, HashSet<String>)> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(" (contains ");
            let ingredients: HashSet<String> = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let allergens: HashSet<String> = iter
                .next()
                .unwrap()
                .split(')')
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            (ingredients, allergens)
        })
        .collect();
    data
}

#[aoc(day21, part1)]
fn part1(data: &Vec<(HashSet<String>, HashSet<String>)>) -> usize {
    let mut ingreds_to_allergen = HashMap::new();
    let mut all_ingreds = HashSet::new();

    for (ingredients, allergens) in data {
        for allergen in allergens {
            let distinct_ingreds = match ingreds_to_allergen.get(&allergen) {
                Some(existing_list) => ingredients.intersection(existing_list).cloned().collect(),
                None => ingredients.iter().cloned().collect(),
            };
            ingreds_to_allergen.insert(allergen, distinct_ingreds);
        }

        ingredients.iter().for_each(|i| {
            all_ingreds.insert(i.to_string());
        });
    }

    let ingreds_with_allergens: HashSet<_> = ingreds_to_allergen
        .values()
        .flat_map(|i| i)
        .cloned()
        .collect();

    let use_of_safe_ingreds = data
        .iter()
        .map(|(ingred, _)| {
            ingred
                .iter()
                .filter(|i| !ingreds_with_allergens.contains(i.as_str()))
                .count()
        })
        .sum::<usize>();

    use_of_safe_ingreds
}

#[aoc(day21, part2)]
fn part2(data: &Vec<(HashSet<String>, HashSet<String>)>) -> String {
    let mut ingreds_to_allergen = HashMap::new();
    for (ingredients, allergens) in data {
        for allergen in allergens {
            let distinct_ingreds = match ingreds_to_allergen.get(&allergen) {
                Some(existing_list) => ingredients.intersection(existing_list).cloned().collect(),
                None => ingredients.iter().cloned().collect(),
            };
            ingreds_to_allergen.insert(allergen, distinct_ingreds);
        }
    }

    let mut sorted_dangerous = HashMap::new();
    let mut counter = 0;

    while ingreds_to_allergen.len() != 0 {
        println!("{:?}", ingreds_to_allergen);

        ingreds_to_allergen = ingreds_to_allergen
            .into_iter()
            .filter(|(allergen, ingredients)| match ingredients.len() {
                1 => {
                    let ingred = ingredients.iter().next().unwrap().to_string();
                    sorted_dangerous.insert(ingred, *allergen);
                    false
                }
                _ => true,
            })
            .collect();

        for (ingred, _) in &sorted_dangerous {
            for (_, ingreds) in &mut ingreds_to_allergen {
                ingreds.remove(&ingred.to_string());
            }
        }
        counter += 1;
        if counter == 100 {
            break;
        }
    }

    println!("{:?}", sorted_dangerous);

    sorted_dangerous
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .map(|(_ingred, _aller)| _ingred.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_b() {
        //For each allergen find the words that are associated with it all times its mentioned
        let data = parse_input(&SAMPLE_DATA);
        let canonical_ingredients = part2(&data);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", canonical_ingredients);
    }

    #[test]
    fn part_a() {
        //For each allergen find the words that are associated with it all times its mentioned
        let data = parse_input(&SAMPLE_DATA);
        let use_of_safe_ingreds = part1(&data);
        assert_eq!(5, use_of_safe_ingreds);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ]
        .join("\n");
    }
}
