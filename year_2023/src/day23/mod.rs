use crate::utils;
use std::collections::{HashSet, VecDeque};

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let mine = Map::from_lines(utils::read_lines("src/day23/mine.txt"));
    let paths = find_paths(&mine);

    assert_eq!(2394, paths.iter().map(|p| p.len() - 1).max().unwrap());
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Coordinates,
    end: Coordinates,
}

impl Map {
    fn from_lines(lines: Vec<String>) -> Map {
        let tiles: Vec<Vec<Tile>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        '.' => Tile::Path,
                        '<' => Tile::Slope(Direction::Left),
                        '>' => Tile::Slope(Direction::Right),
                        '^' => Tile::Slope(Direction::Up),
                        'v' => Tile::Slope(Direction::Down),
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        assert!(tiles.iter().all(|row| row.len() == tiles.len()));

        let start_index = tiles
            .first()
            .unwrap()
            .iter()
            .position(|tile| *tile == Tile::Path)
            .unwrap();

        let start = Coordinates {
            x: start_index,
            y: 0,
        };

        let mut end_index = tiles
            .last()
            .unwrap()
            .iter()
            .position(|tile| *tile == Tile::Path)
            .unwrap();

        let end = Coordinates {
            x: end_index,
            y: tiles.len() - 1,
        };

        Map { tiles, start, end }
    }

    fn get(&self, position: &Coordinates) -> Option<Tile> {
        if position.x < 0
            || position.y < 0
            || position.x >= self.tiles.len()
            || position.y >= self.tiles.len()
        {
            None
        } else {
            Some(self.tiles[position.y][position.x].clone())
        }
    }
}

#[test]
fn test_from_lines() {
    let example = Map::from_lines(_example());
    assert_eq!(example.tiles.len(), 23);
    assert_eq!(example.tiles[0].len(), 23);
    assert_eq!(example.start, Coordinates { x: 1, y: 0 });
    assert_eq!(example.end, Coordinates { x: 21, y: 22 });

    assert_eq!(example.get(&Coordinates { x: 0, y: 0 }), Some(Tile::Forest));
    assert_eq!(example.get(&Coordinates { x: 1, y: 1 }), Some(Tile::Path));
    assert_eq!(
        example.get(&Coordinates { x: 10, y: 3 }),
        Some(Tile::Slope(Direction::Right))
    );
    assert_eq!(
        example.get(&Coordinates { x: 3, y: 4 }),
        Some(Tile::Slope(Direction::Down))
    );
}

fn _example() -> Vec<String> {
    vec![
        "#.#####################".to_string(),
        "#.......#########...###".to_string(),
        "#######.#########.#.###".to_string(),
        "###.....#.>.>.###.#.###".to_string(),
        "###v#####.#v#.###.#.###".to_string(),
        "###.>...#.#.#.....#...#".to_string(),
        "###v###.#.#.#########.#".to_string(),
        "###...#.#.#.......#...#".to_string(),
        "#####.#.#.#######.#.###".to_string(),
        "#.....#.#.#.......#...#".to_string(),
        "#.#####.#.#.#########v#".to_string(),
        "#.#...#...#...###...>.#".to_string(),
        "#.#.#v#######v###.###v#".to_string(),
        "#...#.>.#...>.>.#.###.#".to_string(),
        "#####v#.#.###v#.#.###.#".to_string(),
        "#.....#...#...#.#.#...#".to_string(),
        "#.#########.###.#.#.###".to_string(),
        "#...###...#...#...#.###".to_string(),
        "###.###.#.###v#####v###".to_string(),
        "#...#...#.#.>.>.#.>.###".to_string(),
        "#.###.###.#.###.#.#v###".to_string(),
        "#.....###...###...#...#".to_string(),
        "#####################.#".to_string(),
    ]
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn next(&self) -> Vec<Coordinates> {
        use Direction::*;
        let mut result = vec![];

        for direction in [Left, Right, Up, Down] {
            let maybe_next = self.towards(direction);
            if maybe_next.is_some() {
                result.push(maybe_next.unwrap());
            }
        }
        result
    }

    fn towards(&self, direction: Direction) -> Option<Coordinates> {
        use Direction::*;

        match direction {
            Left => {
                if self.x > 0 {
                    Some(Coordinates {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Right => Some(Coordinates {
                x: self.x + 1,
                y: self.y,
            }),
            Up => {
                if self.y > 0 {
                    Some(Coordinates {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Down => Some(Coordinates {
                x: self.x,
                y: self.y + 1,
            }),
        }
    }
}

struct Walker {
    positions: HashSet<Coordinates>,
    current: Coordinates,
}

impl Walker {
    fn new(position: Coordinates) -> Walker {
        Walker {
            current: position.clone(),
            positions: HashSet::from([position]),
        }
    }

    fn next(&self, map: &Map) -> Vec<Walker> {
        let mut result = vec![];
        for pos in self.current.next() {
            if !self.positions.contains(&pos) {
                match map.get(&pos) {
                    None | Some(Tile::Forest) => {}
                    Some(Tile::Path) => {
                        let mut positions = self.positions.clone();
                        positions.insert(pos.clone());
                        result.push(Walker {
                            current: pos,
                            positions,
                        })
                    }
                    Some(Tile::Slope(d)) => {
                        let mut positions = self.positions.clone();
                        let bottom = pos.towards(d).unwrap();
                        if !self.positions.contains(&bottom) {
                            positions.insert(pos);
                            positions.insert(bottom.clone());
                            result.push(Walker {
                                current: bottom,
                                positions,
                            })
                        }
                    }
                }
            }
        }

        result
    }

    fn print(&self, map: &Map) {
        map.tiles.iter().enumerate().for_each(|(y, row)| {
            let line = row
                .iter()
                .enumerate()
                .map(|(x, tile)| {
                    let coords = Coordinates { x, y };
                    if self.positions.contains(&coords) {
                        "O"
                    } else {
                        match map.get(&coords) {
                            None | Some(Tile::Forest) => "#",
                            Some(Tile::Path) => ".",
                            Some(Tile::Slope(Direction::Left)) => "<",
                            Some(Tile::Slope(Direction::Right)) => ">",
                            Some(Tile::Slope(Direction::Up)) => "^",
                            Some(Tile::Slope(Direction::Down)) => "v",
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            println!("{}", line);
        })
    }
}

fn find_paths(map: &Map) -> Vec<HashSet<Coordinates>> {
    let mut result = vec![];

    let mut walkers = VecDeque::from([Walker::new(map.start.clone())]);
    while !walkers.is_empty() {
        let current = walkers.pop_front().unwrap();
        if current.current == map.end {
            result.push(current.positions);
        } else {
            for new_walker in current.next(&map) {
                walkers.push_back(new_walker);
            }
        }
    }

    result
}

#[test]
fn test_find_paths() {
    let example = Map::from_lines(_example());
    let paths = find_paths(&example);

    let paths_lengths = paths.iter().map(|p| p.len() - 1).collect::<HashSet<_>>();
    println!("{:?}", paths_lengths);

    assert_eq!(6, paths.len());
    assert_eq!(94, *paths_lengths.iter().max().unwrap());
}
