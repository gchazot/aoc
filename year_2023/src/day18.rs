use std::ops::Index;

pub fn execute() -> String {
    let canvas1 = Canvas::from_lines_part1(aoc_utils::read_lines("input/day18.txt"));
    let part1 = canvas1.to_area();

    let canvas2 = Canvas::from_lines_part2(aoc_utils::read_lines("input/day18.txt"));
    let part2 = canvas2.to_area();

    format!("{} {}", part1, part2)
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
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

struct Line {
    start: Coord,
    end: Coord,
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
                let (distance_str, _color_str) = rest.split_once(" ").unwrap();

                let direction = match dir_str {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!(),
                };
                let distance = distance_str.parse::<i64>().unwrap();

                Instruction {
                    direction,
                    distance,
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

                Instruction {
                    direction,
                    distance,
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
                Line { start, end }
            })
            .collect()
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "62573 54662804037719");
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
}
