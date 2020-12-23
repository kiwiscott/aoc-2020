use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<u32> {
    input
        .chars()
        .take_while(|xchar| xchar.is_numeric())
        .map(|xchar| xchar.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[aoc(day23, part1)]
fn part1(cups: &[u32]) -> String {
    let mut crab_cups = CrabCups::new(cups);
    crab_cups.a_move(100);
    crab_cups.result()
}

#[aoc(day23, part2)]
fn part2(cups: &[u32]) -> u32 {
    let mut lot_sa_cups: Vec<u32> = cups.iter().map(|v| *v).collect();
    let mut vec2: Vec<u32> = (10..=1_000_000).collect();
    lot_sa_cups.append(&mut vec2);

    //println!("{:?}", lot_sa_cups);

    //let mut crab_cups = CrabCups::new(&lot_sa_cups);
    //crab_cups.a_move(100);
    //crab_cups.result()
    0
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
    cups: Vec<u32>,
    cup_holder: Vec<u32>,
    current_index: usize,
    max_cup: u32,
    min_cup: u32,
}

impl CrabCups {
    fn new(cups: &[u32]) -> Self {
        let buf: Vec<_> = cups.iter().map(|v| *v).collect();
        CrabCups {
            cups: buf,
            cup_holder: Vec::<u32>::with_capacity(3),
            current_index: 0,
            max_cup: *cups.iter().max().unwrap(),
            min_cup: *cups.iter().min().unwrap(),
        }
    }

    #[allow(irrefutable_let_patterns)]
    fn a_move(&mut self, moves: u32) {
        let mut moves_left = moves;

        while moves_left != 0 {
            //println!("--- move {:?} ---", moves - moves_left + 1);
            //println!(" cups: {:?}", self.cups);
            //find the value and insert
            let start_number = self.next();
            let mut destination = start_number - 1;
            if destination == 0 {
                destination = self.max_cup;
            }
            self.move_cups();
            //println!(
            //    " active:*****{:?}************* : esearching for: {:?}",
            //    start_number, destination
            //);
            //println!(" pick up: {:?}", self.cup_holder);
            //println!(" X-cups: {:?}", self.cups);

            while let cup = self.next() {
                //println!(" searching.......: {:?} == {:?}", destination, cup);

                if cup == destination {
                    //println!(" destination: {:?}\n", destination);
                    //println!(" move to destination: {:?}\n", self.current_index);
                    self.insert_cups();
                    break;
                } else if destination == 0 || (cup == start_number && destination == self.min_cup) {
                    destination = self.max_cup;
                } else if cup == start_number {
                    destination -= 1;
                }
            }

            //Set up next Round
            moves_left -= 1;
            //move cursor to start number again
            while let cup = self.next() {
                if cup == start_number {
                    break;
                }
            }
            self.cup_holder.clear();
        }
    }

    #[allow(irrefutable_let_patterns)]
    fn result(&mut self) -> String {
        while let cup = self.next() {
            if cup == 1 {
                break;
            }
        }
        let mut s = String::new();
        while let cup = self.next() {
            if cup == 1 {
                break;
            }
            s.push_str(&cup.to_string());
        }
        s
    }

    fn move_cups(&mut self) {
        for _ in 0..3 {
            let mut remove = self.current_index;
            if remove >= self.cups.len() {
                remove = 0;
            }
            let i = self.cups.remove(remove);
            self.cup_holder.push(i);
        }
    }
    fn insert_cups(&mut self) {
        while let Some(c) = self.cup_holder.pop() {
            //println!(" inserting:{:?} at index:{:?}", c, self.current_index);

            self.cups.insert(self.current_index, c);
        }
    }

    fn next(&mut self) -> u32 {
        match self.cups.get(self.current_index) {
            Some(n) => {
                self.current_index += 1;
                *n
            }
            None => {
                self.current_index = 0;
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_b() {
        let data = parse_input(&SAMPLE_DATA);
        let max = data.iter().max().unwrap();

        let mut lot_sa_cups: Vec<u32> = data.iter().map(|v| *v).collect();
        let mut vec2: Vec<u32> = ((max+1)..=1_000_000).collect();
        lot_sa_cups.append(&mut vec2);
        
        let mut crab_cups = CrabCups::new(&lot_sa_cups);
        crab_cups.a_move(1_000_000);

        let mut a:u32 = 0; 
        let mut b:u32 = 0; 



        #[allow(irrefutable_let_patterns)]
        while let n = crab_cups.next(){
            if n ==1{
                let a = crab_cups.next(); 
                let b =  crab_cups.next();
                println!("{:?} * {:?} = {:?}",a,b,a*b);
                break; 
            }
        }

        //934001 and then 159792; multiplying these together produces 149245887792.
        assert_eq!(934001,a);
        assert_eq!(159792,b);
        assert_eq!(149245887792,a as u64*b as u64);
    }

    #[test]
    fn test_10_moves() {
        let data = parse_input(&SAMPLE_DATA);
        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(10);

        assert_eq!("92658374", crab_cups.result());
    }

    #[test]
    fn test_2_moves() {
        let data = parse_input(&SAMPLE_DATA);

        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(1);
        assert_eq!(crab_cups.cups, [3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }
    #[test]
    fn test_1_move() {
        let data = parse_input(&SAMPLE_DATA);

        let mut crab_cups = CrabCups::new(&data);
        crab_cups.a_move(1);
        assert_eq!(crab_cups.cups, [3, 2, 8, 9, 1, 5, 4, 6, 7]);
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
