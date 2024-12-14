use std::collections::{HashSet, VecDeque};

pub fn execute() -> String {
    let mine_slippery = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), true);
    let slippery_paths = mine_slippery.find_paths();
    let part1 = slippery_paths.iter().map(|p| p.len() - 1).max().unwrap();

    // TODO: Find something fast enough for part 2
    // let mine_sticky = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), false);
    // let sticky_paths = find_paths(&mine_sticky);
    // assert_eq!(
    //     2394,
    //     sticky_paths.iter().map(|p| p.len() - 1).max().unwrap()
    // );
    // let part2 = sticky_paths.iter().map(|p| p.len() - 1).max().unwrap();
    let part2 = 0;

    format!("{} {}", part1, part2)
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
    fn from_lines(lines: Vec<String>, with_slopes: bool) -> Map {
        let tiles: Vec<Vec<Tile>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        '.' => Tile::Path,
                        '<' => {
                            if with_slopes {
                                Tile::Slope(Direction::Left)
                            } else {
                                Tile::Path
                            }
                        }
                        '>' => {
                            if with_slopes {
                                Tile::Slope(Direction::Right)
                            } else {
                                Tile::Path
                            }
                        }
                        '^' => {
                            if with_slopes {
                                Tile::Slope(Direction::Up)
                            } else {
                                Tile::Path
                            }
                        }
                        'v' => {
                            if with_slopes {
                                Tile::Slope(Direction::Down)
                            } else {
                                Tile::Path
                            }
                        }
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

        let end_index = tiles
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

    fn get(&self, position: &Coordinates) -> &Tile {
        &self.tiles[position.y][position.x]
    }

    fn step(&self, from: Coordinates, direction: &Direction) -> (Coordinates, Vec<Direction>) {
        use Direction::*;

        let mut result = vec![];

        let next = from.towards(direction).unwrap();

        for next_direction in [Left, Right, Up, Down] {
            if let Some(further) = next.towards(&next_direction) {
                if further.x < self.tiles.len() && further.y < self.tiles.len() && from != further {
                    let further_tile = self.get(&further);
                    if further_tile != &Tile::Forest {
                        result.push(next_direction);
                    }
                }
            }
        }

        (next, result)
    }

    fn find_paths(&self) -> Vec<HashSet<Coordinates>> {
        let mut result = vec![];

        let mut walkers = VecDeque::from([Walker::new(self.start.clone())]);
        while !walkers.is_empty() {
            let current = walkers.pop_front().unwrap();
            if current.current == self.end {
                result.push(current.positions);
            } else {
                for new_walker in current.next(&self) {
                    walkers.push_back(new_walker);
                }
            }
        }

        result
    }
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
            let maybe_next = self.towards(&direction);
            if maybe_next.is_some() {
                result.push(maybe_next.unwrap());
            }
        }
        result
    }

    fn towards(&self, direction: &Direction) -> Option<Coordinates> {
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

#[derive(Debug)]
struct Segment {
    from: Coordinates,
    to: Coordinates,
    steps: i32,
}

#[derive(Clone)]
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

    fn next(self, map: &Map) -> Vec<Walker> {
        let actual_next_iter = self
            .current
            .next()
            .into_iter()
            .filter(|pos| &Tile::Forest != map.get(pos) && !self.positions.contains(pos));

        let mut result: Vec<Walker> = vec![];
        let mut to_remove = vec![];
        let mut actual_next = vec![];

        for (i, pos) in actual_next_iter.enumerate() {
            if i != 0 {
                result.push(self.clone());
            }
            actual_next.push(pos);
        }
        if actual_next.len() > 0 {
            result.push(self);
        }

        for i in 0..actual_next.len() {
            let pos = actual_next.pop().unwrap();
            match map.get(&pos) {
                Tile::Path => {
                    result[i].positions.insert(pos.clone());
                    result[i].current = pos;
                }
                Tile::Slope(d) => {
                    let bottom = pos.towards(d).unwrap();
                    if result[i].positions.insert(bottom.clone()) {
                        result[i].positions.insert(pos);
                        result[i].current = bottom;
                    } else {
                        to_remove.push(i);
                    }
                }
                _ => unreachable!(),
            }
        }
        for i in to_remove.iter().rev() {
            result.remove(*i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "2394 0");
    }

    #[test]
    fn test_from_lines() {
        let example = Map::from_lines(example(), true);
        assert_eq!(example.tiles.len(), 23);
        assert_eq!(example.tiles[0].len(), 23);
        assert_eq!(example.start, Coordinates { x: 1, y: 0 });
        assert_eq!(example.end, Coordinates { x: 21, y: 22 });

        assert_eq!(example.get(&Coordinates { x: 0, y: 0 }), &Tile::Forest);
        assert_eq!(example.get(&Coordinates { x: 1, y: 1 }), &Tile::Path);
        assert_eq!(
            example.get(&Coordinates { x: 10, y: 3 }),
            &Tile::Slope(Direction::Right)
        );
        assert_eq!(
            example.get(&Coordinates { x: 3, y: 4 }),
            &Tile::Slope(Direction::Down)
        );
    }

    #[test]
    fn test_find_paths() {
        let example_slippery = Map::from_lines(example(), true);
        let slippery_paths = example_slippery.find_paths();
        let slippery_paths_lengths = slippery_paths
            .iter()
            .map(|p| p.len() - 1)
            .collect::<HashSet<_>>();

        assert_eq!(6, slippery_paths.len());
        assert_eq!(94, *slippery_paths_lengths.iter().max().unwrap());

        let example_sticky = Map::from_lines(example(), false);
        let sticky_paths = example_sticky.find_paths();
        let sticky_paths_lengths = sticky_paths
            .iter()
            .map(|p| p.len() - 1)
            .collect::<HashSet<_>>();

        assert_eq!(154, *sticky_paths_lengths.iter().max().unwrap());
    }
    impl Walker {
        fn _print(&self, map: &Map) {
            map.tiles.iter().enumerate().for_each(|(y, row)| {
                let line = row
                    .iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        let coords = Coordinates { x, y };
                        if self.positions.contains(&coords) {
                            "O"
                        } else {
                            match tile {
                                Tile::Forest => "#",
                                Tile::Path => ".",
                                Tile::Slope(Direction::Left) => "<",
                                Tile::Slope(Direction::Right) => ">",
                                Tile::Slope(Direction::Up) => "^",
                                Tile::Slope(Direction::Down) => "v",
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                println!("{}", line);
            })
        }
    }

    fn example() -> Vec<String> {
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
}