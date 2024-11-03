use crate::utils;
use std::collections::{HashMap, HashSet};

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let mine = Canvas::from_lines(utils::read_lines("src/day18/mine.txt"));

    let pixels = mine.to_pixels_filled();

    assert_eq!(62573, pixels.len());

    // for line in mine.to_lines() {
    //     println!(
    //         "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" stroke-linecap=\"square\"/>",
    //         line.start.x, line.start.y, line.end.x, line.end.y, line.color,
    //     );
    // }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    distance: i32,
    color: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

struct Line {
    start: Coord,
    end: Coord,
    color: String,
}

struct Canvas {
    instructions: Vec<Instruction>,
}

impl Canvas {
    fn from_lines(lines: Vec<String>) -> Canvas {
        let instructions = lines
            .iter()
            .map(|line| {
                let (dir_str, rest) = line.split_once(" ").unwrap();
                let (distance_str, color_str) = rest.split_once(" ").unwrap();

                let direction = match dir_str {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!(),
                };
                let distance = distance_str.parse::<i32>().unwrap();
                let color = color_str
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string();

                Instruction {
                    direction,
                    distance,
                    color,
                }
            })
            .collect();
        Canvas { instructions }
    }

    fn to_lines(&self) -> Vec<Line> {
        use Direction::*;

        let mut current = Coord { x: 0, y: 0 };
        self.instructions
            .iter()
            .map(|instruction| {
                let start = current.clone();
                match instruction.direction {
                    Up => current.y -= instruction.distance,
                    Down => current.y += instruction.distance,
                    Left => current.x -= instruction.distance,
                    Right => current.x += instruction.distance,
                }
                let end = current.clone();
                Line {
                    start,
                    end,
                    color: instruction.color.clone(),
                }
            })
            .collect()
    }

    fn to_pixels_outline(&self) -> HashMap<Coord, String> {
        use Direction::*;

        let mut result = HashMap::new();

        let mut current = Coord { x: 0, y: 0 };
        for instruction in self.instructions.iter() {
            for _ in 0..instruction.distance {
                match instruction.direction {
                    Up => current.y -= 1,
                    Down => current.y += 1,
                    Left => current.x -= 1,
                    Right => current.x += 1,
                }
                result.insert(current.clone(), instruction.color.clone());
            }
        }

        result
    }

    fn to_pixels_filled(&self) -> HashSet<Coord> {
        let mut result = self.to_pixels_outline().into_keys().collect::<HashSet<_>>();
        let mut min = Coord {
            x: result.iter().min_by_key(|&coord| coord.x).unwrap().x,
            y: result.iter().min_by_key(|&coord| coord.y).unwrap().y,
        };
        let mut max = Coord {
            x: result.iter().max_by_key(|&coord| coord.x).unwrap().x,
            y: result.iter().max_by_key(|&coord| coord.y).unwrap().y,
        };

        let start = Coord {
            x: (min.x + max.x) / 2,
            y: (min.y + max.y) / 2,
        };

        let mut frontline = vec![start.clone()];
        result.insert(start);

        while !frontline.is_empty() {
            let mut new_frontline = vec![];
            for current in frontline.iter() {
                for next in vec![
                    Coord {
                        x: current.x + 1,
                        y: current.y,
                    },
                    Coord {
                        x: current.x,
                        y: current.y + 1,
                    },
                    Coord {
                        x: current.x - 1,
                        y: current.y,
                    },
                    Coord {
                        x: current.x,
                        y: current.y - 1,
                    },
                ] {
                    if result.insert(next.clone()) {
                        new_frontline.push(next);
                    }
                }
            }
            frontline = new_frontline;
        }

        result
    }
}

#[test]
fn test_from_lines() {
    use Direction::*;

    let example = _example();

    assert_eq!(example.instructions.len(), 14);

    assert!(matches!(example.instructions[0].direction, Right));
    assert!(matches!(example.instructions[1].direction, Down));
    assert!(matches!(example.instructions[2].direction, Left));
    assert!(matches!(example.instructions[13].direction, Up));

    assert_eq!(example.instructions[0].distance, 6);
    assert_eq!(example.instructions[1].distance, 5);
    assert_eq!(example.instructions[2].distance, 2);
    assert_eq!(example.instructions[13].distance, 2);

    assert_eq!(example.instructions[0].color, "#70c710".to_string());
    assert_eq!(example.instructions[1].color, "#0dc571".to_string());
    assert_eq!(example.instructions[2].color, "#5713f0".to_string());
    assert_eq!(example.instructions[13].color, "#7a21e3".to_string());
}

fn _example() -> Canvas {
    Canvas::from_lines(vec![
        "R 6 (#70c710)".to_string(),
        "D 5 (#0dc571)".to_string(),
        "L 2 (#5713f0)".to_string(),
        "D 2 (#d2c081)".to_string(),
        "R 2 (#59c680)".to_string(),
        "D 2 (#411b91)".to_string(),
        "L 5 (#8ceee2)".to_string(),
        "U 2 (#caa173)".to_string(),
        "L 1 (#1b58a2)".to_string(),
        "U 2 (#caa171)".to_string(),
        "R 2 (#7807d2)".to_string(),
        "U 3 (#a77fa3)".to_string(),
        "L 2 (#015232)".to_string(),
        "U 2 (#7a21e3)".to_string(),
    ])
}

#[test]
fn test_to_lines() {
    let example = _example();
    let lines = example.to_lines();

    assert_eq!(lines.len(), 14);

    assert_eq!(Coord { x: 0, y: 0 }, lines[0].start);
    assert_eq!(Coord { x: 6, y: 0 }, lines[0].end);
    assert_eq!(Coord { x: 6, y: 0 }, lines[1].start);
    assert_eq!(Coord { x: 6, y: 5 }, lines[1].end);
    assert_eq!(Coord { x: 6, y: 5 }, lines[2].start);
    assert_eq!(Coord { x: 4, y: 5 }, lines[2].end);
    assert_eq!(Coord { x: 0, y: 2 }, lines[13].start);
    assert_eq!(Coord { x: 0, y: 0 }, lines[13].end);

    assert_eq!(lines[0].color, "#70c710".to_string());
    assert_eq!(lines[1].color, "#0dc571".to_string());
    assert_eq!(lines[2].color, "#5713f0".to_string());
    assert_eq!(lines[13].color, "#7a21e3".to_string());
}

#[test]
#[ignore]
fn test_make_svg() {
    let example = _example();
    for line in example.to_lines() {
        println!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" stroke-linecap=\"square\"/>",
            line.start.x, line.start.y, line.end.x, line.end.y, line.color,
        );
    }
}

#[test]
fn test_to_pixels_outline() {
    let example = _example();
    let pixels = example.to_pixels_outline();

    assert_eq!(38, pixels.len())
}

#[test]
fn test_to_pixels_filled() {
    let example = _example();
    let pixels = example.to_pixels_filled();

    assert_eq!(62, pixels.len())
}
