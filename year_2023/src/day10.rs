use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

pub fn execute() -> String {
    let map = PipeMap::from_file("day10.txt");
    let net = map.to_network();
    let length = net.pipe_length(map.start().unwrap());

    let part1 = length / 2;
    let part2 = map.inner_size();

    format!("{} {}", part1, part2)
}

struct Network {
    connections: HashMap<Position, HashSet<Position>>,
}

impl Network {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn add(&mut self, from: Position, to: Position) {
        self.connections
            .entry(from)
            .or_insert(HashSet::new())
            .insert(to);
    }

    fn get(&self, pos: &Position) -> HashSet<Position> {
        let neighbours = self.connections.get(pos);
        if neighbours.is_some() {
            neighbours.unwrap().clone()
        } else {
            HashSet::new()
        }
    }

    fn clean(&self, start: &Position) -> Self {
        let mut clean = Self::new();
        for position in self.reachable_from(start) {
            for neighbour in self.get(&position) {
                clean.add(position.clone(), neighbour);
            }
        }
        clean
    }

    fn pipe_length(&self, start: &Position) -> usize {
        let pipe_positions = self.reachable_from(start);
        pipe_positions.len()
    }

    fn reachable_from(&self, start: &Position) -> HashSet<Position> {
        let mut reachable = HashSet::new();
        let mut next_positions = VecDeque::from([start.clone()]);

        while !next_positions.is_empty() {
            let next = next_positions.pop_front().unwrap();
            let neighbours = self.get(&next);

            reachable.insert(next);

            for neighbour in neighbours {
                if !reachable.contains(&neighbour) {
                    next_positions.push_back(neighbour.clone());
                }
            }
        }
        reachable
    }

    fn inner_region(&self, start: &Position) -> HashSet<Position> {
        let expanded = self.clean(start).expand();
        let inner_expanded = expanded._inner_region_core();

        let positions = inner_expanded
            .iter()
            .filter(|&pos| pos.0 % 2 == 0 && pos.1 % 2 == 0)
            .map(|pos| Position(pos.0 / 2, pos.1 / 2));

        HashSet::from_iter(positions)
    }
    fn _inner_region_core(&self) -> HashSet<Position> {
        let max_x = self.connections.keys().map(|pos| pos.0).max().unwrap() + 1;
        let max_y = self.connections.keys().map(|pos| pos.1).max().unwrap() + 1;

        let outer = self._outer_region(max_x, max_y);

        let mut inner = HashSet::<Position>::new();
        for i in 0..max_x {
            for j in 0..max_y {
                let current = Position(i, j);
                if self.connections.contains_key(&current) || outer.contains(&current) {
                    continue;
                }
                inner.insert(Position(i, j));
            }
        }
        inner
    }

    fn _outer_region(&self, max_x: usize, max_y: usize) -> HashSet<Position> {
        let mut outer = HashSet::new();
        let mut to_visit = VecDeque::from([Position(0, 0), Position(max_x, max_y)]);

        while !to_visit.is_empty() {
            use Direction::*;

            let current = to_visit.pop_front().unwrap();

            if self.connections.contains_key(&current) {
                continue;
            }

            for direction in [North, South, East, West] {
                if !current.can_step(&direction) {
                    continue;
                }
                let next = current.to(&direction).unwrap();
                if next.0 > max_x
                    || next.1 > max_y
                    || outer.contains(&next)
                    || to_visit.contains(&next)
                {
                    continue;
                }
                to_visit.push_back(next);
            }

            outer.insert(current);
        }
        outer
    }

    fn expand(&self) -> Self {
        let mut expanded = Self::new();
        for (pos, neighbours) in self.connections.iter() {
            let exp_pos = Position(pos.0 * 2, pos.1 * 2);
            for neigh in neighbours {
                let exp_neigh = Position(neigh.0 * 2, neigh.1 * 2);
                let exp_betwn =
                    Position((exp_pos.0 + exp_neigh.0) / 2, (exp_pos.1 + exp_neigh.1) / 2);

                expanded.add(exp_pos.clone(), exp_betwn.clone());
                expanded.add(exp_betwn, exp_neigh);
            }
        }
        expanded
    }
}

fn get_direction(a: &Position, b: &Position) -> Result<Direction, &'static str> {
    let delta = (b.0 as i64 - a.0 as i64, b.1 as i64 - a.1 as i64);
    match delta {
        (0, -1) => Ok(Direction::North),
        (0, 1) => Ok(Direction::South),
        (1, 0) => Ok(Direction::East),
        (-1, 0) => Ok(Direction::West),
        _ => Err("Not a valid step"),
    }
}

struct PipeMap {
    nodes: HashMap<Position, Pipe>,
}

impl PipeMap {
    fn from_file(filename: &str) -> PipeMap {
        let path = format!("input/{}", &filename);
        let lines = aoc_utils::read_lines(&path);

        let mut nodes = HashMap::<Position, Pipe>::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let pos = Position(j + 1, i + 1);
                nodes.insert(pos, pipe_from_char(char));
            }
        }

        PipeMap { nodes }
    }

    fn start(&self) -> Option<&Position> {
        let start_item = self
            .nodes
            .iter()
            .find_map(|(position, pipe)| matches!(pipe, Pipe::Start).then_some(position));
        start_item
    }

    fn to_network(&self) -> Network {
        let mut network = Network::new();
        for (position, pipe) in self.nodes.iter() {
            for forward in pipe_to_directions(pipe) {
                if !position.can_step(&forward) {
                    continue;
                }
                let dest = position.to(&forward).unwrap();

                let backward = get_direction(&dest, position).unwrap();
                let dest_pipe = self.nodes.get(&dest);
                if !dest_pipe.is_some() {
                    continue;
                }

                let dest_directions = pipe_to_directions(dest_pipe.unwrap());
                if dest_directions.contains(&backward) {
                    network.add(position.clone(), dest);
                }
            }
        }
        network
    }

    fn inner_size(&self) -> usize {
        let net = self.to_network();
        let start = self.start().unwrap();
        let inner = net.inner_region(start);
        inner.len()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
struct Position(usize, usize);

impl Position {
    fn to(&self, direction: &Direction) -> Result<Self, &str> {
        if self.can_step(direction) {
            let mut result = self.clone();
            result.step(direction);
            Ok(result)
        } else {
            Err("Would step out")
        }
    }

    fn step(&mut self, direction: &Direction) {
        use Direction::*;
        match direction {
            North => self.1 -= 1,
            South => self.1 += 1,
            East => self.0 += 1,
            West => self.0 -= 1,
        };
    }

    fn can_step(&self, direction: &Direction) -> bool {
        use Direction::*;
        match direction {
            North => self.1 > 0,
            West => self.0 > 0,
            _ => true,
        }
    }
}

enum Pipe {
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    EastWest,
    Start,
    None,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Pipe::NorthSouth => '|',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthEast => 'F',
            Pipe::SouthWest => '7',
            Pipe::EastWest => '-',
            Pipe::Start => 'S',
            Pipe::None => '.',
        };
        write!(f, "{}", value)
    }
}

fn pipe_from_char(c: char) -> Pipe {
    match c {
        '|' => Pipe::NorthSouth,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        'F' => Pipe::SouthEast,
        '7' => Pipe::SouthWest,
        '-' => Pipe::EastWest,
        'S' => Pipe::Start,
        _ => Pipe::None,
    }
}

fn pipe_to_directions(pipe: &Pipe) -> HashSet<Direction> {
    use Direction::*;
    match pipe {
        Pipe::NorthSouth => HashSet::from([North, South]),
        Pipe::NorthEast => HashSet::from([North, East]),
        Pipe::NorthWest => HashSet::from([North, West]),
        Pipe::SouthEast => HashSet::from([South, East]),
        Pipe::SouthWest => HashSet::from([South, West]),
        Pipe::EastWest => HashSet::from([East, West]),
        Pipe::Start => HashSet::from([North, South, East, West]),
        Pipe::None => HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "6942 297");
    }

    #[test]
    fn test_inner_size() {
        let map2 = PipeMap::from_file("day10-example2.txt");
        assert_eq!(map2.inner_size(), 1);
        let map3 = PipeMap::from_file("day10-example3.txt");
        assert_eq!(map3.inner_size(), 1);
        let map4 = PipeMap::from_file("day10-example4.txt");
        assert_eq!(map4.inner_size(), 4);
        let map5 = PipeMap::from_file("day10-example5.txt");
        assert_eq!(map5.inner_size(), 4);
        let map6 = PipeMap::from_file("day10-example6.txt");
        assert_eq!(map6.inner_size(), 8);
        let map7 = PipeMap::from_file("day10-example7.txt");
        assert_eq!(map7.inner_size(), 10);
    }

    #[test]
    fn test_inner_region() {
        let map2 = PipeMap::from_file("day10-example2.txt");
        let net2 = map2.to_network();

        let inner = net2.inner_region(map2.start().unwrap());

        assert_eq!(inner.len(), 1);
        assert!(inner.contains(&Position(3, 3)));
    }

    #[test]
    fn test_expand_network() {
        let map3 = PipeMap::from_file("day10-example3.txt");
        let net3 = map3.to_network().clean(map3.start().unwrap());
        assert_eq!(8, net3.connections.len());
        let exp3 = net3.expand();
        assert_eq!(8, net3.connections.len());
        assert_eq!(16, exp3.connections.len());
    }

    #[test]
    fn test_pipe_length() {
        let map3 = PipeMap::from_file("day10-example3.txt");
        let net3 = map3.to_network();
        let length = net3.pipe_length(map3.start().unwrap());
        assert_eq!(8, length);

        // This is the furthest point away
        assert_eq!(4, length / 2);
    }

    #[test]
    fn test_clean() {
        let map2 = PipeMap::from_file("day10-example2.txt");
        let net2 = map2.to_network();
        let map3 = PipeMap::from_file("day10-example3.txt");
        let net3 = map3.to_network();
        let clean3 = net3.clean(map3.start().unwrap());

        assert_eq!(net2.connections, clean3.connections);
    }

    #[test]
    fn test_map_to_network() {
        let net1 = PipeMap::from_file("day10-example1.txt").to_network();

        assert_eq!(8, net1.connections.len());

        assert_eq!(
            HashSet::from([Position(2, 3), Position(3, 2)]),
            net1.get(&Position(2, 2)),
        );
        assert_eq!(
            HashSet::from([Position(4, 2), Position(4, 4)]),
            net1.get(&Position(4, 3)),
        );
        assert_eq!(
            HashSet::from([Position(2, 3), Position(3, 4)]),
            net1.get(&Position(2, 4)),
        );

        let net2 = PipeMap::from_file("day10-example2.txt").to_network();
        assert_eq!(net1.connections.len(), net2.connections.len());
        assert_eq!(net1.connections, net2.connections);

        let net3 = PipeMap::from_file("day10-example3.txt").to_network();

        assert!(net3.connections.len() >= net1.connections.len());
        for (pos, neighbours) in net1.connections.iter() {
            assert_eq!(&net3.get(pos), neighbours);
        }
    }
    #[test]
    fn test_get_direction() {
        use Direction::*;

        fn check(expected: Result<Direction, &'static str>, a: Position, b: Position) {
            assert_eq!(expected, get_direction(&a, &b));
        }

        check(Ok(North), Position(1, 2), Position(1, 1));
        check(Ok(South), Position(1, 1), Position(1, 2));
        check(Ok(East), Position(1, 1), Position(2, 1));
        check(Ok(West), Position(2, 1), Position(1, 1));
        check(Err("Not a valid step"), Position(1, 1), Position(1, 1));
        check(Err("Not a valid step"), Position(1, 1), Position(1, 3));
        check(Err("Not a valid step"), Position(1, 3), Position(1, 1));
        check(Err("Not a valid step"), Position(3, 1), Position(1, 1));
        check(Err("Not a valid step"), Position(1, 1), Position(3, 1));
        check(Err("Not a valid step"), Position(1, 1), Position(3, 3));
        check(Err("Not a valid step"), Position(3, 3), Position(1, 1));
    }

    #[test]
    fn test_parse_pipe_map() {
        let map1 = PipeMap::from_file("day10-example1.txt");
        assert_eq!(5 * 5, map1.nodes.len());
        assert!(matches!(map1.start(), None));
        assert!(matches!(map1.nodes[&Position(1, 1)], Pipe::None));
        assert!(matches!(map1.nodes[&Position(2, 2)], Pipe::SouthEast));
        assert!(matches!(map1.nodes[&Position(3, 2)], Pipe::EastWest));
        assert!(matches!(map1.nodes[&Position(4, 2)], Pipe::SouthWest));
        assert!(matches!(map1.nodes[&Position(2, 3)], Pipe::NorthSouth));
        assert!(matches!(map1.nodes[&Position(4, 3)], Pipe::NorthSouth));
        assert!(matches!(map1.nodes[&Position(2, 4)], Pipe::NorthEast));
        assert!(matches!(map1.nodes[&Position(3, 4)], Pipe::EastWest));
        assert!(matches!(map1.nodes[&Position(4, 4)], Pipe::NorthWest));
        assert!(matches!(map1.nodes[&Position(1, 5)], Pipe::None));
        assert!(matches!(map1.nodes[&Position(5, 5)], Pipe::None));

        let map2 = PipeMap::from_file("day10-example2.txt");
        assert_eq!(5 * 5, map2.nodes.len());
        assert!(matches!(map2.start(), Some(Position(2, 2))));
        assert!(matches!(map2.nodes[&Position(1, 1)], Pipe::None));
        assert!(matches!(map2.nodes[&Position(2, 2)], Pipe::Start));
        assert!(matches!(map2.nodes[&Position(4, 2)], Pipe::SouthWest));
        assert!(matches!(map2.nodes[&Position(2, 4)], Pipe::NorthEast));

        let map3 = PipeMap::from_file("day10-example3.txt");
        assert_eq!(5 * 5, map3.nodes.len());
        assert!(matches!(map3.start(), Some(Position(2, 2))));
        assert!(matches!(map3.nodes[&Position(1, 1)], Pipe::EastWest));
        assert!(matches!(map3.nodes[&Position(2, 2)], Pipe::Start));
        assert!(matches!(map3.nodes[&Position(5, 1)], Pipe::SouthWest));
        assert!(matches!(map3.nodes[&Position(1, 5)], Pipe::NorthEast));
        assert!(matches!(map3.nodes[&Position(5, 5)], Pipe::SouthEast));
    }

    #[test]
    fn test_position_to() {
        use Direction::*;

        let origin = Position(0, 0);
        assert_eq!(Position(0, 0), origin);
        let east = origin.to(&East).unwrap();
        assert_eq!(Position(1, 0), east);
        let southeast = east.to(&South).unwrap();
        assert_eq!(Position(1, 1), southeast);
        let south = southeast.to(&West).unwrap();
        assert_eq!(Position(0, 1), south);
        let origin_again = south.to(&North).unwrap();
        assert_eq!(Position(0, 0), origin_again);
    }
    #[test]
    fn test_position_step() {
        use Direction::*;

        let mut pos = Position(0, 0);
        assert_eq!(Position(0, 0), pos);
        pos.step(&East);
        assert_eq!(Position(1, 0), pos);
        pos.step(&South);
        assert_eq!(Position(1, 1), pos);
        pos.step(&West);
        assert_eq!(Position(0, 1), pos);
        pos.step(&North);
        assert_eq!(Position(0, 0), pos);

        pos.step(&East);
        pos.step(&East);
        pos.step(&East);
        assert_eq!(Position(3, 0), pos);
        pos.step(&South);
        pos.step(&South);
        pos.step(&South);
        pos.step(&South);
        pos.step(&South);
        assert_eq!(Position(3, 5), pos);
        pos.step(&West);
        pos.step(&West);
        assert_eq!(Position(1, 5), pos);
        pos.step(&North);
        assert_eq!(Position(1, 4), pos);
    }

    #[test]
    fn test_position_can_step() {
        use Direction::*;
        assert!(!Position(0, 0).can_step(&North));
        assert!(Position(0, 0).can_step(&South));
        assert!(Position(0, 0).can_step(&East));
        assert!(!Position(0, 0).can_step(&West));

        assert!(!Position(1, 0).can_step(&North));
        assert!(Position(1, 0).can_step(&South));
        assert!(Position(1, 0).can_step(&East));
        assert!(Position(1, 0).can_step(&West));

        assert!(Position(0, 1).can_step(&North));
        assert!(Position(0, 1).can_step(&South));
        assert!(Position(0, 1).can_step(&East));
        assert!(!Position(0, 1).can_step(&West));

        assert!(Position(1, 1).can_step(&North));
        assert!(Position(1, 1).can_step(&South));
        assert!(Position(1, 1).can_step(&East));
        assert!(Position(1, 1).can_step(&West));
    }

    #[test]
    fn test_parse_pipe() {
        assert!(matches!(pipe_from_char('|'), Pipe::NorthSouth));
        assert!(matches!(pipe_from_char('L'), Pipe::NorthEast));
        assert!(matches!(pipe_from_char('J'), Pipe::NorthWest));
        assert!(matches!(pipe_from_char('F'), Pipe::SouthEast));
        assert!(matches!(pipe_from_char('7'), Pipe::SouthWest));
        assert!(matches!(pipe_from_char('-'), Pipe::EastWest));
        assert!(matches!(pipe_from_char('S'), Pipe::Start));

        for i in 0..255u8 {
            let pipe = pipe_from_char(i as char);
            if "|LJF7-S".contains(i as char) {
                assert!(matches!(
                    &pipe,
                    Pipe::Start
                        | Pipe::NorthEast
                        | Pipe::NorthWest
                        | Pipe::SouthEast
                        | Pipe::SouthWest
                        | Pipe::EastWest
                        | Pipe::NorthSouth
                ));
                assert!(!matches!(pipe, Pipe::None));
            } else {
                assert!(matches!(pipe, Pipe::None));
            }
        }
    }
}
