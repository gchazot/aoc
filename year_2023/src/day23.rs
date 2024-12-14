use std::collections::{HashSet, VecDeque};

pub fn execute() -> String {
    let mine_slippery = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), true);
    let slippery_routes = mine_slippery.find_routes();
    let part1: i32 = slippery_routes
        .iter()
        .map(|route| route.iter().map(|segment| segment.steps).sum())
        .max()
        .unwrap();

    // let mine_sticky = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), false);
    // let sticky_routes = mine_sticky.find_routes();
    // let part2: i32 = sticky_routes
    //     .iter()
    //     .map(|route| route.iter().map(|segment| segment.steps).sum())
    //     .max()
    //     .unwrap();
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
                    if *further_tile == Tile::Forest {
                        continue;
                    }
                    if let Tile::Slope(slope) = further_tile {
                        if *slope != next_direction {
                            continue;
                        }
                    }
                    result.push(next_direction);
                }
            }
        }

        (next, result)
    }

    fn find_routes(&self) -> Vec<Vec<Segment>> {
        let segments = self.get_segments();
        let start_segment = segments.iter().find(|&s| s.from == self.start).unwrap();
        let mut result = vec![];

        let mut walkers = VecDeque::from([(vec![start_segment.clone()], &start_segment.to)]);
        while !walkers.is_empty() {
            let (current, current_end) = walkers.pop_front().unwrap();
            if *current_end == self.end {
                result.push(current);
            } else {
                let next_options = segments.iter().filter(|&s| {
                    (s.from == *current_end || (!s.one_way && s.to == *current_end))
                        && !current.contains(s)
                });
                for option in next_options {
                    let mut new_route = current.clone();
                    new_route.push(option.clone());
                    let new_end = if *current_end == option.from {
                        &option.to
                    } else {
                        &option.from
                    };
                    walkers.push_back((new_route, new_end));
                }
            }
        }

        result
    }

    fn get_segments(&self) -> Vec<Segment> {
        let mut segments = vec![];
        let mut junctions = HashSet::new();

        let mut starts = VecDeque::from([(self.start.clone(), Direction::Down)]);

        while !starts.is_empty() {
            let (start, mut direction) = starts.pop_front().unwrap();
            let mut walker = Walker::new(start.clone());
            let mut steps = 0;
            let mut seen_slope = false;
            loop {
                steps += 1;
                let (new_pos, mut directions) = self.step(walker.current, &direction);
                if let Tile::Slope(_) = self.get(&new_pos) {
                    seen_slope = true;
                }
                walker.current = new_pos;

                if directions.len() == 1 {
                    direction = directions.pop().unwrap();
                } else {
                    if directions.len() != 0 || walker.current == self.end {
                        segments.push(Segment {
                            from: start.clone(),
                            to: walker.current.clone(),
                            steps,
                            one_way: seen_slope,
                        });
                    }
                    if junctions.insert(walker.current.clone()) {
                        for direction in directions {
                            starts.push_back((walker.current.clone(), direction));
                        }
                    }
                    break;
                }
            }
        }
        segments
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

#[derive(Clone, Debug)]
struct Segment {
    from: Coordinates,
    to: Coordinates,
    steps: i32,
    one_way: bool,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (!self.one_way && self.from == other.to && self.to == other.from)
    }
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
    fn test_find_routes() {
        let example_slippery = Map::from_lines(example(), true);
        let slippery_paths = example_slippery.find_routes();
        let slippery_paths_lengths = slippery_paths
            .iter()
            .map(|p| p.iter().map(|s| s.steps).sum())
            .collect::<HashSet<_>>();

        assert_eq!(6, slippery_paths.len());
        assert_eq!(94, *slippery_paths_lengths.iter().max().unwrap());

        let example_sticky = Map::from_lines(example(), false);
        let sticky_paths = example_sticky.find_routes();
        let sticky_paths_lengths = sticky_paths
            .iter()
            .map(|p| p.iter().map(|s| s.steps).sum())
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
