use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(the_numbers: &Vec<i32>) -> i32 {
    let mut parts = Vec::<i32>::new();
    number_of_elements_equal(2020, the_numbers, 2, &mut parts);
    parts.iter().product::<i32>()
}

#[aoc(day1, part2)]
fn part2(the_numbers: &Vec<i32>) -> i32 {
    let mut parts = Vec::<i32>::new();
    number_of_elements_equal(2020, the_numbers, 3, &mut parts);
    parts.iter().product::<i32>()
}

fn find_number_two_values(value: i32, numbers: &Vec<i32>) -> (i32, i32) {
    for i in numbers {
        let left_over = &(value - i);
        if numbers.contains(left_over) {
            return (*i, *left_over);
        }
    }
    return (0, 0);
}

fn number_of_elements_equal(value: i32, numbers: &Vec<i32>, elements: i32, parts: &mut Vec<i32>) {
    //if there are two parts only required we need to find a number that equals the value
    if elements == 2 {
        let x = find_number_two_values(value, numbers);
        if x != (0, 0) {
            parts.push(x.0);
            parts.push(x.1);
            return;
        }
    }

    if elements > 2 {
        for i in numbers {
            number_of_elements_equal(value - i, numbers, elements - 1, parts);
            if !parts.is_empty() {
                parts.insert(0, *i);
                return;
            }
        }
    }
}
