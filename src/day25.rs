use aoc_runner_derive::{aoc, aoc_generator};
//use std::collections::HashSet;
//use std::slice::Iter;

#[aoc_generator(day25)]
fn parse_input(input: &str) -> (u128, u128) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse::<u128>().unwrap(),
        lines.next().unwrap().parse::<u128>().unwrap(),
    )
}

const DIVISOR: u128 = 20201227;

#[aoc(day25, part1)]
fn part1((card_pk, door_pk): &(u128, u128)) -> u128 {
    match find_subject(*card_pk) {
        Some(n) => {
            let room_loop = find_loop_size(*door_pk, n);
            let pk_from_card = calculate_encryption_key(*card_pk, room_loop);
            return pk_from_card;
        }
        None => panic!("Cannot calculate subject for value"),
    }
}

fn find_loop_size(pk: u128, subject: u128) -> u128 {
    let mut value: u128 = 1;
    let mut loop_size: u128 = 0;

    while pk != value {
        loop_size += 1;
        value = value * subject;
        value = value % DIVISOR;
    }
    return loop_size;
}

#[allow(unused_assignments)]
fn find_subject(pk: u128) -> Option<u128> {
    let break_at = 100_000_000;

    let mut subject_break = 2;
    let mut loop_break = 0;
    let mut value = 0;

    while subject_break < break_at {
        subject_break += 1;
        loop_break = 0;
        value = 1;
        while loop_break < break_at {
            loop_break += 1;
            value = value * subject_break;
            value = value % DIVISOR;

            if value == pk {
                return Some(subject_break);
            }
        }
    }
    None
}

fn calculate_encryption_key(subject: u128, loop_size: u128) -> u128 {
    let mut value: u128 = 1;

    let mut loops: u128 = 0;
    while loop_size != loops {
        loops += 1;
        value = value * subject;
        value = value % 20201227;
    }

    value
}

/*
 handshake used by the card and the door involves an operation that transforms a subject number.
 To transform a subject number, start with the value 1. Then, a number of times called the loop size, perform the following steps:

Set the value to itself multiplied by the subject number.
Set the value to the remainder after dividing the value by 20201227.
The card always uses a specific, secret loop size when it transforms a subject number. The door always uses a different, secret loop size.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_subject_test() {
        let (card_pk, _) = parse_input(&SAMPLE_DATA);

        let card_loop = find_subject(card_pk);
        assert_eq!(Some(7_u128), card_loop);
    }

    #[test]
    fn test_sn() {
        let (card_pk, room_pk) = parse_input(&SAMPLE_DATA);
        let subject = find_subject(card_pk).unwrap();
        let card_loop = find_loop_size(card_pk, subject);
        let room_loop = find_loop_size(room_pk, subject);

        assert_eq!(8, card_loop);
        assert_eq!(11, room_loop);

        let pk_from_room = calculate_encryption_key(room_pk, card_loop);
        let pk_from_card = calculate_encryption_key(card_pk, room_loop);

        assert_eq!(pk_from_card, 14897079);
        assert_eq!(pk_from_card, pk_from_room);
    }

    #[test]
    fn test_part_1() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(14897079, part1(&data));
    }

    #[test]
    fn parse_data() {
        let (pk1, pk2) = parse_input(&SAMPLE_DATA);
        assert_eq!(pk1, 5764801);
        assert_eq!(pk2, 17807724);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = ["5764801", "17807724", "",].join("\n");
    }
}
