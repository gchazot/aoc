pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day12.txt");

    let puzzle = Puzzle::from_lines(data);

    let part1 = puzzle.count_feasible();
    let part2 = "_";

    format!("{} {}", part1, part2)
}

struct Puzzle {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Puzzle {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut blocks = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

        let regions = blocks
            .pop()
            .unwrap()
            .into_iter()
            .map(|line| Region::from_line(line.clone()))
            .collect();

        let shapes = blocks
            .into_iter()
            .map(|lines| Shape::from_lines(lines.into()))
            .collect();

        Self { shapes, regions }
    }

    fn count_feasible(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| region.is_feasible(&self.shapes))
            .count()
    }
}

struct Shape {
    id: u8,
    bits: [[bool; 3]; 3],

    size: usize,
}

impl Shape {
    fn from_lines(lines: Vec<String>) -> Self {
        let id = lines[0].strip_suffix(":").unwrap().parse().unwrap();
        let mut bits = [[false; 3]; 3];
        for (i, line) in lines[1..].iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                bits[i][j] = c == '#';
            }
        }

        let size = bits
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&bit| if bit { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum();
        Self { id, bits, size }
    }
}

struct Region {
    width: usize,
    height: usize,
    shapes_required: Vec<usize>,
}

impl Region {
    fn from_line(line: String) -> Self {
        let (dims, requirements) = line.split_once(": ").unwrap();

        let (width, height) = dims.split_once('x').unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();

        let shapes_required = requirements
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            width,
            height,
            shapes_required,
        }
    }

    fn is_feasible(&self, shapes: &Vec<Shape>) -> bool {
        let available_area = self.width * self.height;
        let required_area = self
            .shapes_required
            .iter()
            .zip(shapes.iter())
            .map(|(n, shape)| n * shape.size)
            .sum();
        available_area >= required_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "531 _");
    }

    #[test]
    fn test_is_feasible() {
        let puzzle = Puzzle::from_lines(example());

        assert!(puzzle.regions[0].is_feasible(&puzzle.shapes));
        assert!(puzzle.regions[1].is_feasible(&puzzle.shapes));
        assert!(puzzle.regions[2].is_feasible(&puzzle.shapes));
    }
    #[test]
    fn test_puzzle_from_lines() {
        let puzzle = Puzzle::from_lines(example());

        assert_eq!(puzzle.shapes.len(), 6);
        assert_eq!(puzzle.regions.len(), 3);

        assert_eq!(puzzle.regions[0].width, 4);
        assert_eq!(puzzle.regions[0].height, 4);
        assert_eq!(puzzle.regions[0].shapes_required, vec![0, 0, 0, 0, 2, 0]);

        assert_eq!(puzzle.regions[1].width, 12);
        assert_eq!(puzzle.regions[1].height, 5);
        assert_eq!(puzzle.regions[1].shapes_required, vec![1, 0, 1, 0, 2, 2]);

        assert_eq!(puzzle.regions[2].width, 12);
        assert_eq!(puzzle.regions[2].height, 5);
        assert_eq!(puzzle.regions[2].shapes_required, vec![1, 0, 1, 0, 3, 2]);
    }

    #[test]
    fn test_shape_from_lines() {
        let lines = vec![
            String::from("0:"),
            String::from("###"),
            String::from("##."),
            String::from("##."),
        ];
        let shape = Shape::from_lines(lines);
        assert_eq!(shape.id, 0);
        assert_eq!(
            shape.bits,
            [[true, true, true], [true, true, false], [true, true, false]]
        );
    }

    #[test]
    fn test_region_from_line() {
        let line = String::from("12x5: 1 0 1 0 2 2");
        let region = Region::from_line(line);
        assert_eq!(region.width, 12);
        assert_eq!(region.height, 5);
        assert_eq!(region.shapes_required, vec![1, 0, 1, 0, 2, 2]);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("0:"),
            String::from("###"),
            String::from("##."),
            String::from("##."),
            String::from(""),
            String::from("1:"),
            String::from("###"),
            String::from("##."),
            String::from(".##"),
            String::from(""),
            String::from("2:"),
            String::from(".##"),
            String::from("###"),
            String::from("##."),
            String::from(""),
            String::from("3:"),
            String::from("##."),
            String::from("###"),
            String::from("##."),
            String::from(""),
            String::from("4:"),
            String::from("###"),
            String::from("#.."),
            String::from("###"),
            String::from(""),
            String::from("5:"),
            String::from("###"),
            String::from(".#."),
            String::from("###"),
            String::from(""),
            String::from("4x4: 0 0 0 0 2 0"),
            String::from("12x5: 1 0 1 0 2 2"),
            String::from("12x5: 1 0 1 0 3 2"),
        ]
    }
}
