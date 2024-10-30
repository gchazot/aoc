use crate::utils;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let mine = Platform::from_lines(utils::read_lines("src/day14/mine.txt"));
    assert_eq!(100, mine.rows.len());

    let mine_slid_north = mine.slide_north();
    assert_eq!(107430, mine_slid_north.load_north());
}

#[derive(Clone, Debug)]
enum Shape {
    Sphere,
    Cube,
}
#[derive(Debug)]
struct Platform {
    rows: Vec<Vec<Option<Shape>>>,
}

impl Platform {
    fn from_lines(lines: Vec<String>) -> Platform {
        let rows = lines
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
        Platform { rows }
    }

    fn slide_north(&self) -> Platform {
        let width = self.rows[0].len();
        assert!(self.rows.iter().all(|row| row.len() == width));

        let mut rows = vec![vec![None; width]; self.rows.len()];

        for col in 0..width {
            let mut current_row: usize = 0;
            for (i, row) in self.rows.iter().enumerate() {
                match row[col] {
                    Some(Shape::Sphere) => {
                        rows[current_row][col] = Some(Shape::Sphere);
                        current_row += 1;
                    }

                    Some(Shape::Cube) => {
                        rows[i][col] = Some(Shape::Cube);
                        current_row = i + 1;
                    }

                    None => {}
                }
            }
        }

        Platform { rows }
    }

    fn load_north(self) -> usize {
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
    let example = _example().slide_north();

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
    let example = _example().slide_north();
    assert_eq!(136, example.load_north());
}
