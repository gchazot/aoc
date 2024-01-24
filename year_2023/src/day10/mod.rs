use crate::utils;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let map = PipeMap::from_file("mine.txt");
    let net = map.to_network();
    let length = net.pipe_length(map.start().unwrap());

    assert_eq!(13884, length);
    assert_eq!(6942, length / 2);
}

#[test]
fn test_pipe_length() {
    let map3 = PipeMap::from_file("example3.txt");
    let net3 = map3.to_network();
    let length = net3.pipe_length(map3.start().unwrap());
    assert_eq!(8, length);

    // This is the furthest point away
    assert_eq!(4, length / 2);
}

#[test]
fn test_clean() {
    let map2 = PipeMap::from_file("example2.txt");
    let net2 = map2.to_network();
    let map3 = PipeMap::from_file("example3.txt");
    let net3 = map3.to_network();
    let clean3 = net3.clean(map3.start().unwrap());

    assert_eq!(net2.connections, clean3.connections);
}

#[test]
fn test_map_to_network() {
    let net1 = PipeMap::from_file("example1.txt").to_network();

    assert_eq!(8, net1.len());

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

    let net2 = PipeMap::from_file("example2.txt").to_network();
    assert_eq!(net1.len(), net2.len());
    assert_eq!(net1.connections, net2.connections);

    let net3 = PipeMap::from_file("example3.txt").to_network();

    assert!(net3.len() >= net1.len());
    for (pos, neighbours) in net1.connections.iter() {
        assert_eq!(&net3.get(pos), neighbours);
    }
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

    fn len(&self) -> usize {
        self.connections.len()
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

#[test]
fn test_parse_pipe_map() {
    let map1 = PipeMap::from_file("example1.txt");
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

    let map2 = PipeMap::from_file("example2.txt");
    assert_eq!(5 * 5, map2.nodes.len());
    assert!(matches!(map2.start(), Some(Position(2, 2))));
    assert!(matches!(map2.nodes[&Position(1, 1)], Pipe::None));
    assert!(matches!(map2.nodes[&Position(2, 2)], Pipe::Start));
    assert!(matches!(map2.nodes[&Position(4, 2)], Pipe::SouthWest));
    assert!(matches!(map2.nodes[&Position(2, 4)], Pipe::NorthEast));

    let map3 = PipeMap::from_file("example3.txt");
    assert_eq!(5 * 5, map3.nodes.len());
    assert!(matches!(map3.start(), Some(Position(2, 2))));
    assert!(matches!(map3.nodes[&Position(1, 1)], Pipe::EastWest));
    assert!(matches!(map3.nodes[&Position(2, 2)], Pipe::Start));
    assert!(matches!(map3.nodes[&Position(5, 1)], Pipe::SouthWest));
    assert!(matches!(map3.nodes[&Position(1, 5)], Pipe::NorthEast));
    assert!(matches!(map3.nodes[&Position(5, 5)], Pipe::SouthEast));
}

struct PipeMap {
    nodes: HashMap<Position, Pipe>,
}

impl PipeMap {
    fn from_file(filename: &str) -> PipeMap {
        let path = format!("src/day10/{}", &filename);
        let lines = utils::read_lines(&path);

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

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
