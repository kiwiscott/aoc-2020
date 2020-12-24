use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .chars()
        .take_while(|xchar| xchar.is_numeric())
        .map(|xchar| xchar.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[aoc(day23, part1)]
fn part1(cups: &[usize]) -> String {
    let mut crab_cups = CrabCups::new(cups);
    crab_cups.a_move(100);

    let mut n = 1;
    let mut res = vec![];
    loop {
        n = crab_cups.cups[n];

        if res.contains(&n.to_string()) || n == 1 {
            break;
        }
        res.push(n.to_string());
    }
    res.join("")
}

#[aoc(day23, part2)]
fn part2(cups: &[usize]) -> usize {
    let mut lot_sa_cups: Vec<usize> = cups.iter().map(|v| *v).collect();
    let mut vec2: Vec<usize> = (10..=1_000_000).map(|v| v as usize).collect();
    lot_sa_cups.append(&mut vec2);

    //println!("{:?}", lot_sa_cups);

    let mut crab_cups = CrabCups::new(&lot_sa_cups);
    crab_cups.a_move(10_000_000);

    let a = crab_cups.cups[1];
    let b = crab_cups.cups[crab_cups.cups[1]];
    println!("{:?} * {:?} = {:?}", a, b, a * b);
    a * b
}

/*
-- Part Two ---
Due to what you can only assume is a mistranslation (you're not exactly fluent in Crab), you are quite surprised when the crab starts arranging many cups in a circle on your raft - one million (1000000) in total.

Your labeling is still correct for the first few cups; after that, the remaining cups are just numbered in an increasing fashion starting from the number after the highest number in your list and proceeding one by one until one million is reached. (For example, if your labeling were 54321, the cups would be numbered 5, 4, 3, 2, 1, and then start counting up from 6 until one million is reached.) In this way, every number from one through one million is used exactly once.

After discovering where you made the mistake in translating Crab Numbers, you realize the small crab isn't going to do merely 100 moves; the crab is going to do ten million (10000000) moves!

The crab is going to hide your stars - one each - under the two cups that will end up immediately clockwise of cup 1. You can have them if you predict what the labels on those cups will be when the crab is finished.

In the above example (389125467), this would be 934001 and then 159792; multiplying these together produces 149245887792.

Determine which two cups will end up immediately clockwise of cup 1. What do you get if you multiply their labels together?
*/

struct CrabCups {
    cups: Vec<usize>,
    max_cup: usize,
}

impl CrabCups {
    fn new(cups: &[usize]) -> Self {
        //Prepare the dict lookup
        let mut ring = vec![0; cups.len() + 1];

        ring[0] = cups[0];
        for i in 1..cups.len() {
            ring[cups[i - 1]] = cups[i];
        }
        ring[*cups.iter().last().unwrap() as usize] = ring[0];

        CrabCups {
            cups: ring,
            max_cup: *cups.iter().max().unwrap(),
        }
    }

    fn a_move(&mut self, moves: u32) {
        let mut moves_done = 0;

        let mut c_index = 0;

        while moves_done != moves {
            let current = self.cups[c_index];

            let cup_holder = [
                self.cups[current],
                self.cups[self.cups[current]],
                self.cups[self.cups[self.cups[current]]],
            ];

            /*  println!("-----MOVE {:?}------", moves_done);
            println!("Current: {:?}", current);

            println!("cup_holder {:?}", cup_holder);
            println!("order_cups {:?}", self.order_cups()); */

            self.cups[current] = self.cups[cup_holder[2]];

            let mut destination = current;
            while destination == current || cup_holder.contains(&destination) {
                destination -= 1;
                if destination == 0 {
                    destination = self.max_cup;
                }
            }
            //println!("destination {:?}\n", destination);

            //Insert back into the ring
            self.cups[cup_holder[2]] = self.cups[destination];
            self.cups[destination] = cup_holder[0];
            //Set up next Round
            moves_done += 1;
            //move cursor to start number again
            c_index = current;
        }
    }

    fn order_cups(&self) -> Vec<usize> {
        let mut n = 0;
        /* println!(
            "{:?}",
            self.cups
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v))
                .collect::<Vec::<(usize, usize)>>()
        ); */
        let mut res = vec![];
        loop {
            n = self.cups[n];
            if res.contains(&n) {
                break;
            }
            res.push(n);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_b() {
        let data = parse_input(&SAMPLE_DATA);
        let x = part2(&data);
        assert_eq!(149245887792, x);
    }

    #[test]
    fn test_part_1() {
        let data = parse_input(&SAMPLE_DATA);
        let s = part1(&data);
        assert_eq!("67384529", s);
    }

    #[test]
    fn test_3_moves() {
        let data = parse_input(&SAMPLE_DATA);

        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(1);
        assert_eq!(order_cups(crab_cups), [3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn test_2_moves() {
        let data = parse_input(&SAMPLE_DATA);

        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(2);
        assert_eq!(order_cups(crab_cups), [3, 2, 5, 4, 6, 7, 8, 9, 1]);
    }

    fn order_cups(cc: CrabCups) -> Vec<usize> {
        let mut n = 0;
        println!(
            "{:?}",
            cc.cups
                .iter()
                .enumerate()
                .map(|(i, v)| (i, *v))
                .collect::<Vec::<(usize, usize)>>()
        );
        let mut res = vec![];
        loop {
            n = cc.cups[n];
            if res.contains(&n) {
                break;
            }
            res.push(n);
        }
        res
    }

    #[test]
    fn test_1_move() {
        let data = parse_input(&SAMPLE_DATA);

        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(1);
        assert_eq!(order_cups(crab_cups), [3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn test_input() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(data, [3, 8, 9, 1, 2, 5, 4, 6, 7]);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = ["389125467", "",].join("\n");
    }
}
