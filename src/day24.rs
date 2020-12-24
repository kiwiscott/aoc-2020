use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
//use std::slice::Iter;

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Directions> {
    input
        .lines()
        .filter_map(|s| {
            let mut directions = Directions::new();
            let mut prev: Option<char> = None;
            for c in s.chars() {
                if c == 'e' || c == 'w' {
                    let dir = match (prev, c) {
                        (None, 'e') => Some(Direction::East),
                        (Some('n'), 'e') => Some(Direction::NorthEast),
                        (Some('s'), 'e') => Some(Direction::SouthEast),
                        (None, 'w') => Some(Direction::West),
                        (Some('n'), 'w') => Some(Direction::NorthWest),
                        (Some('s'), 'w') => Some(Direction::SouthWest),
                        _ => None,
                    };

                    if let Some(direction) = dir {
                        directions.push(direction);
                    }
                    prev = None;
                } else {
                    prev = Some(c);
                }
            }

            match directions.len() {
                0 => None,
                _ => Some(directions),
            }
        })
        .collect::<Vec<Directions>>()
}

type Directions = Vec<Direction>;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x: x, y: y, z: z }
    }

    fn zero() -> Self {
        Point { x: 0, y: 0, z: 0 }
    }
    fn adjacent_points(&self) -> Vec<Point> {
        [
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
        .iter()
        .map(|d| self.adjacent_point(*d))
        .collect::<Vec<Point>>()
    }
    fn adjacent_point(&self, direction: Direction) -> Self {
        self.offset_point(&[direction])
    }

    fn offset_point(&self, directions: &[Direction]) -> Self {
        let (mut x, mut y, mut z) = (0, 0, 0);

        for direction in directions {
            match direction {
                Direction::East => {
                    x += -1;
                    y += 1;
                    z += 0;
                }
                Direction::SouthEast => {
                    x += -1;
                    y += 0;
                    z += 1;
                }
                Direction::SouthWest => {
                    x += 0;
                    y += -1;
                    z += 1;
                }
                Direction::West => {
                    x += 1;
                    y += -1;
                    z += 0;
                }
                Direction::NorthWest => {
                    x += 1;
                    y += 0;
                    z += -1;
                }
                Direction::NorthEast => {
                    x += 0;
                    y += 1;
                    z += -1;
                }
            }
        }

        Point::new(self.x + x, self.y + y, self.z + z)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[aoc(day24, part1)]
fn part1(directions: &[Directions]) -> usize {
    let mut black_tiles = HashSet::new();

    for d in directions {
        let offset = Point::zero().offset_point(d);
        if !black_tiles.remove(&offset) {
            black_tiles.insert(offset);
        }
    }

    black_tiles.len()
}

#[aoc(day24, part2)]
fn part2(directions: &[Directions]) -> usize {
    //Day One create all tiles
    let mut black_tiles = HashSet::new();

    for d in directions {
        let offset = Point::zero().offset_point(d);
        if !black_tiles.remove(&offset) {
            black_tiles.insert(offset);
        }
    }
    for _i in 1..=100 {
        //let x = black_tiles.len();
        //Add all the tiles that are not currently accounted for

        //existing_tiles
        let existing_tiles: HashSet<Point> = black_tiles
            .iter()
            .filter(|point| {
                let adjacent_black = point
                    .adjacent_points()
                    .iter()
                    .filter(|p| black_tiles.contains(p))
                    .count();
                /*
                Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                */
                adjacent_black == 1
            })
            .map(|p| p.clone())
            .collect();

        let new_points: HashSet<Point> = black_tiles
            .iter()
            .flat_map(|point| point.adjacent_points())
            .filter(|point| !existing_tiles.contains(point))
            .filter(|point| {
                /*
                Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                */
                point
                    .adjacent_points()
                    .iter()
                    .filter(|p| black_tiles.contains(p))
                    .count()
                    == 2
            })
            .collect();

        black_tiles = existing_tiles
            .iter()
            .chain(new_points.iter())
            .map(|p| *p)
            .collect();
    }

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(2208, part2(&data));
    }

    #[test]
    fn test_part_1() {
        let data = parse_input(&SAMPLE_DATA);

        assert_eq!(10, part1(&data));
    }

    #[test]
    fn parse_navigate() {
        let data = parse_input(&SAMPLE_ROW);
        let point = Point::zero().offset_point(&data[0]);
        assert_eq!(Point::new(0, -3, 3), point);
    }

    #[test]
    fn navigate() {
        let steps = [
            (-1, 0, 1, Direction::SouthEast),
            (-1, -1, 2, Direction::SouthWest),
            (-1, 0, 1, Direction::NorthEast),
            (-1, -1, 2, Direction::SouthWest),
            (-1, -2, 3, Direction::SouthWest),
            (-2, -2, 4, Direction::SouthEast),
            (-1, -2, 3, Direction::NorthWest),
            (0, -3, 3, Direction::West),
            (1, -3, 2, Direction::NorthWest),
            (0, -3, 3, Direction::SouthEast),
        ];

        let mut offset_point = Point::zero();
        for (x, y, z, direction) in steps.iter() {
            let point = Point::new(*x, *y, *z);

            offset_point = offset_point.adjacent_point(*direction);

            assert_eq!(
                point, offset_point,
                "Step failed. Added {:?} Expected {:?}",
                direction, point
            );
        }
    }

    #[test]
    fn offset() {
        assert_eq!(
            Point::new(-1, 1, 0),
            Point::zero().adjacent_point(Direction::East)
        );
        assert_eq!(
            Point::new(-1, 0, 1),
            Point::zero().adjacent_point(Direction::SouthEast)
        );
        assert_eq!(
            Point::new(0, -1, 1),
            Point::zero().adjacent_point(Direction::SouthWest)
        );
        assert_eq!(
            Point::new(1, -1, 0),
            Point::zero().adjacent_point(Direction::West)
        );
        assert_eq!(
            Point::new(1, 0, -1),
            Point::zero().adjacent_point(Direction::NorthWest)
        );
        assert_eq!(
            Point::new(0, 1, -1),
            Point::zero().adjacent_point(Direction::NorthEast)
        );
    }

    #[test]
    fn parse_data() {
        let data = parse_input(&SAMPLE_ROW);
        //let x = part1(&data);
        assert_eq!(
            data[0],
            [
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::NorthEast,
                Direction::SouthWest,
                Direction::SouthWest,
                Direction::SouthEast,
                Direction::NorthWest,
                Direction::West,
                Direction::NorthWest,
                Direction::SouthEast,
            ]
        );
    }

    lazy_static! {
        static ref SAMPLE_ROW: String = ["seswneswswsenwwnwse", "",].join("\n");
        static ref SAMPLE_DATA: String = [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
            "",
        ]
        .join("\n");
    }
}
