use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::ops::Range;

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<Cube> {
    let (w, z) = (0, 0);
    let mut pocket = vec![];
    let mut y = 0;
    let mut x = 0;

    for line in input.lines() {
        for c in line.chars() {
            let cube = match c {
                '.' => (Coordinate::new(x, y, z, w), INACTIVE),
                '#' => (Coordinate::new(x, y, z, w), ACTIVE),
                _ => {
                    println!("{:?}", c);
                    unreachable!()
                }
            };
            pocket.push(cube);
            x = x + 1;
        }
        y = y + 1;
        x = 0;
    }
    pocket
}
#[aoc(day17, part1)]
fn part1(cubes: &Vec<Cube>) -> i32 {
    let mut dimension = PocketDimension {
        cubes: cubes.clone(),
        dimensions: 3,
    };
    for _ in 1..=6 {
        dimension.cycle();
    }
    dimension.active_cubes()
}

#[aoc(day17, part2)]
fn part2(cubes: &Vec<Cube>) -> i32 {
    let mut dimension = PocketDimension {
        cubes: cubes.clone(),
        dimensions: 4,
    };
    for _ in 1..=6 {
        dimension.cycle();
    }
    dimension.active_cubes()
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
impl Coordinate {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.w.cmp(&other.w) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.z.cmp(&other.z) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match self.y.cmp(&other.y) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => self.x.cmp(&other.x),
                },
            },
        }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.z == other.z && self.y == other.y && self.x == other.x
    }
}

impl Eq for Coordinate {}

const ACTIVE: bool = true;
const INACTIVE: bool = false;
type Cube = (Coordinate, bool);
struct PocketDimension {
    cubes: Vec<Cube>,
    dimensions: i32,
}

impl PocketDimension {
    fn cycle(&mut self) {
        let min = self.min_cordinate().unwrap();
        let max = self.max_cordinate().unwrap();

        let mut new_cube = vec![];

        for w in self.range_for('w', min, max) {
            for z in self.range_for('z', min, max) {
                for y in self.range_for('y', min, max) {
                    for x in self.range_for('x', min, max) {
                        let c = match self.active_neighbours(x, y, z, w) {
                            3 => (Coordinate::new(x, y, z, w), ACTIVE),
                            2 => (Coordinate::new(x, y, z, w), self.is_active(x, y, z, w)),
                            _ => (Coordinate::new(x, y, z, w), INACTIVE),
                        };
                        new_cube.push(c);
                    }
                }
            }
        }
        self.cubes = new_cube;
    }
    fn range_for(&self, dimension: char, min: Coordinate, max: Coordinate) -> Range<i32> {
        let mut r = Range { start: 0, end: 1 };

        match dimension {
            'w' => {
                if self.dimensions >= 4 {
                    r.start = min.w - 1;
                    r.end = max.w + 1 + 1;
                }
            }
            'z' => {
                if self.dimensions >= 3 {
                    r.start = min.z - 1;
                    r.end = max.z + 1 + 1;
                }
            }
            'y' => {
                if self.dimensions >= 2 {
                    r.start = min.y - 1;
                    r.end = max.y + 1 + 1;
                }
            }
            'x' => {
                if self.dimensions >= 2 {
                    r.start = min.x - 1;
                    r.end = max.x + 1 + 1;
                }
            }
            _ => (),
        }
        return r;
    }

    fn is_active(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        self.cubes
            .iter()
            .any(|(co, st)| *st && (co.w == w && co.x == x && co.y == y && co.z == z))
    }
    fn active_cubes(&self) -> i32 {
        self.cubes.iter().filter(|(_, st)| *st).count() as i32
    }

    fn min_cordinate(&self) -> Option<Coordinate> {
        match self.cubes.iter().min() {
            Some(c) => Some(c.0),
            None => None,
        }
    }
    fn max_cordinate(&self) -> Option<Coordinate> {
        match self.cubes.iter().max() {
            Some(c) => Some(c.0),
            None => None,
        }
    }

    fn active_neighbours(&self, x: i32, y: i32, z: i32, w: i32) -> i32 {
        self.cubes
            .iter()
            .filter(|(co, st)| {
                *st && (self.dimensions < 1||(co.x >= x - 1 && co.x <= x + 1 ))
                    && (self.dimensions < 2||(co.y >= y - 1 && co.y <= y + 1 ))
                    && (self.dimensions < 3||(co.z >= z - 1 && co.z <= z + 1 ))
                    && (self.dimensions < 4||(co.w >= w - 1 && co.w <= w + 1 ))
                    // Exclude the same item
                    && ! (co.x == x && co.y == y && co.z == z && co.w == w )
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = ".#.\n..#\n###";
        let expected = [
            (
                Coordinate {
                    x: 0,
                    y: 0,
                    z: 0,
                    w: 0,
                },
                false,
            ),
            (
                Coordinate {
                    x: 1,
                    y: 0,
                    z: 0,
                    w: 0,
                },
                true,
            ),
            (
                Coordinate {
                    x: 2,
                    y: 0,
                    z: 0,
                    w: 0,
                },
                false,
            ),
            (
                Coordinate {
                    x: 0,
                    y: 1,
                    z: 0,
                    w: 0,
                },
                false,
            ),
            (
                Coordinate {
                    x: 1,
                    y: 1,
                    z: 0,
                    w: 0,
                },
                false,
            ),
            (
                Coordinate {
                    x: 2,
                    y: 1,
                    z: 0,
                    w: 0,
                },
                true,
            ),
            (
                Coordinate {
                    x: 0,
                    y: 2,
                    z: 0,
                    w: 0,
                },
                true,
            ),
            (
                Coordinate {
                    x: 1,
                    y: 2,
                    z: 0,
                    w: 0,
                },
                true,
            ),
            (
                Coordinate {
                    x: 2,
                    y: 2,
                    z: 0,
                    w: 0,
                },
                true,
            ),
        ];
        let dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 3,
        };

        assert_eq!(dimension.cubes.len(), 9);
        for (ec, es) in expected.iter() {
            assert_eq!(
                1,
                dimension
                    .cubes
                    .iter()
                    .filter(|(c, s)| c.x == ec.x
                        && c.y == ec.y
                        && c.z == ec.z
                        && c.w == ec.w
                        && s == es)
                    .count(),
                "{:?} {:?}",
                ec,
                es
            );
        }
    }
    #[test]
    fn test_cordinates() {
        let s = ".#.\n..#\n###";
        let dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 3,
        };

        let min = dimension.min_cordinate().unwrap();
        assert_eq!((min.x, min.y, min.z), (0, 0, 0));

        let max = dimension.max_cordinate().unwrap();
        assert_eq!((max.x, max.y, max.z), (2, 2, 0));
    }

    #[test]
    fn neighbours_in_grid() {
        let s = ".#.\n..#\n###";
        let dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 3,
        };

        assert_eq!(1, dimension.active_neighbours(0, 0, 0, 0));
        assert_eq!(2, dimension.active_neighbours(2, 2, 0, 0));
    }
    #[test]
    fn neignbours_different_z() {
        let s = ".#.\n..#\n###";
        let dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 3,
        };

        assert_eq!(1, dimension.active_neighbours(0, 0, -1, 0));
        assert_eq!(2, dimension.active_neighbours(3, 2, 1, 0));
        assert_eq!(0, dimension.active_neighbours(8, 8, 8, 0));
    }
    #[test]
    fn sample_test() {
        let s = ".#.\n..#\n###";
        let mut dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 3,
        };

        for _ in 1..=6 {
            dimension.cycle();
        }
        assert_eq!(112, dimension.active_cubes());
    }
    #[test]
    fn second_part_test() {
        let s = ".#.\n..#\n###";
        let mut dimension = PocketDimension {
            cubes: parse_input(s),
            dimensions: 4,
        };

        for _ in 1..=6 {
            dimension.cycle();
        }
        assert_eq!(848, dimension.active_cubes());
    }
}
