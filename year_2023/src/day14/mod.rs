use aoc_utils as utils;
use std::collections::HashMap;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let mut mine = Platform::from_lines(utils::read_lines("src/day14/mine.txt"));
    assert_eq!(100, mine.rows.len());

    mine.slide_north();
    assert_eq!(107430, mine.load_north());

    let mut mine_cycled = Platform::from_lines(utils::read_lines("src/day14/mine.txt"));
    mine_cycled.cycle_much(1000000000);
    assert_eq!(96317, mine_cycled.load_north()); // 96314 < n < 96344
}

#[derive(Clone, Debug)]
enum Shape {
    Sphere,
    Cube,
}
#[derive(Clone, Debug)]
struct Platform {
    rows: Vec<Vec<Option<Shape>>>,
    width: usize,
}

impl Platform {
    fn to_lines(&self) -> Vec<String> {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|shape| match shape {
                        Some(Shape::Sphere) => 'O'.to_string(),
                        Some(Shape::Cube) => '#'.to_string(),
                        None => '.'.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect()
    }

    fn from_lines(lines: Vec<String>) -> Platform {
        let rows: Vec<Vec<Option<Shape>>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => Some(Shape::Sphere),
                        '#' => Some(Shape::Cube),
                        '.' => None,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let width = rows[0].len();
        assert!(rows.iter().all(|row| row.len() == width));

        Platform { rows, width }
    }

    fn cycle_much(&mut self, loops: u32) {
        let mut cache = HashMap::new();

        let mut loops_remaining = loops;

        let mut i: u32 = 0;
        while i < 1000 {
            let cache_key = self.to_lines().join("\n");
            if cache.contains_key(&cache_key) {
                let first_seen = cache.get(&cache_key).unwrap();
                let loop_size = i - first_seen;
                loops_remaining = loops_remaining % loop_size;
                break;
            }

            self.cycle();
            loops_remaining -= 1;

            cache.insert(cache_key, i);
            i += 1;
        }

        for _ in 0..loops_remaining {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    fn slide_north(&mut self) {
        for col in 0..self.width {
            let mut current_row = 0;
            for j in 0..self.rows.len() {
                let row = j;
                match self.rows[row][col] {
                    Some(Shape::Sphere) => {
                        self.rows[row][col] = None;
                        self.rows[current_row][col] = Some(Shape::Sphere);
                        current_row += 1;
                    }

                    Some(Shape::Cube) => {
                        current_row = j + 1;
                    }

                    None => {}
                }
            }
        }
    }

    fn slide_south(&mut self) {
        for col in 0..self.width {
            let mut current_row = 0;
            for j in 0..self.rows.len() {
                let row = self.rows.len() - 1 - j;
                match self.rows[row][col] {
                    Some(Shape::Sphere) => {
                        self.rows[row][col] = None;
                        let sphere = self.rows.len() - 1 - current_row;
                        self.rows[sphere][col] = Some(Shape::Sphere);
                        current_row += 1;
                    }

                    Some(Shape::Cube) => {
                        current_row = j + 1;
                    }

                    None => {}
                }
            }
        }
    }

    fn slide_west(&mut self) {
        for j in 0..self.rows.len() {
            let mut current_col = 0;
            for i in 0..self.width {
                let col = i;
                match self.rows[j][col] {
                    Some(Shape::Sphere) => {
                        self.rows[j][col] = None;
                        self.rows[j][current_col] = Some(Shape::Sphere);
                        current_col += 1;
                    }

                    Some(Shape::Cube) => {
                        current_col = i + 1;
                    }

                    None => {}
                }
            }
        }
    }

    fn slide_east(&mut self) {
        for j in 0..self.rows.len() {
            let mut current_col = 0;
            for i in 0..self.width {
                let col = self.width - 1 - i;
                match self.rows[j][col] {
                    Some(Shape::Sphere) => {
                        self.rows[j][col] = None;
                        self.rows[j][self.width - 1 - current_col] = Some(Shape::Sphere);
                        current_col += 1;
                    }

                    Some(Shape::Cube) => {
                        current_col = i + 1;
                    }

                    None => {}
                }
            }
        }
    }

    fn load_north(&self) -> usize {
        self.rows
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter(|elem| matches!(elem, Some(Shape::Sphere)))
                    .count()
                    * (i + 1)
            })
            .sum()
    }
}

#[test]
fn test_from_lines() {
    let empty = Platform::from_lines(vec!["".to_string()]);
    assert_eq!(1, empty.rows.len());
    assert_eq!(0, empty.rows[0].len());

    let example = _example();
    assert_eq!(10, example.rows.len());
    assert_eq!(10, example.rows[0].len());
}

fn _example() -> Platform {
    Platform::from_lines(vec![
        "O....#....".to_string(),
        "O.OO#....#".to_string(),
        ".....##...".to_string(),
        "OO.#O....O".to_string(),
        ".O.....O#.".to_string(),
        "O.#..O.#.#".to_string(),
        "..O..#O..O".to_string(),
        ".......O..".to_string(),
        "#....###..".to_string(),
        "#OO..#....".to_string(),
    ])
}

#[test]
fn test_slide_north() {
    let mut example = _example();
    example.slide_north();

    assert!(matches!(example.rows[0][0], Some(Shape::Sphere)));
    assert!(matches!(example.rows[1][0], Some(Shape::Sphere)));
    assert!(matches!(example.rows[2][0], Some(Shape::Sphere)));
    assert!(matches!(example.rows[3][0], Some(Shape::Sphere)));
    assert!(matches!(example.rows[4][0], None));
    assert!(matches!(example.rows[5][0], None));
    assert!(matches!(example.rows[6][0], None));
    assert!(matches!(example.rows[7][0], None));
    assert!(matches!(example.rows[8][0], Some(Shape::Cube)));
    assert!(matches!(example.rows[9][0], Some(Shape::Cube)));

    assert!(matches!(example.rows[0][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[1][5], None));
    assert!(matches!(example.rows[2][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[3][5], Some(Shape::Sphere)));
    assert!(matches!(example.rows[4][5], None));
    assert!(matches!(example.rows[5][5], None));
    assert!(matches!(example.rows[6][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[7][5], None));
    assert!(matches!(example.rows[8][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[9][5], Some(Shape::Cube)));
}

#[test]
fn test_load_north() {
    let mut example = _example();
    example.slide_north();
    assert_eq!(136, example.load_north());
}

#[test]
fn test_cycle() {
    let mut example = _example();
    example.cycle();

    assert!(matches!(example.rows[0][0], None));
    assert!(matches!(example.rows[1][0], None));
    assert!(matches!(example.rows[2][0], None));
    assert!(matches!(example.rows[3][0], None));
    assert!(matches!(example.rows[4][0], None));
    assert!(matches!(example.rows[5][0], None));
    assert!(matches!(example.rows[6][0], None));
    assert!(matches!(example.rows[7][0], None));
    assert!(matches!(example.rows[8][0], Some(Shape::Cube)));
    assert!(matches!(example.rows[9][0], Some(Shape::Cube)));

    assert!(matches!(example.rows[0][1], None));
    assert!(matches!(example.rows[1][1], None));
    assert!(matches!(example.rows[2][1], None));
    assert!(matches!(example.rows[3][1], Some(Shape::Sphere)));
    assert!(matches!(example.rows[4][1], None));
    assert!(matches!(example.rows[5][1], Some(Shape::Sphere)));
    assert!(matches!(example.rows[6][1], None));
    assert!(matches!(example.rows[7][1], None));
    assert!(matches!(example.rows[8][1], None));
    assert!(matches!(example.rows[9][1], None));

    assert!(matches!(example.rows[0][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[1][5], None));
    assert!(matches!(example.rows[2][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[3][5], None));
    assert!(matches!(example.rows[4][5], Some(Shape::Sphere)));
    assert!(matches!(example.rows[5][5], None));
    assert!(matches!(example.rows[6][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[7][5], None));
    assert!(matches!(example.rows[8][5], Some(Shape::Cube)));
    assert!(matches!(example.rows[9][5], Some(Shape::Cube)));
}

#[test]
fn test_cycle_much() {
    let mut example = _example();
    example.cycle_much(1000000000);
    assert_eq!(64, example.load_north());
}
