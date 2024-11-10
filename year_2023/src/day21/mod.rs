use crate::utils;
use std::collections::HashSet;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let garden = Garden::from_lines(utils::read_lines("src/day21/mine.txt"));
    let mut gardener = Gardener::new(garden);

    for _ in 0..64 {
        gardener.step();
    }
    assert_eq!(3847, gardener.positions.len());
}

struct Garden {
    plots: Vec<Coordinates>,
    start: Coordinates,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Garden {
    fn from_lines(lines: Vec<String>) -> Garden {
        let mut plots = Vec::new();
        let mut start = Coordinates { x: 0, y: 0 };
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coords = Coordinates { x, y };
                match c {
                    '.' => {
                        plots.push(coords);
                    }
                    'S' => {
                        start = coords.clone();
                        plots.push(coords);
                    }
                    '#' => {}
                    _ => unreachable!(),
                }
            }
        }
        Garden { plots, start }
    }
}

#[test]
fn test_from_lines() {
    let lines = _example();

    let example = Garden::from_lines(lines);

    assert_eq!(81, example.plots.len());
    assert_eq!(Coordinates { x: 5, y: 5 }, example.start);
}

fn _example() -> Vec<String> {
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ]
}

struct Gardener {
    garden: Garden,
    positions: HashSet<Coordinates>,
    steps: usize,
}

impl Gardener {
    fn new(garden: Garden) -> Gardener {
        let positions = HashSet::from([garden.start.clone()]);
        Gardener {
            garden,
            positions,
            steps: 0,
        }
    }

    fn step(&mut self) {
        let mut new_positions = HashSet::new();
        for position in self.positions.iter() {
            for next_pos in [
                Coordinates {
                    x: position.x + 1,
                    y: position.y,
                },
                Coordinates {
                    x: position.x - 1,
                    y: position.y,
                },
                Coordinates {
                    x: position.x,
                    y: position.y + 1,
                },
                Coordinates {
                    x: position.x,
                    y: position.y - 1,
                },
            ] {
                if self.garden.plots.contains(&next_pos) {
                    new_positions.insert(next_pos);
                }
            }
        }
        self.positions = new_positions;
        self.steps += 1;
    }
}

#[test]
fn test_gardener() {
    let garden = Garden::from_lines(_example());
    let gardener = Gardener::new(garden);

    assert_eq!(
        HashSet::from([Coordinates { x: 5, y: 5 }]),
        gardener.positions
    );
    assert_eq!(0, gardener.steps);
}

#[test]
fn test_step() {
    let garden = Garden::from_lines(_example());
    let mut gardener = Gardener::new(garden);

    gardener.step();
    assert_eq!(1, gardener.steps);
    assert_eq!(
        HashSet::from([Coordinates { x: 5, y: 4 }, Coordinates { x: 4, y: 5 }]),
        gardener.positions
    );

    gardener.step();
    assert_eq!(2, gardener.steps);
    assert_eq!(
        HashSet::from([
            Coordinates { x: 5, y: 3 },
            Coordinates { x: 5, y: 5 },
            Coordinates { x: 3, y: 5 },
            Coordinates { x: 4, y: 6 }
        ]),
        gardener.positions
    );
}

fn test_example() {
    let garden = Garden::from_lines(_example());
    let mut gardener = Gardener::new(garden);

    for _ in 0..6 {
        gardener.step();
    }
    assert_eq!(16, gardener.positions.len());
}
