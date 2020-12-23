#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
// import commonly used items from the prelude:
use rand::prelude::*;

/*
THERE AT LEAST 2 BUGS IN THIS CODE

1. The loader isn't always working as its not joining the second row edges properly.
2. There's duplicate found on the dragons code - so  I used a hashmap to ripthem out.

*/

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    lazy_static! {
        static ref ID: Regex = Regex::new(r"^Tile\W(\d*):$").unwrap();
        static ref ROW: Regex = Regex::new(r"^([\.#]+)$").unwrap();
    }

    let mut result: HashMap<u16, Vec<String>> = HashMap::new();

    let mut current_id: u16 = 0;
    for line in input.lines() {
        if ID.is_match(line) {
            let caps = ID.captures(line).unwrap();
            current_id = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
        } else if ROW.is_match(line) {
            let entry = result.entry(current_id).or_insert(vec![]);
            entry.push(line.to_string());
        }
    }

    result
        .iter()
        .map(|(k, v)| Tile::new(*k, v.to_vec()))
        .collect::<Vec<Tile>>()
}
#[aoc(day20, part1)]
fn part1(tiles: &[Tile]) -> u64 {
    let dimension = (tiles.len() as f32).sqrt().abs() as u8;

    let mut b = Board::new(dimension);
    let mut counter = 0;
    while b.tile_placement.len() != tiles.len() {
        b.place(&tiles);
        counter += 1;
        if counter == 100 {
            panic!("canoot place the tiles")
        }
    }
    b.place(&tiles);

    println!("{}", b.print_tiles());

    let xxxx: u64 = (b.tile_placement[0].id as u64)
        * (b.tile_placement[(b.dimension - 1) as usize].id as u64)
        * (b.tile_placement[((b.dimension * b.dimension) - (b.dimension)) as usize].id as u64)
        * (b.tile_placement[((b.dimension * b.dimension) - 1) as usize].id as u64);
    xxxx
}

#[aoc(day20, part2)]
fn part2(tiles: &[Tile]) -> usize {
    let dimension = (tiles.len() as f32).sqrt().abs() as u8;
    let mut b = Board::new(dimension);
    let mut counter = 0;
    while b.tile_placement.len() != tiles.len() {
        b.place(&tiles);
        counter += 1;
        if counter == 100 {
            panic!("canoot place the tiles")
        }
    }
    //println!("{}", b.print_tiles());

    let mut tile: Tile = b.to_image_tile();
    let mut dragon_coords = HashMap::new();
    let mut dragon_coords2 = vec![];

    for tt in ITERATE_POSSIBLE_PIVOTS.iter() {
        tile = tile.rotate(*tt);
        dragon_coords.clear();

        for y in 1..tile.coords.len() - 1 {
            let prev_line = tile.coords.get(y - 1).unwrap();
            let this_line = tile.coords.get(y).unwrap();
            let next_line = tile.coords.get(y + 1).unwrap();
            let mut start_index = 0;
            let mut stop = false;
            while !stop {
                match check_line_sea_dragon(start_index, this_line, prev_line, next_line) {
                    Some(x) => {
                        start_index = start_index + 20;
                        dragon_coords2.push(format!("{:#2}_{:#2}", y, x));
                        dragon_coords.insert(format!("{}_{}", y, x), (y, x));
                    }
                    None => {
                        stop = true;
                    }
                }
            }
        }

        if 0 != dragon_coords.len() {
            break;
        }
    }

    let hashes = tile
        .coords
        .iter()
        .map(|line| line.chars().filter(|c| c == &'#').count())
        .sum::<usize>();

    dragon_coords2.sort();
    println!("Dragons 2: {:?}", dragon_coords2);

    println!("Dragons: {:?}", dragon_coords);
    println!("Dragons: {:?}", dragon_coords.len());
    println!("Hashes: {:?}", hashes);
    println!(
        "Hashes - Dragons: {:?}",
        hashes - (dragon_coords.len() * 15)
    );
    hashes - (dragon_coords.len() * 15)
}

fn check_line_sea_dragon(
    start_index: usize,
    this_line: &str,
    prev: &str,
    next: &str,
) -> Option<usize> {
    const PREV_MATCHES: [usize; 1] = [18];
    const THIS_MATCHES: [usize; 8] = [0, 5, 6, 11, 12, 17, 18, 19];
    const NEXT_MATCHES: [usize; 6] = [1, 4, 7, 10, 13, 16];
    let prev_v: Vec<char> = prev.chars().collect();
    let this_v: Vec<char> = this_line.chars().collect();
    let next_v: Vec<char> = next.chars().collect();
    /*
    ..................#.
    #....##....##....###
    .#..#..#..#..#..#...
    */
    for i in start_index..(prev_v.len() - 18) {
        let this_m = THIS_MATCHES.iter().all(|ci| match this_v.get(i + ci) {
            None => false,
            Some(c) => c == &'#',
        });

        let prev_m = PREV_MATCHES.iter().all(|ci| match prev_v.get(i + ci) {
            None => false,
            Some(c) => c == &'#',
        });

        let next_m = NEXT_MATCHES.iter().all(|ci| match next_v.get(i + ci) {
            None => false,
            Some(c) => c == &'#',
        });

        if this_m && prev_m && next_m {
            return Some(i);
        }
    }
    None
}

const ITERATE_POSSIBLE_PIVOTS: [Move; 12] = [
    //keeps things the same
    Move::Clone,
    //Normal
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    //Hoirontal Only
    Move::HorizontalFlip,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    //Vertical
    Move::VerticalFlip,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
];

const EDGE_ARRAY: [Edge; 4] = [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    Clone,
    HorizontalFlip,
    RotateLeft,
    VerticalFlip,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    coords: Vec<String>,
}

impl Tile {
    pub fn new(id: u16, coords: Vec<String>) -> Self {
        Self {
            id: id,
            coords: coords,
        }
    }

    pub fn edge(&self, edge: &Edge) -> String {
        match edge {
            Edge::Top => self.coords.first().unwrap().to_string(),
            Edge::Bottom => self.coords.last().unwrap().to_string(),

            Edge::Left => self
                .coords
                .iter()
                .map(|m| m.chars().nth(0).unwrap())
                .collect::<String>(),
            Edge::Right => self
                .coords
                .iter()
                .map(|m| m.chars().last().unwrap())
                .collect::<String>(),
        }
    }

    pub fn rotate(&self, direction: Move) -> Tile {
        let new_coords = match direction {
            Move::Clone => self.coords.clone(),
            Move::HorizontalFlip => self
                .coords
                .iter()
                .map(|s| s.chars().rev().collect::<String>())
                .collect(),
            Move::VerticalFlip => self.coords.iter().rev().map(|s| s.to_string()).collect(),

            Move::RotateLeft => {
                let mut rotated = vec![];
                for i in 0..self.coords.len() {
                    let mut s = String::new();
                    for x in self.coords.iter().rev() {
                        s.push(x.chars().nth(i).unwrap());
                    }
                    rotated.push(s);
                }
                rotated
            }
        };

        Self::new(self.id, new_coords)
    }

    pub fn matched_edges(&self, tiles: &[Tile]) -> usize {
        let mut matches = vec![];

        for t in tiles {
            if t.id == self.id {
                continue;
            }
            for e in EDGE_ARRAY.iter() {
                let edge_str = self.edge(e);
                let rev_edge_str = self.edge(e).chars().rev().collect::<String>();

                let m = EDGE_ARRAY
                    .iter()
                    .any(|other_e| edge_str == t.edge(other_e) || rev_edge_str == t.edge(other_e));

                if m {
                    matches.push(e);
                    break;
                }
            }
        }
        matches.iter().count()
    }
    #[allow(dead_code)]
    fn print_tile(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!("{:#2}: ", self.id));
        s.push('\n');

        for ss in self.coords.iter() {
            s.push_str(&ss);
            s.push('\n');
        }
        s
    }
}

struct Board {
    dimension: u8,
    tile_placement: Vec<Tile>,
}

impl Board {
    pub fn new(dimension: u8) -> Self {
        Board {
            dimension: dimension as u8,
            tile_placement: vec![],
        }
    }

    fn print_tiles(&self) -> String {
        let mut s = String::new();

        for (i, v) in self.tile_placement.iter().enumerate() {
            if i % self.dimension as usize == 0 {
                s.push('\n');
            }
            s.push_str(&format!("{:?}:{:#2}, ", i, v.id));
        }
        s.push('\n');

        let dim = self.dimension as usize;
        //tiles all have the same dimensions and remove the border
        let rows_per_tile = self.tile_placement.get(0).unwrap().coords.len();
        let total_rows = dim * rows_per_tile;
        let mut image: Vec<String> = (1..=total_rows).map(|_| String::new()).collect();

        //Loop through all the tiles that we have placed
        for (index, tile) in self.tile_placement.iter().enumerate() {
            //there's 8 rows per group and the number of groups
            let my_block_index = (index / dim) * rows_per_tile;
            for (ci, coord) in tile.coords.iter().skip(1).enumerate().take(rows_per_tile) {
                if let Some(elem) = image.get_mut(my_block_index + ci) {
                    //elem.push_str(&format!("{}", tile.id));
                    elem.push_str(&coord);
                    elem.push(' ');
                }
            }
        }

        s.push('\n');
        for ss in image.iter() {
            s.push_str(&ss);
            s.push('\n');
        }

        s
    }

    fn place(&mut self, tiles: &[Tile]) {
        let mut to_place = tiles.to_vec();

        self.tile_placement.clear();
        if let Some((id1, id2)) = self.place_first_corner(&to_place) {
            to_place.retain(|x| x.id != id1 && x.id != id2);
        }

        //Process the Normal Model HERE -- Lets try and first the row
        loop {
            let count = self.tile_placement.len();

            let (top_edge, left_edge) = self.get_adj_edges();
            for e in &to_place {
                match self.fuse(&top_edge, &left_edge, &e) {
                    Some(t2) => {
                        self.tile_placement.push(t2.clone());
                        to_place.retain(|x| x.id != t2.id);
                        break;
                    }
                    None => (),
                }
            }

            if count == self.tile_placement.len() {
                break;
            }
        }
    }

    fn place_first_corner(&mut self, to_place: &[Tile]) -> Option<(u16, u16)> {
        let corner_ids = to_place
            .iter()
            .filter(|t| t.matched_edges(to_place) == 2)
            .map(|t| t.clone())
            .collect::<Vec<Tile>>();
        let edge_ids = to_place
            .iter()
            .filter(|t| t.matched_edges(to_place) == 3)
            .map(|t| t.clone())
            .collect::<Vec<Tile>>();

        //Build the Edge of the Puzzle by choosing the first corner
        let x: u32 = random();
        let a_corner = corner_ids.get((x % 4) as usize).unwrap();

        let mut c = a_corner.clone();
        for p in ITERATE_POSSIBLE_PIVOTS.iter() {
            c = c.rotate(*p);
            let left_edge_must_be = Some(c.edge(&Edge::Right));

            for e in &edge_ids {
                if let Some(t2) = self.fuse(&None, &left_edge_must_be, &e) {
                    self.tile_placement.push(c.clone());
                    self.tile_placement.push(t2.clone());

                    return Some((c.id, t2.id));
                }
            }
        }
        None
    }

    fn get_adj_edges(&self) -> (Option<String>, Option<String>) {
        let count = self.tile_placement.len();
        let y = (count / (self.dimension as usize)) as u8;
        let x = (count % (self.dimension as usize)) as u8;

        let top_edge = match y == 0 {
            true => None,
            false => {
                let above = self
                    .tile_placement
                    .get(count - self.dimension as usize)
                    .unwrap();
                Some(above.edge(&Edge::Bottom))
            }
        };

        let left_edge = match x == 0 {
            true => None,
            false => {
                let beside = self.tile_placement.last().unwrap();
                Some(beside.edge(&Edge::Right))
            }
        };
        (top_edge, left_edge)
    }

    fn fuse(&self, top: &Option<String>, left: &Option<String>, t2: &Tile) -> Option<Tile> {
        let mut t2 = t2.clone();

        for p1 in ITERATE_POSSIBLE_PIVOTS.iter() {
            t2 = t2.rotate(*p1);
            let top_match = match top {
                Some(top_edge) => top_edge == &t2.edge(&Edge::Top),
                None => true,
            };

            let left_match = match left {
                Some(left_edge) => left_edge == &t2.edge(&Edge::Left),
                None => true,
            };

            if top_match && left_match {
                return Some(t2);
            }
        }

        return None;
    }

    fn to_image_tile(&self) -> Tile {
        let dim = self.dimension as usize;
        //tiles all have the same dimensions and remove the border
        let rows_per_tile = self.tile_placement.get(0).unwrap().coords.len() - 2;
        let total_rows = dim * rows_per_tile;
        let mut image: Vec<String> = (1..=total_rows).map(|_| String::new()).collect();

        //Loop through all the tiles that we have placed
        for (index, tile) in self.tile_placement.iter().enumerate() {
            //there's 8 rows per group and the number of groups
            let my_block_index = (index / dim) * rows_per_tile;

            for (ci, coord) in tile.coords.iter().skip(1).enumerate().take(rows_per_tile) {
                if let Some(elem) = image.get_mut(my_block_index + ci) {
                    elem.push_str(&coord[1..coord.len() - 1])
                }
            }
        }
        Tile::new(2456, image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragons() {
        let mut tile: Tile = test_get_top_match().unwrap();
        let mut dragon_coords = vec![];

        for tt in ITERATE_POSSIBLE_PIVOTS.iter() {
            tile = tile.rotate(*tt);

            for y in 1..tile.coords.len() - 1 {
                let prev_line = tile.coords.get(y - 1).unwrap();
                let this_line = tile.coords.get(y).unwrap();
                let next_line = tile.coords.get(y + 1).unwrap();
                match check_line_sea_dragon(0, this_line, prev_line, next_line) {
                    Some(x) => {
                        dragon_coords.push((x, y));
                        println!("ADDED {:?} {:?}", x, y);
                    }
                    None => (),
                }
            }
            if 0 != dragon_coords.len() {
                break;
            }
        }
        let hashes = tile
            .coords
            .iter()
            .map(|line| line.chars().filter(|c| c == &'#').count())
            .sum::<usize>();

        assert_eq!(273, hashes - (dragon_coords.len() * 15));
        assert_eq!(2, dragon_coords.len());
    }

    #[test]
    fn parse_test() {
        let s: String = [
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
        ]
        .join("\n");
        let v = parse_input(&s);
        let t = v.get(0).unwrap();

        println!("{}", t.print_tile());
        assert_eq!(t.edge(&Edge::Top), "#.##...##.", "1111111111111111111111");

        let t = t.rotate(Move::RotateLeft);
        println!("{}", t.print_tile());
        assert_eq!(t.edge(&Edge::Right), "#.##...##.", "2222222222222222");

        let t = t.rotate(Move::RotateLeft);
        println!("{}", t.print_tile());
        assert_eq!(t.edge(&Edge::Bottom), ".##...##.#", "3333333333333333333");

        let t = t.rotate(Move::RotateLeft);
        println!("{}", t.print_tile());
        assert_eq!(t.edge(&Edge::Left), ".##...##.#", "444444444444444");

        let t = t.rotate(Move::RotateLeft);
        println!("{}", t.print_tile());
        assert_eq!(t.edge(&Edge::Top), "#.##...##.", "555555");
    }
    #[test]
    fn rotate_test() {
        let at = Tile::new(
            1,
            vec!["123".to_string(), "456".to_string(), "789".to_string()],
        );

        let mut t = at.rotate(Move::HorizontalFlip);
        assert_eq!(t.coords, vec!["321", "654", "987"]);

        t = at.rotate(Move::RotateLeft);
        assert_eq!(t.coords, vec!["741", "852", "963"]);
    }
    #[test]
    fn edge_test() {
        let t = Tile::new(
            1,
            vec!["123".to_string(), "456".to_string(), "789".to_string()],
        );
        assert_eq!(t.edge(&Edge::Top), "123");
        assert_eq!(t.edge(&Edge::Bottom), "789");
        assert_eq!(t.edge(&Edge::Left), "147");
        assert_eq!(t.edge(&Edge::Right), "369");
    }
    #[test]
    fn place_test() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut b = Board::new(3);
        let mut counter = 0;
        while b.tile_placement.len() != 9 {
            b.place(&tiles);
            counter += 1;
            if counter == 10 {
                break;
            }
        }

        println!("{}", b.print_tiles());

        assert_eq!(9, b.tile_placement.len());
    }
    #[test]
    fn firse_corner_test() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let corner_ids = tiles
            .iter()
            .filter(|t| t.matched_edges(&tiles) == 2)
            .map(|t| t.id)
            .collect::<Vec<u16>>();

        assert!(corner_ids.contains(&1951));
        assert!(corner_ids.contains(&3079));
        assert!(corner_ids.contains(&2971));
        assert!(corner_ids.contains(&1171));
    }
    #[test]
    fn edges() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut b = Board::new(3);
        let mut counter = 0;
        while b.tile_placement.len() != 9 {
            b.place(&tiles);
            counter += 1;
            if counter == 10 {
                break;
            }
        }

        println!("{}", b.print_tiles());
        println!("{:?}", b.tile_placement.get(0).unwrap().edge(&Edge::Right));
        println!("{:?}", b.tile_placement.get(1).unwrap().edge(&Edge::Left));

        assert_eq!(
            b.tile_placement.get(0).unwrap().edge(&Edge::Right),
            b.tile_placement.get(1).unwrap().edge(&Edge::Left),
            "0->1"
        );
        assert_eq!(
            b.tile_placement.get(1).unwrap().edge(&Edge::Right),
            b.tile_placement.get(2).unwrap().edge(&Edge::Left),
            "1->2"
        );

        println!(
            "www-{:?}",
            b.tile_placement.get(3).unwrap().edge(&Edge::Top)
        );
        println!(
            "yyy-{:?}",
            b.tile_placement.get(0).unwrap().edge(&Edge::Bottom)
        );
        assert_eq!(
            b.tile_placement.get(3).unwrap().edge(&Edge::Top),
            b.tile_placement.get(0).unwrap().edge(&Edge::Bottom),
            "3T->0B"
        );

        assert_eq!(
            b.tile_placement.get(4).unwrap().edge(&Edge::Top),
            b.tile_placement.get(1).unwrap().edge(&Edge::Bottom),
            "1->1"
        );
        assert_eq!(
            b.tile_placement.get(4).unwrap().edge(&Edge::Left),
            b.tile_placement.get(3).unwrap().edge(&Edge::Right),
            "1->3"
        );
        assert_eq!(
            b.tile_placement.get(4).unwrap().edge(&Edge::Right),
            b.tile_placement.get(5).unwrap().edge(&Edge::Left),
            "1->5"
        );
        assert_eq!(
            b.tile_placement.get(4).unwrap().edge(&Edge::Bottom),
            b.tile_placement.get(7).unwrap().edge(&Edge::Top),
            "1->7"
        );
    }
    #[test]
    fn tile_image_top() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut b = Board::new(3);
        let mut counter = 0;
        while b.tile_placement.len() != 9 {
            b.place(&tiles);
            counter += 1;
            if counter == 10 {
                break;
            }
        }
        println!("{}", b.print_tiles());

        let mut matched = false;
        let expected = EXPECTED_IMAGE.lines().next().unwrap();

        let mut t: Tile = b.to_image_tile();

        for tt in ITERATE_POSSIBLE_PIVOTS.iter() {
            println!("ROTATED: {:?}", tt);
            t = t.rotate(*tt);

            if t.coords.iter().any(|c| c == expected) {
                matched = true;
                break;
            }
        }

        assert!(
            matched,
            "We couldnt pivot the image to match the desired row"
        );
    }

    fn test_get_top_match() -> Option<Tile> {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut b = Board::new(3);
        let mut counter = 0;
        while b.tile_placement.len() != 9 {
            b.place(&tiles);
            counter += 1;
            if counter == 10 {
                break;
            }
        }
        let expected = EXPECTED_IMAGE.lines().next().unwrap();

        let mut t: Tile = b.to_image_tile();

        for tt in ITERATE_POSSIBLE_PIVOTS.iter() {
            t = t.rotate(*tt);

            if t.coords.get(0).unwrap() == expected {
                return Some(t);
            }
        }
        None
    }

    /*
    ..................#.
    #....##....##....###
    .#..#..#..#..#..#...
    */

    lazy_static! {
        static ref EXPECTED_IMAGE: String = [
            ".#.#..#.##...#.##..#####",
            "###....#.#....#..#......",
            "##.##.###.#.#..######...",
            "###.#####...#.#####.#..#",
            "##.#....#.##.####...#.##",
            "...########.#....#####.#",
            "....#..#...##..#.#.###..",
            ".####...#..#.....#......",
            "#..#.##..#..###.#.##....",
            "#.####..#.####.#.#.###..",
            "###.#.#...#.######.#..##",
            "#.####....##..########.#",
            "##..##.#...#...#.#.#.#..",
            "...#..#..#.#.##..###.###",
            ".#.#....#.##.#...###.##.",
            "###.#...#..#.##.######..",
            ".#.#.###.##.##.#..#.##..",
            ".####.###.#...###.#..#.#",
            "..#.#..#..#.#.#.####.###",
            "#..####...#.#.#.###.###.",
            "#####..#####...###....##",
            "#.##..#..#...#..####...#",
            ".#.###..##..##..####.##.",
            "...###...##...#...#..###",
        ]
        .join("\n");
        static ref SINGLE: String = [
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
        ]
        .join("\n");
        static ref FIRST_EXAMPLE: String = [
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###..."
        ]
        .join("\n");
    }
}
