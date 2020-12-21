#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    const WIDTH: usize = 10;
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
    let mut b = Board::new(&tiles);
    b.place();

    print!("\n");
    for i in 0..((b.dimension * b.dimension) as usize) {
        if i % b.dimension as usize == 0 {
            print!("\n");
        }
        print!(" {:?} ", b.tile_placement.get(i).unwrap().id);
    }
    print!("\n");

    println!(
        " Corners: [{:?},{:?},{:?},{:?}] \n",
        b.tile_placement[0].id,
        b.tile_placement[(b.dimension - 1) as usize].id,
        b.tile_placement[((b.dimension * b.dimension) - (b.dimension)) as usize].id,
        b.tile_placement[((b.dimension * b.dimension) - 1) as usize].id,
    );

    let xxxx: u64 = (b.tile_placement[0].id as u64)
        * (b.tile_placement[(b.dimension - 1) as usize].id as u64)
        * (b.tile_placement[((b.dimension * b.dimension) - (b.dimension)) as usize].id as u64)
        * (b.tile_placement[((b.dimension * b.dimension) - 1) as usize].id as u64);
    xxxx
}

const ITERATE_POSSIBLE_PIVOTS: [Move; 19] = [
    //Normal
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    //Hoirontal Only
    Move::HorizontalFlip,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    //Horizontal and Vertical
    Move::VerticalFlip,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    //Vertical Only
    Move::HorizontalFlip,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
    Move::RotateLeft,
];

const EDGE_ARRAY: [Edge; 4] = [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    HorizontalFlip,
    VerticalFlip,
    RotateLeft,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Edge {
    None,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum MatchEdge {
    None,
    Any,
    This(Vec<char>),
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

            Edge::Right => self
                .coords
                .iter()
                .map(|m| m.chars().nth(0).unwrap())
                .collect::<String>(),
            Edge::Left => self
                .coords
                .iter()
                .map(|m| m.chars().last().unwrap())
                .collect::<String>(),
            _ => unreachable!(),
        }
    }

    pub fn rotate(&self, direction: Move) -> Tile {
        let new_coords = match direction {
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
}

struct Board {
    dimension: u8,
    tiles: Vec<Tile>,
    tile_placement: Vec<Tile>,
}

impl Board {
    pub fn new(tiles: &[Tile]) -> Self {
        let dimension = (tiles.len() as f32).sqrt().abs();

        Board {
            dimension: dimension as u8,
            tiles: tiles.to_vec(),
            tile_placement: vec![],
        }
    }
    pub fn first_corner(&self) -> Option<u16> {
        let ts: Vec<u16> = self
            .tiles
            .iter()
            .filter(|p| p.matched_edges(&self.tiles) == 2)
            .map(|p| p.id)
            .collect();

        if ts.len() > 0 {
            return Some(ts[0]);
        }
        None
    }
    pub fn place(&mut self) {
        let corner_ids = self
            .tiles
            .iter()
            .filter(|t| t.matched_edges(&self.tiles) == 2)
            .map(|t| t.clone())
            .collect::<Vec<Tile>>();

        let edge_ids = self
            .tiles
            .iter()
            .filter(|t| t.matched_edges(&self.tiles) == 3)
            .map(|t| t.clone())
            .collect::<Vec<Tile>>();

        let center_ids = self
            .tiles
            .iter()
            .filter(|t| t.matched_edges(&self.tiles) == 4)
            .map(|t| t.clone())
            .collect::<Vec<Tile>>();

        //Build the Edge of the Puzzle by choosing the first corner
        let first_corner = &corner_ids[0];
        for e in edge_ids.iter() {
            match self.fuse(first_corner.clone(), e.clone(), &Edge::Right) {
                Some((t1, t2)) => {
                    self.tile_placement.push(t1.clone());
                    self.tile_placement.push(t2.clone());
                    break;
                }
                None => continue,
            }
        }

        //Process the Normal Model HERE -- Lets try and first the row
        loop {
            let count = self.tile_placement.len();
            let y = (count / (self.dimension as usize)) as u8;
            let x = (count % (self.dimension as usize)) as u8;

            let mut the_iter = center_ids.iter();
            let mut edge_to_attach = Edge::Right;
            let mut to_attach_to = self.tile_placement.last().unwrap().clone();

            if (y == 0 || y == self.dimension - 1) && (x == 0 || x == self.dimension - 1) {
                the_iter = corner_ids.iter();
            } else if (y == 0 || y == self.dimension - 1) || (x == 0 || x == self.dimension - 1) {
                the_iter = edge_ids.iter();
            }

            if x == 0 {
                //we need to attach to the bottom of the row above
                edge_to_attach = Edge::Bottom;
                to_attach_to = self
                    .tile_placement
                    .get(((y - 1) * self.dimension) as usize)
                    .unwrap()
                    .clone();
            }

            let id = to_attach_to.id;

            for e in the_iter {
                if e.id == id || self.tile_placement.iter().any(|p| p.id == e.id) {
                    continue;
                }

                match self.fuse_one(to_attach_to.clone(), e.clone(), &edge_to_attach) {
                    Some(t2) => {
                        self.tile_placement.push(t2.clone());
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

    fn fuse_one(&self, t1: Tile, t2: Tile, edge_from: &Edge) -> Option<Tile> {
        let mut t2 = t2.clone();

        let edge_we_need_to_find = match edge_from {
            Edge::Top => Edge::Bottom,
            Edge::Bottom => Edge::Top,
            Edge::Left => Edge::Right,
            Edge::Right => Edge::Left,
            _ => unreachable!(),
        };

        for p1 in ITERATE_POSSIBLE_PIVOTS.iter() {
            if t1.edge(edge_from) == t2.edge(&edge_we_need_to_find) {
                return Some(t2);
            }
            t2 = t2.rotate(*p1);
        }

        return None;
    }

    fn fuse(&self, t1: Tile, t2: Tile, edge_from: &Edge) -> Option<(Tile, Tile)> {
        let edge_to = match edge_from {
            Edge::Top => Edge::Bottom,
            Edge::Bottom => Edge::Bottom,
            Edge::Left => Edge::Right,
            Edge::Right => Edge::Left,
            _ => unreachable!(),
        };

        let mut t1 = t1.clone();
        let mut t2 = t2.clone();

        for p in ITERATE_POSSIBLE_PIVOTS.iter() {
            for p1 in ITERATE_POSSIBLE_PIVOTS.iter() {
                if t1.edge(edge_from) == t2.edge(&edge_to) {
                    return Some((t1, t2));
                }
                t2 = t2.rotate(*p1);
            }
            t1 = t1.rotate(*p);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_test() {
        let at = Tile::new(
            1,
            vec!["123".to_string(), "456".to_string(), "789".to_string()],
        );

        let mut t = at.rotate(Move::HorizontalFlip);
        assert_eq!(t.coords, vec!["321", "654", "987"]);

        t = at.rotate(Move::VerticalFlip);
        assert_eq!(t.coords, vec!["789", "456", "123"]);

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
        assert_eq!(t.edge(&Edge::Right), "147");
        assert_eq!(t.edge(&Edge::Left), "369");
    }
    #[test]
    fn place_test() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut b = Board::new(&tiles);
        b.place();
        assert_eq!(9, b.tile_placement.len());
    }
    #[test]
    fn firse_corner_test() {
        let tiles = parse_input(&FIRST_EXAMPLE);

        let mut b = Board::new(&tiles);
        match b.first_corner() {
            Some(n) => assert!(n == 1951_u16 || n == 3079_u16 || n == 2971_u16 || n == 1171_u16),
            None => assert_eq!("", "1", "NO CORNERS FOUND"),
        }
    }

    lazy_static! {
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
