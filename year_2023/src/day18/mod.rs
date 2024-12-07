use aoc_utils as utils;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let part1 = Canvas::from_lines_part1(utils::read_lines("src/day18/mine.txt"));

    let pixels = part1.to_pixels_filled();
    assert_eq!(62573, pixels.len());
    assert_eq!(62573, part1.to_area());

    // for line in part1.to_lines() {
    //     println!(
    //         "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" stroke-linecap=\"square\"/>",
    //         line.start.x, line.start.y, line.end.x, line.end.y, line.color,
    //     );
    // }

    let part2 = Canvas::from_lines_part2(utils::read_lines("src/day18/mine.txt"));
    assert_eq!(54662804037719, part2.to_area());
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    distance: i64,
    color: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i64,
    y: i64,
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
    fn from_lines_part1(lines: Vec<String>) -> Canvas {
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
                let distance = distance_str.parse::<i64>().unwrap();
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

    fn from_lines_part2(lines: Vec<String>) -> Canvas {
        let instructions = lines
            .iter()
            .map(|line| {
                let (_, instruction_str) = line.rsplit_once(" ").unwrap();
                let distance_str = &instruction_str[2..7];
                let dir_str = &instruction_str[7..8];

                let direction = match dir_str {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => unreachable!(),
                };
                let distance = i64::from_str_radix(distance_str, 16).unwrap();
                let color = "#ffffff".to_string();

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
        let min = Coord {
            x: result.iter().min_by_key(|&coord| coord.x).unwrap().x,
            y: result.iter().min_by_key(|&coord| coord.y).unwrap().y,
        };
        let max = Coord {
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
                    if next.x >= min.x
                        && next.x <= max.x
                        && next.y >= min.y
                        && next.y <= max.y
                        && result.insert(next.clone())
                    {
                        new_frontline.push(next);
                    }
                }
            }
            frontline = new_frontline;
        }

        result
    }

    fn to_area(&self) -> i64 {
        let abs = |x: i64| if x < 0 { -x } else { x };

        let lines = self.to_lines();

        // Uses the trapezoids formula, but only covers the area drawn by the MIDDLE of the trench
        let trapezoids = lines.iter().fold(0, |acc, line| {
            acc + (line.start.x - line.end.x) * (line.start.y + line.end.y)
        }) / 2;
        // So we need to add the other half of the trench
        let missing_edges = lines.iter().fold(0, |acc, line| {
            acc + abs(line.start.x - line.end.x) + abs(line.start.y - line.end.y)
        }) / 2;
        // And the outside corners
        let missing_corners = self.dominant_turns().1 as i64 / 4;

        trapezoids + missing_edges + missing_corners
    }

    fn dominant_turns(&self) -> (Direction, u32) {
        use Direction::*;

        let mut left = 0;
        let mut right = 0;

        for (i, instruction) in self.instructions.iter().enumerate() {
            let prev_index = if i > 0 {
                i - 1
            } else {
                self.instructions.len() - 1
            };
            let prev = self.instructions.index(prev_index);

            match (&prev.direction, &instruction.direction) {
                (Left, Up) | (Right, Down) | (Up, Right) | (Down, Left) => right += 1,
                (Left, Down) | (Right, Up) | (Up, Left) | (Down, Right) => left += 1,
                _ => unreachable!(),
            };
        }

        if left > right {
            (Left, left - right)
        } else {
            (Right, right - left)
        }
    }
}

#[test]
fn test_from_lines() {
    use Direction::*;

    let example = Canvas::from_lines_part1(_example());

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

fn _example() -> Vec<String> {
    vec![
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
    ]
}

#[test]
fn test_to_lines() {
    let example = Canvas::from_lines_part1(_example());
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
    let example = Canvas::from_lines_part2(_example());
    for line in example.to_lines() {
        println!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"100\" stroke-linecap=\"square\"/>",
            line.start.x, line.start.y, line.end.x, line.end.y, line.color,
        );
    }
}

#[test]
fn test_to_pixels_outline() {
    let example = Canvas::from_lines_part1(_example());
    let pixels = example.to_pixels_outline();

    assert_eq!(38, pixels.len())
}

#[test]
fn test_to_pixels_filled() {
    let part1 = Canvas::from_lines_part1(_example());
    let pixels_part1 = part1.to_pixels_filled();
    assert_eq!(62, pixels_part1.len());
}

#[test]
fn test_to_area() {
    let part1 = Canvas::from_lines_part1(_example());
    assert_eq!(62, part1.to_area());

    let part2 = Canvas::from_lines_part2(_example());
    assert_eq!(952408144115, part2.to_area());
}

#[test]
fn test_dominant_turns() {
    let part1 = Canvas::from_lines_part1(_example());

    assert_eq!((Direction::Right, 4), part1.dominant_turns())
}
