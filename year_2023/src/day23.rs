use std::collections::{HashMap, HashSet, VecDeque};

pub fn execute() -> String {
    let mine_slippery = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), true);
    let part1 = mine_slippery.find_longest_route();

    let mine_sticky = Map::from_lines(aoc_utils::read_lines("input/day23.txt"), false);
    let part2 = mine_sticky.find_longest_route();

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

    fn find_longest_route(&self) -> i32 {
        let (segments, junctions) = self.get_segments();
        let junction_ids = junctions
            .iter()
            .enumerate()
            .map(|(i, junction)| (junction, i))
            .collect::<HashMap<_, _>>();

        let n_vertex = junctions.len();
        let mut edges = vec![HashMap::<usize, i32>::new(); n_vertex];
        for segment in segments {
            let from = junction_ids[&segment.from];
            let to = junction_ids[&segment.to];
            edges[from].insert(to, segment.steps);
            if !segment.one_way {
                edges[to].insert(from, segment.steps);
            };
        }

        fn dfs(
            from: usize,
            to: usize,
            path: &mut Vec<usize>,
            len: i32,
            n_vertex: usize,
            edges: &[HashMap<usize, i32>],
            seen: &mut [bool],
        ) -> (i32, Vec<usize>) {
            path.push(from);
            let mut max_len = 0;
            let mut max_edges = vec![];

            let from_edges = &edges[from];
            for (&next, &edge_len) in from_edges.iter() {
                if next != from && !seen[next] {
                    seen[next] = true;

                    let option_len = len + edge_len;
                    let (next_len, next_edges) = if next == to {
                        let mut last_edges = path.clone();
                        last_edges.push(next);
                        (option_len, last_edges)
                    } else {
                        dfs(next, to, path, option_len, n_vertex, edges, seen)
                    };

                    if next_len > max_len {
                        max_len = next_len;
                        max_edges = next_edges;
                    }

                    seen[next] = false;
                }
            }
            path.pop();
            (max_len, max_edges)
        }

        let mut seen = vec![false; n_vertex];
        let (best_len, _best_edges) = dfs(
            junction_ids[&self.start],
            junction_ids[&self.end],
            &mut vec![],
            0,
            n_vertex,
            edges.as_slice(),
            seen.as_mut_slice(),
        );

        best_len
    }

    fn get_segments(&self) -> (Vec<Segment>, HashSet<Coordinates>) {
        let mut segments = vec![];
        let mut junctions = HashSet::from([self.start.clone()]);

        let mut starts = VecDeque::from([(self.start.clone(), Direction::Down)]);

        while !starts.is_empty() {
            let (start, mut direction) = starts.pop_front().unwrap();
            let mut walker = start.clone();
            let mut steps = 0;
            let mut seen_slope = false;
            loop {
                steps += 1;
                let (new_pos, mut directions) = self.step(walker, &direction);
                if let Tile::Slope(_) = self.get(&new_pos) {
                    seen_slope = true;
                }
                walker = new_pos;

                if !seen_slope && directions.len() == 1 {
                    direction = directions.pop().unwrap();
                } else {
                    if directions.len() != 0 || walker == self.end {
                        segments.push(Segment {
                            from: start.clone(),
                            to: walker.clone(),
                            steps,
                            one_way: seen_slope,
                        });
                    }
                    if junctions.insert(walker.clone()) {
                        for direction in directions {
                            starts.push_back((walker.clone(), direction));
                        }
                    }
                    break;
                }
            }
        }
        (segments, junctions)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "2394 6554");
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
    fn test_find_longest_route() {
        let example_slippery = Map::from_lines(example(), true);
        assert_eq!(94, example_slippery.find_longest_route());

        let example_sticky = Map::from_lines(example(), false);
        assert_eq!(154, example_sticky.find_longest_route());
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
