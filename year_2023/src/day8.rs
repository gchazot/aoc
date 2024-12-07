use aoc_utils as utils;
use std::collections::HashMap;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let map = Map::from_file("day8.txt");
    assert_eq!(19199, camel_steps(&map));
    assert_eq!(13663968099527, ghost_steps(&map));
}

#[test]
fn test_camel_steps() {
    let map1 = Map::from_file("day8-example1.txt");
    assert_eq!(2, camel_steps(&map1));

    let map2 = Map::from_file("day8-example2.txt");
    assert_eq!(6, camel_steps(&map2));
}

fn camel_steps(map: &Map) -> usize {
    let nav = map.navigator("AAA".into(), Some("ZZZ".into()));
    nav.count()
}

#[test]
fn test_ghost_steps() {
    let map3 = Map::from_file("day8-example3.txt");
    assert_eq!(6, ghost_steps(&map3));
}

fn ghost_steps(map: &Map) -> u64 {
    let periods: Vec<_> = map
        .nodes
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|start| ghost_navigator(map, start.clone()).count() as u32)
        .collect();

    let prime_factors = periods.iter().map(utils::prime_factors);

    let mut max_exponents = HashMap::new();
    for factors in prime_factors {
        for (number, exponent) in factors {
            let max_exponent = max_exponents.entry(number).or_insert(0);
            if exponent > *max_exponent {
                *max_exponent = exponent;
            }
        }
    }

    max_exponents
        .iter()
        .map(|(&n, &exponent)| n.pow(exponent) as u64)
        .product()
}

#[test]
fn test_ghost_navigator() {
    let map3 = Map::from_file("day8-example3.txt");
    assert_eq!(2, ghost_navigator(&map3, "11A".into()).count());
    assert_eq!(3, ghost_navigator(&map3, "22A".into()).count());
}
fn ghost_navigator(map: &Map, start: String) -> Navigator {
    let nav = map.navigator(start, None);
    nav
}

#[test]
fn test_navigator() {
    let map1 = Map::from_file("day8-example1.txt");
    let mut nav1 = map1.navigator("AAA".into(), Some("ZZZ".into()));
    assert_eq!(0, nav1.position);
    assert_eq!(Some(String::from("CCC")), nav1.next());
    assert_eq!(1, nav1.position);
    assert_eq!(Some(String::from("ZZZ")), nav1.next());
    assert_eq!(2, nav1.position);
    assert_eq!(None, nav1.next());

    let map1 = Map::from_file("day8-example2.txt");
    let mut nav2 = map1.navigator("AAA".into(), Some("ZZZ".into()));
    assert_eq!(0, nav2.position);
    assert_eq!(Some(String::from("BBB")), nav2.next());
    assert_eq!(1, nav2.position);
    assert_eq!(Some(String::from("AAA")), nav2.next());
    assert_eq!(2, nav2.position);
    assert_eq!(Some(String::from("BBB")), nav2.next());
    assert_eq!(3, nav2.position);
    assert_eq!(Some(String::from("AAA")), nav2.next());
    assert_eq!(1, nav2.position);
    assert_eq!(Some(String::from("BBB")), nav2.next());
    assert_eq!(2, nav2.position);
    assert_eq!(Some(String::from("ZZZ")), nav2.next());
    assert_eq!(3, nav2.position);
    assert_eq!(None, nav2.next());
}

struct Navigator<'a> {
    map: &'a Map,
    location: Option<String>,
    position: usize,
    end: Option<String>,
}

impl<'a> Iterator for Navigator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current_location = &self.location.clone().unwrap();
        let current_node = self.map.nodes.get(current_location).unwrap();

        if self.end.is_some() && self.end == Some(current_node.name.clone()) {
            return None;
        } else if self.end.is_none() && current_node.name.ends_with("Z") {
            return None;
        }

        if self.position >= self.map.instructions.len() {
            self.position = 0;
        }
        let direction = self.map.instructions[self.position];

        self.location = match direction {
            'L' => Some(current_node.left.clone()),
            'R' => Some(current_node.right.clone()),
            _ => panic!("Invalid direction: {direction}"),
        };
        self.position += 1;

        return self.location.clone();
    }
}

#[test]
fn test_parse_map() {
    let map1 = Map::from_file("day8-example1.txt");

    assert_eq!(vec!['R', 'L'], map1.instructions);
    assert_eq!(7, map1.nodes.len());
    assert_eq!("ZZZ", map1.nodes["CCC"].left);
    assert_eq!("GGG", map1.nodes["CCC"].right);

    let map2 = Map::from_file("day8-example2.txt");

    assert_eq!(vec!['L', 'L', 'R'], map2.instructions);
    assert_eq!(3, map2.nodes.len());
    assert_eq!("AAA", map2.nodes["BBB"].left);
    assert_eq!("ZZZ", map2.nodes["BBB"].right);
}

struct Map {
    instructions: Vec<char>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn from_file(filename: &str) -> Map {
        let path = format!("input/{filename}");
        let lines = utils::read_lines(&path);

        let instructions = Vec::<char>::from_iter(lines[0].chars());
        let mut nodes = HashMap::<String, Node>::new();
        for line in &lines[2..] {
            let node = Node::from_line(line);
            nodes.insert(node.name.clone(), node);
        }

        Map {
            instructions,
            nodes,
        }
    }

    fn navigator(&self, start: String, end: Option<String>) -> Navigator {
        Navigator {
            map: self,
            location: Some(start),
            position: 0,
            end,
        }
    }
}

#[test]
fn test_node() {
    let a = Node::from_line("AAA = (BBB, CCC)");
    assert_eq!("AAA", a.name);
    assert_eq!("BBB", a.left);
    assert_eq!("CCC", a.right);

    let a = Node::from_line("BBB = (DDD, EEE)");
    assert_eq!("BBB", a.name);
    assert_eq!("DDD", a.left);
    assert_eq!("EEE", a.right);
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from_line(line: &str) -> Node {
        let (name, directions) = line.split_once(" = ").unwrap();
        let (left, right) = directions
            .trim()
            .trim_matches(|c| "()".contains(c))
            .split_once(", ")
            .unwrap();
        Node {
            name: name.into(),
            left: left.into(),
            right: right.into(),
        }
    }
}
