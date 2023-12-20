use crate::utils;
use std::collections::HashMap;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let map = Map::from_file("mine.txt");
    assert_eq!(19199, camel_steps(map));
}

#[test]
fn test_path_counter() {
    let map1 = Map::from_file("example1.txt");
    assert_eq!(2, camel_steps(map1));

    let map2 = Map::from_file("example2.txt");
    assert_eq!(6, camel_steps(map2));
}

fn camel_steps(map: Map) -> usize {
    let nav1 = map.navigator();
    nav1.count() - 1
}

#[test]
fn test_camel_walk() {
    let map1 = Map::from_file("example1.txt");
    let mut nav1 = map1.navigator();
    assert_eq!(None, nav1.location);
    assert_eq!(0, nav1.position);
    assert_eq!(Some(String::from("AAA")), nav1.next());
    assert_eq!(0, nav1.position);
    assert_eq!(Some(String::from("CCC")), nav1.next());
    assert_eq!(1, nav1.position);
    assert_eq!(Some(String::from("ZZZ")), nav1.next());
    assert_eq!(2, nav1.position);
    assert_eq!(None, nav1.next());

    let map1 = Map::from_file("example2.txt");
    let mut nav2 = map1.navigator();
    assert_eq!(None, nav2.location);
    assert_eq!(0, nav2.position);
    assert_eq!(Some(String::from("AAA")), nav2.next());
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
}

impl<'a> Iterator for Navigator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.location.is_none() {
            self.location = Some(String::from("AAA"))
        } else {
            let current_location = &self.location.clone().unwrap();
            let current_node = self.map.nodes.get(current_location).unwrap();

            if current_node.name == "ZZZ" {
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
        }
        return self.location.clone();
    }
}

#[test]
fn test_parse_map() {
    let map1 = Map::from_file("example1.txt");

    assert_eq!(vec!['R', 'L'], map1.instructions);
    assert_eq!(7, map1.nodes.len());
    assert_eq!("ZZZ", map1.nodes["CCC"].left);
    assert_eq!("GGG", map1.nodes["CCC"].right);

    let map2 = Map::from_file("example2.txt");

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
        let path = format!("src/day8/{filename}");
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

    fn navigator(&self) -> Navigator {
        Navigator {
            map: self,
            location: None,
            position: 0,
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
