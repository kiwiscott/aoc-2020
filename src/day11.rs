use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;
use std::fmt;

const COVID_ADJ_THRESHOLD: u8 = 4;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day11, part1)]
fn part1(data: &Vec<String>) -> usize {
    let mut ferry = Ferry::from(data);
    ferry.covid_adj_threshold = 4;
    while ferry.shuffle() {
        ()
    }
    ferry.occupied_seats()
}
#[aoc(day11, part2)]
fn part2(data: &Vec<String>) -> usize {
    let mut ferry = Ferry::from(data);
    ferry.covid_adj_threshold = 5;
    ferry.max_distance = 100;
    while ferry.shuffle() {
        ()
    }
    ferry.occupied_seats()
}

#[derive(Debug)]
struct Ferry {
    covid_adj_threshold: u8,
    rows: Vec<Vec<SeatState>>,
    already_shuffled: bool,
    max_distance: i32,
}

impl Ferry {
    /// Returns a bool indicating if people seats states changed.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    fn shuffle(&mut self) -> bool {
        let mut new_rows = Vec::new();
        let mut were_there_changes = false;

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut new_seats = Vec::new();
            for (seat_index, seat) in row.iter().enumerate() {
                let (changed, new_state) =
                    self.new_state(*seat, (row_index as isize) + 1, (seat_index as isize) + 1);

                if changed {
                    were_there_changes = true;
                }
                new_seats.push(new_state);
            }
            new_rows.push(new_seats);
        }
        self.rows = new_rows;
        self.already_shuffled = true;
        were_there_changes
    }

    fn occupied_seats(&self) -> usize {
        self.rows
            .iter()
            .map(|seats| {
                seats
                    .iter()
                    .filter(|seat| **seat == SeatState::Occupied)
                    .count()
            })
            .sum()
    }

    fn new_state(&self, state: SeatState, row: isize, seat: isize) -> (bool, SeatState) {
        /*
         - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
         - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
         - Otherwise, the seat's state does not change.
        */
        //println!("Checking Seat {:?} {:?} {:?}", state, row, seat);
        if state == SeatState::Floor {
            return (false, SeatState::Floor);
        }

        if !self.already_shuffled {
            return (true, SeatState::Occupied);
        }

        //println!("{:?}", self.adjacent_occupied_seats(row, seat));

        if state == SeatState::Empty && self.adjacent_occupied_seats(row, seat) == 0 {
            return (true, SeatState::Occupied);
        }

        if state == SeatState::Occupied
            && self.adjacent_occupied_seats(row, seat) >= self.covid_adj_threshold
        {
            return (true, SeatState::Empty);
        }

        (false, state)
    }

    fn adjacent_occupied_seats(&self, row: isize, seat: isize) -> u8 {
        let mut neighbours = 0;

        let check_row = row - 1; //zero based
        let check_seat = seat - 1; //zero based

        for ri in [-1, 0, 1].iter() {
            for si in [-1, 0, 1].iter() {
                if *ri == 0 && *si == 0 {
                    continue;
                }

                let mut cr = check_row + ri;
                let mut cs = check_seat + si;
                let mut counter = 1;

                while self.between_zero_and(cr, self.rows.len())
                    && self.between_zero_and(cs, self.rows[0].len())
                    && self.rows[usize::try_from(cr).unwrap()][usize::try_from(cs).unwrap()]
                        == SeatState::Floor
                {
                    if counter >= self.max_distance {
                        break;
                    }
                    cr = cr + ri;
                    cs = cs + si;
                    counter += 1;
                }
                //println!("{:?} {:?}",cr,cs);
                if self.between_zero_and(cr, self.rows.len())
                    && self.between_zero_and(cs, self.rows[0].len())
                    && self.rows[usize::try_from(cr).unwrap()][usize::try_from(cs).unwrap()]
                        == SeatState::Occupied
                {
                    neighbours += 1;
                }
            }
        }

        return neighbours;
    }
    fn between_zero_and(&self, value: isize, max: usize) -> bool {
        value >= 0 && usize::try_from(value).unwrap() < max
    }
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::new();
        for r in &self.rows {
            r.iter().for_each(|ss| {
                formatted += &ss.to_string();
            });
            formatted += "\n";
        }
        write!(f, "{}", formatted)
    }
}

impl From<&Vec<String>> for Ferry {
    fn from(lines: &Vec<String>) -> Self {
        let mut rows: Vec<Vec<SeatState>> = vec![];

        for line in lines {
            let parsed = line
                .chars()
                .map(|c| SeatState::from(c))
                .collect::<Vec<SeatState>>();
            rows.push(parsed);
        }

        Ferry {
            rows: rows,
            covid_adj_threshold: COVID_ADJ_THRESHOLD,
            already_shuffled: false,
            max_distance: 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SeatState {
    Floor,
    Occupied,
    Empty,
}

impl From<char> for SeatState {
    fn from(c: char) -> Self {
        match c {
            '.' => SeatState::Floor,
            '#' => SeatState::Occupied,
            'L' => SeatState::Empty,
            _ => panic!("Cannot parse bad data!!"),
        }
    }
}

impl fmt::Display for SeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeatState::Floor => write!(f, "."),
            SeatState::Occupied => write!(f, "#"),
            SeatState::Empty => write!(f, "L"),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn ferry_seats() {
        let s = vec![String::from("L.LL.LL.L#")];
        let expected = vec![
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Empty,
            SeatState::Floor,
            SeatState::Empty,
            SeatState::Occupied,
        ];
        assert_eq!(expected, Ferry::from(&s).rows[0]);
    }
    #[test]
    fn check_adjacent_count() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("#L."),
        ];
        let ferry = Ferry::from(&s);
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 1), "Row 1 Seat 1");
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 2), "Row 1 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 3), "Row 1 Seat 3");

        assert_eq!(1, ferry.adjacent_occupied_seats(2, 1), "Row 2 Seat 1");
        assert_eq!(1, ferry.adjacent_occupied_seats(2, 2), "Row 2 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(2, 3), "Row 2 Seat 3");

        assert_eq!(0, ferry.adjacent_occupied_seats(3, 1), "Row 3 Seat 1");
        assert_eq!(1, ferry.adjacent_occupied_seats(3, 2), "Row 3 Seat 2");
        assert_eq!(0, ferry.adjacent_occupied_seats(3, 3), "Row 3 Seat 3");
    }
    #[test]
    fn first_shuffle_all_seats_occupied() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];
        let expected = vec![
            SeatState::Occupied,
            SeatState::Occupied,
            SeatState::Occupied,
        ];

        let mut ferry = Ferry::from(&s);
        let changes = ferry.shuffle();
        //First Shuffle
        assert!(changes, "Expected Changes");
        assert_eq!(expected, ferry.rows[0], "Row 1");
        assert_eq!(expected, ferry.rows[1], "Row 2");
        assert_eq!(expected, ferry.rows[2], "Row 3");
    }
    #[test]
    fn second_shuffle_corners_occupied() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];
        let mut ferry = Ferry::from(&s);
        ferry.shuffle();
        println!("{}", ferry.to_string());
        let changes = ferry.shuffle();
        println!("{}", ferry.to_string());

        println!("{:?}", ferry);

        //Second Shuffle
        assert!(changes, "Expected Changes");
        assert_eq!(
            ferry.rows[0],
            vec![SeatState::Occupied, SeatState::Empty, SeatState::Occupied],
            "Row 1"
        );
        assert_eq!(
            ferry.rows[1],
            vec![SeatState::Empty, SeatState::Empty, SeatState::Empty],
            "Row 2"
        );
        assert_eq!(
            ferry.rows[2],
            vec![SeatState::Occupied, SeatState::Empty, SeatState::Occupied],
            "Row 3"
        );
    }
    #[test]
    fn during_suffle_floors_dont_change() {
        let s = vec![
            String::from("L.L"),
            String::from("L.L"),
            String::from("L.L"),
        ];
        let mut ferry = Ferry::from(&s);
        ferry.shuffle();
        //Second Shuffle
        let expected = vec![SeatState::Occupied, SeatState::Floor, SeatState::Occupied];

        assert_eq!(ferry.rows[0], expected, "Row 1");
        assert_eq!(ferry.rows[1], expected, "Row 2");
        assert_eq!(ferry.rows[2], expected, "Row 3");
    }
    #[test]
    fn run_with_sample_data() {
        let s = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];
        let mut ferry = Ferry::from(&s);
        while ferry.shuffle() {
            println!("{}", ferry.to_string());
        }

        //From Expected Rsults
        let expected_results = vec![
            String::from("#.#L.L#.##"),
            String::from("#LLL#LL.L#"),
            String::from("L.#.L..#.."),
            String::from("#L##.##.L#"),
            String::from("#.#L.LL.LL"),
            String::from("#.#L#L#.##"),
            String::from("..L.L....."),
            String::from("#L#L##L#L#"),
            String::from("#.LLLLLL.L"),
            String::from("#.#L#L#.##"),
        ];
        let expected_ferry = Ferry::from(&expected_results);
        for row in 0..10 {
            assert_eq!(ferry.rows[row], expected_ferry.rows[row], "Row {:?}", row);
        }
        assert_eq!(37, ferry.occupied_seats(), "Seats Wrong");
    }
    #[test]
    fn run_with_sample_data_round_2() {
        let s = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];

        let mut ferry = Ferry::from(&s);
        ferry.covid_adj_threshold = 5;
        ferry.max_distance = 100;

        ferry.shuffle();
        ferry.shuffle();
        let expected = vec![
            String::from("#.LL.LL.L#"),
            String::from("#LLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLL#"),
            String::from("#.LLLLLL.L"),
            String::from("#.LLLLL.L#"),
        ];
        let expected_ferry = Ferry::from(&expected);
        for row in 0..10 {
            assert_eq!(ferry.rows[row], expected_ferry.rows[row], "Row {:?}", row);
        }

        while ferry.shuffle() {
            println!("{}", ferry.to_string());
        }
        assert_eq!(26, ferry.occupied_seats(), "From Sample Data ");
    }

    #[test]
    fn seats_to_check_test() {
        let s = vec![
            String::from("LLL"),
            String::from("LLL"),
            String::from("LLL"),
        ];

        let ferry = Ferry::from(&s);
        assert_eq!(0, ferry.adjacent_occupied_seats(1, 1));
    }

    #[test]
    fn seats_to_check_test_all() {
        let s = vec![
            String::from("###"),
            String::from("###"),
            String::from("###"),
        ];

        let ferry = Ferry::from(&s);
        assert_eq!(3, ferry.adjacent_occupied_seats(1, 1));
        assert_eq!(8, ferry.adjacent_occupied_seats(2, 2));
        assert_eq!(3, ferry.adjacent_occupied_seats(3, 3));
    }
    #[test]
    fn seats_to_check_test_all_true_surrounded() {
        let s = vec![
            String::from("#####"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#...#"),
            String::from("#####"),
        ];

        let mut ferry = Ferry::from(&s);
        ferry.max_distance = 5;

        assert_eq!(8, ferry.adjacent_occupied_seats(3, 3));
        assert_eq!(8, ferry.adjacent_occupied_seats(2, 2));
        assert_eq!(5, ferry.adjacent_occupied_seats(5, 4));
    }
}
