use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|c| seat_id(&c)).collect()
}

#[aoc(day5, part1)]
fn part1(seats: &Vec<u32>) -> u32 {
    *seats.iter().max().unwrap()
}
#[aoc(day5, part2)]
fn part2(seats: &Vec<u32>) -> u32 {
    let max_seat: u32 = *seats.iter().max().unwrap();

    let allocated_seats: HashSet<u32> = seats.iter().map(|i| *i).collect();

    let _free_seats: Vec<u32> = (8..max_seat)
        .filter(|i| !allocated_seats.contains(i))
        .collect();
    //HINT: Its not at the front of the plane so lets take the last
    *_free_seats.last().unwrap()
}

fn seat_id(code: &str) -> u32 {
    //Have to use floats here as there's no ceiling / floor function in 'integers'
    //and casting back and forth isn't a good idea.
    let mut last: f32 = 127.0;
    let mut start: f32 = 0.0;

    let mut clast: f32 = 7.0;
    let mut cstart: f32 = 0.0;

    for c in code.chars() {
        match c {
            'F' => last = start + ((last - start) / 2.0).floor(),
            'B' => start = start + ((last - start) / 2.0).ceil(),

            'L' => clast = cstart + ((clast - cstart) / 2.0).floor(),
            'R' => cstart = cstart + ((clast - cstart) / 2.0).ceil(),

            _ => (),
        }
    }
    return (last * 8.0 + clast) as u32;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn testx() {
        assert_eq!(357, seat_id("FBFBBFFRLR"), "FBFBBFFRLR");
        assert_eq!(567, seat_id("BFFFBBFRRR"), "BFFFBBFRRR");
        assert_eq!(119, seat_id("FFFBBBFRRR"), "FFFBBBFRRR");
        assert_eq!(820, seat_id("BBFFBBFRLL"), "BBFFBBFRLL");
    }
}
