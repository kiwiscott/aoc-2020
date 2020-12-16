use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|l| l.to_string().parse::<i32>().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn part1(stanza: &[i32]) -> i32 {
    let mut emg = ElvenMemoryGame::new(stanza);
    emg.play_until(2020)
}
#[aoc(day15, part2)]
fn part2(stanza: &[i32]) -> i32 {
    let mut emg = ElvenMemoryGame::new(stanza);
    emg.play_until(30000000)
}

struct ElvenMemoryGame {
    turns: HashMap<i32, (i32, i32)>,
    last: i32,
    turn: i32,
}

impl ElvenMemoryGame {
    pub fn new(stanza: &[i32]) -> Self {
        let mut turns = HashMap::<i32, (i32, i32)>::with_capacity(512);
        let turn: i32 = stanza.len() as i32;
        let mut last: i32 = 0;

        for i in stanza {
            let turn_count = (turns.len() as i32) + 1;

            match turns.get_mut(i) {
                Some(t) => {
                    *t = (t.1, turn_count);
                }
                None => {
                    turns.insert(*i, (0, turn_count));
                }
            };

            last = *i;
        }

        ElvenMemoryGame {
            turns: turns,
            last: last,
            turn: turn,
        }
    }
    pub fn next(&mut self) -> i32 {
        self.turn += 1;

        let (first, last) = self.turns.get(&self.last).unwrap();

        let value = match first == &0 || last == &0 {
            true => 0,
            false => last - first,
        };

        match self.turns.get_mut(&value) {
            Some(t) => {
                *t = (t.1, self.turn);
            }
            None => {
                self.turns.insert(value, (0, self.turn));
            }
        };

        self.last = value;
        value
    }
    pub fn play_until(&mut self, end_at: i32) -> i32 {
        while self.turn < end_at {
            self.next();
        }
        self.last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_example() {
        //Given 2,3,1, the 30000000th number spoken is 6895259
        let mut emg = ElvenMemoryGame::new(&[2, 3, 1]);
        let x = emg.play_until(30000000);
        assert_eq!(6895259, x);
    }
    #[test]
    fn play_until_2020() {
        let mut emg = ElvenMemoryGame::new(&[0, 3, 6]);
        let x = emg.play_until(2020);
        assert_eq!(436, x);
    }

    #[test]
    fn play_other_examples() {
        /*
        Given the starting numbers 1,3,2, the 2020th number spoken is 1.
        Given the starting numbers 2,1,3, the 2020th number spoken is 10.
        Given the starting numbers 1,2,3, the 2020th number spoken is 27.
        Given the starting numbers 2,3,1, the 2020th number spoken is 78.
        Given the starting numbers 3,2,1, the 2020th number spoken is 438.
        Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
        */
        let tests = [
            ([1, 3, 2], 1),
            ([2, 1, 3], 10),
            ([1, 2, 3], 27),
            ([2, 3, 1], 78),
            ([3, 2, 1], 438),
            ([3, 1, 2], 1836),
        ];

        for (start, expected) in tests.iter() {
            let mut emg = ElvenMemoryGame::new(start);
            assert_eq!(expected, &emg.play_until(2020));
        }
    }

    #[test]
    fn sample1() {
        let mut emg = ElvenMemoryGame::new(&[0, 3, 6]);
        /*
        Turn 1: The 1st number spoken is a starting number, 0.
        Turn 2: The 2nd number spoken is a starting number, 3.
        Turn 3: The 3rd number spoken is a starting number, 6.
        Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
        Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken
        (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
        Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
        Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
        Turn 8: Since 1 is new, the 8th number spoken is 0.
        Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
        Turn 10: 4 is new, so the 10th number spoken is 0.
        */

        assert_eq!(0, emg.next()); //Turn 4
        assert_eq!(3, emg.next()); //5
        assert_eq!(3, emg.next()); //6
        assert_eq!(1, emg.next()); //7
        assert_eq!(0, emg.next()); //8
        assert_eq!(4, emg.next()); //9
        assert_eq!(0, emg.next()); //10
    }
}
