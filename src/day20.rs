#[allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum MatchEdge {
    None,
    Any,
    This(Vec<char>)
}

#[derive(Debug, Clone)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}
impl Edge {
    fn cycle(&self, direction: i8) -> Self {
        if direction == -1 {
            match *self {
                Self::Top => Self::Left,
                Self::Left => Self::Bottom,
                Self::Bottom => Self::Right,
                Self::Right => Self::Top,
            }
        } else {
            match *self {
                Self::Top => Self::Right,
                Self::Right => Self::Bottom,
                Self::Bottom => Self::Left,
                Self::Left => Self::Top,
            }
        }
    }
}
const EDGE_ARRAY: [Edge; 4] = [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Rotation {
    Zero = 0,
    Ninety = 90,
    OneEighty = 180,
    TwoSeventy = 270,
}

impl Rotation {
    fn offset(&self) -> usize {
        match *self {
            Self::Zero => 0,
            Self::Ninety => 1,
            Self::OneEighty => 2,
            Self::TwoSeventy => 3,
        }
    }

    fn next(&self) -> Self {
        match *self {
            Self::Zero => Self::Ninety,
            Self::Ninety => Self::OneEighty,
            Self::OneEighty => Self::TwoSeventy,
            Self::TwoSeventy => Self::Zero,
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    points: Vec<char>,
    width: usize,
    flipped_x: bool,
    flipped_y: bool,
    rotation: Rotation,
}
impl Tile {
    pub fn new(id: u64, width: usize, points: Vec<char>) -> Self {
        let t = Self {
            id: id,
            width: width,
            points: points,
            flipped_x: false,
            flipped_y: false,
            rotation: Rotation::Zero,
        };

        t
    }
    pub fn edge(&self, edge: &Edge) -> Vec<char> {
        let mut return_edge = edge.clone();
        (0..self.rotation.offset()).for_each(|_| return_edge = return_edge.cycle(-1));

        //should reverse
        let reverse = match return_edge {
            Edge::Top | Edge::Bottom => {
                self.rotation == Rotation::OneEighty || self.rotation == Rotation::TwoSeventy
            }
            Edge::Left | Edge::Right => {
                self.rotation == Rotation::Ninety || self.rotation == Rotation::OneEighty
            }
        };
        /*
        println!(
            "Asking for:{:?} Rotation:{:?} Returning {:?} Reversed:{:?}",
            edge, self.rotation, return_edge, reverse
        );
        */

        let mut v = match return_edge {
            Edge::Top => self.top(),
            Edge::Right => self.right(),
            Edge::Bottom => self.bottom(),
            Edge::Left => self.left(),
        };

        if reverse {
            v.reverse();
        }
        v
    }

    fn top(&self) -> Vec<char> {
        self.points
            .iter()
            .take(self.width)
            .map(|x| *x)
            .collect::<Vec<char>>()
    }

    fn bottom(&self) -> Vec<char> {
        let start = self.points.len() - self.width;
        self.points
            .iter()
            .skip(start)
            .map(|x| *x)
            .collect::<Vec<char>>()
    }
    fn left(&self) -> Vec<char> {
        self.points
            .iter()
            .step_by(self.width)
            .map(|x| *x)
            .collect::<Vec<char>>()
    }
    fn right(&self) -> Vec<char> {
        self.points
            .iter()
            .skip(self.width - 1)
            .step_by(self.width)
            .map(|x| *x)
            .collect::<Vec<char>>()
    }

    pub fn matched_edges(&self, tiles: &[Tile]) -> usize {
        let mut matches = vec![];

        for t in tiles {
            if t.id == self.id {
                continue;
            }
            for e in EDGE_ARRAY.iter() {
                let edge_vec = self.edge(e);

                let mut rev_edge_vec = self.edge(e);
                rev_edge_vec.reverse();

                let m = EDGE_ARRAY
                    .iter()
                    .any(|other_e| edge_vec == t.edge(other_e) || rev_edge_vec == t.edge(other_e));

                if m {
                    matches.push(e);
                    break;
                }
            }
        }
        matches.iter().count()
    }

    pub fn match(&mut self, top:MatchEdge ,right:MatchEdge ,bottom:MatchEdge ,left:MatchEdge ) -> Some((Rotation,Rotation))
    {

    }
}

struct Board {
    dimension : usize,
    tile_placement : HashMap<u32, (u64, Rotation)>,
}
impl Board{
    pub fn new(dimension : usize) -> Self {
        Board{
            dimension : dimension, 
            tile_placement :HashMap::<u32, (u64, Rotation)>::new()          
        }
    }

    pub fn place(&self, tiles: Vec<Tiles>){

        let x = self.tile_placement.len() / self.dimension; 
        let y = self.tile_placement.len() % self.dimension;

        let match_top = match y ==0{
            true => MatchEdge::None, 
            false => MatchEdge::Any
        };

        let match_bottom = match y == (self.dimension-1) {
            true => MatchEdge::None, 
            false => MatchEdge::Any
        };
                 ;
         let match_left =match x == 0 {
            true => MatchEdge::None, 
            false => MatchEdge::Any
        };

        let match_right  = match x == (self.dimension-1) {
                true => MatchEdge::None, 
                false => MatchEdge::Any
        };
        let mut to_place = tiles.clone();         
        for t in self.orginal_tiles
        {
            for tp in to_place
            {
                if t.id == tp.id{
                    continue;
                }
                let orientation = t.match(tp, match_top, match_left,match_bottom,match_top);

            }
        }
    }
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<Tile> {
    const WIDTH: usize = 10;
    lazy_static! {
        static ref ID: Regex = Regex::new(r"^Tile\W(\d*):$").unwrap();
        static ref ROW: Regex = Regex::new(r"^([\.#]+)$").unwrap();
    }
    let mut result: HashMap<u64, Vec<char>> = HashMap::new();

    let mut current_id: u64 = 0;
    for line in input.lines() {
        if ID.is_match(line) {
            let caps = ID.captures(line).unwrap();
            current_id = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        } else if ROW.is_match(line) {
            let entry = result.entry(current_id).or_insert(vec![]);
            for c in line.chars() {
                entry.push(c);
            }
        }
    }

    result
        .iter()
        .map(|(k, v)| Tile::new(*k, WIDTH, v.to_vec()))
        .collect::<Vec<Tile>>()
}

#[aoc(day20, part1)]
fn part1(tiles: &[Tile]) -> u64 {
    

    let corner_ids = tiles
        .iter()
        .filter(|t| t.matched_edges(tiles) == 2)
        .map(|t| t.id)
        .collect::<Vec<u64>>();

    let edge_ids = tiles
        .iter()
        .filter(|t| t.matched_edges(tiles) == 3)
        .map(|t| t.id)
        .collect::<Vec<u64>>();

    let center_ids = tiles
        .iter()
        .filter(|t| t.matched_edges(tiles) == 4)
        .map(|t| t.id)
        .collect::<Vec<u64>>();

    println!("Corners: {:?} \n", corner_ids);
    println!("Edges: {:?} \n", edge_ids);
    println!("Centers: {:?} \n", center_ids);

    corner_ids.iter().product::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_first_part() {
        let p = parse_input(&SINGLE);
        assert_eq!(1, p.len());
    }

    #[test]
    fn check_edges_and_rotations() {
        let mut p = parse_input(&SINGLE);
        let mut tile = p.first_mut().unwrap();

        let top: Vec<char> = "..##.#..#.".chars().collect();
        let bottom: Vec<char> = "..###..###".chars().collect();
        let left: Vec<char> = ".#####..#.".chars().collect();
        let right: Vec<char> = "...#.##..#".chars().collect();

        let mut rtop: Vec<char> = top.clone();
        rtop.reverse();

        let mut rbottom: Vec<char> = bottom.clone();
        rbottom.reverse();

        let mut rleft: Vec<char> = left.clone();
        rleft.reverse();

        let mut rright: Vec<char> = right.clone();
        rright.reverse();

        tile.rotation = Rotation::Zero;
        assert_eq!(tile.edge(&Edge::Top), top, "TOP 0");
        assert_eq!(tile.edge(&Edge::Bottom), bottom, "BOTTOM 0");
        assert_eq!(tile.edge(&Edge::Left), left, "LEFT 0");
        assert_eq!(tile.edge(&Edge::Right), right, "RIGHT 0");

        tile.rotation = Rotation::Ninety;
        assert_eq!(tile.edge(&Edge::Top), rleft, "TOP 90");
        assert_eq!(tile.edge(&Edge::Bottom), rright, "BOTTOM 90");
        assert_eq!(tile.edge(&Edge::Left), bottom, "LEFT 90");
        assert_eq!(tile.edge(&Edge::Right), top, "RIGHT 90");

        tile.rotation = Rotation::OneEighty;
        assert_eq!(tile.edge(&Edge::Top), rbottom, "TOP 180");
        assert_eq!(tile.edge(&Edge::Bottom), rtop, "BOTTOM 180");
        assert_eq!(tile.edge(&Edge::Left), rright, "LEFT 180");
        assert_eq!(tile.edge(&Edge::Right), rleft, "RIGHT 180");

        tile.rotation = Rotation::TwoSeventy;
        assert_eq!(tile.edge(&Edge::Top), right, "TOP 270");
        assert_eq!(tile.edge(&Edge::Bottom), left, "BOTTOM 270");
        assert_eq!(tile.edge(&Edge::Left), rtop, "LEFT 270");
        assert_eq!(tile.edge(&Edge::Right), rbottom, "RIGHT 270");
    }

    #[test]
    fn parse_first_example() {
        let p = parse_input(&FIRST_EXAMPLE);
        assert_eq!(9, p.len());
    }
    #[test]
    fn edges_without_match() {
        let p = parse_input(&FIRST_EXAMPLE);
        let tile = p.iter().find(|x| x.id == 1951).unwrap();
        println!("{:?}", tile.matched_edges(&p));
        assert_eq!(2, tile.matched_edges(&p));
    }
    #[test]
    fn edges_with_match() {
        let p = parse_input(&FIRST_EXAMPLE);
        let tile = p.iter().find(|x| x.id == 1427).unwrap();
        println!("{:?}", tile.matched_edges(&p));
        assert_eq!(4, tile.matched_edges(&p));
    }
    #[test]
    fn board() {
        let tiles = parse_input(&FIRST_EXAMPLE);
        let mut board= Board::new(tiles.len()); 
        board.place(&tiles);

        assert_eq!(board.tile_placement.get(&0),(1951,Rotation::OneEighty));

        /*
        1951    2311    3079
        2729    1427    2473
        2971    1489    1171
        */
    }

    #[test]
    fn rotate_test() {
        let s_vec =  vec!("123", "456","789");
        //flip v
        let v = s_vec.iter().map(|s|s.chars().rev().collect::<String>()).collect::<String>();
        assert_eq!(1, v.len());

        
        /*
        1951    2311    3079
        2729    1427    2473
        2971    1489    1171
        */
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
