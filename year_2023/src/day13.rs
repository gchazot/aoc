use std::fmt;

pub fn execute() -> String {
    let mut patterns: Vec<Pattern> = aoc_utils::read_lines("input/day13.txt")
        .split(|line| line.len() == 0)
        .map(|pattern_lines| Pattern::from_lines(pattern_lines.to_vec()))
        .collect();

    let part1 = patterns
        .iter()
        .map(|pattern| pattern.symmetry_score().unwrap())
        .sum::<usize>();
    let part2 = patterns
        .iter_mut()
        .map(|pattern| pattern.new_symmetry_score().unwrap())
        .sum::<usize>();

    format!("{} {}", part1, part2)
}

#[derive(Clone)]
struct Pattern {
    rows: Vec<Vec<bool>>,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rows
                .iter()
                .map(|row| -> String {
                    row.iter()
                        .map(|&val| -> String {
                            match val {
                                true => String::from("#"),
                                false => String::from("."),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Pattern {
    fn from_lines(lines: Vec<String>) -> Pattern {
        let rows = lines
            .iter()
            .map(|line| line.chars().map(|ch| ch == '#').collect())
            .collect();
        Pattern { rows }
    }

    fn new_symmetry_score(&mut self) -> Option<usize> {
        let orig_row_symmetries = self.row_symmetries();
        let orig_col_symmetries = self.col_symmetries();
        assert_eq!(1, orig_row_symmetries.len() + orig_col_symmetries.len());

        for j in 0..self.rows.len() {
            for i in 0..self.rows[j].len() {
                self.rows[j][i] = !self.rows[j][i];

                let row_symmetries: Vec<usize> = self
                    .row_symmetries()
                    .into_iter()
                    .filter(|row| !orig_row_symmetries.contains(row))
                    .collect();

                if row_symmetries.len() == 1 {
                    return Some(row_symmetries.into_iter().sum());
                } else if row_symmetries.len() > 1 {
                    return None;
                }

                let col_symmetries: Vec<usize> = self
                    .col_symmetries()
                    .into_iter()
                    .filter(|col| !orig_col_symmetries.contains(col))
                    .collect();
                if col_symmetries.len() == 1 {
                    return Some(col_symmetries.into_iter().sum::<usize>() * 100);
                }

                self.rows[j][i] = !self.rows[j][i];
            }
        }
        println!("No smudge found\n{:?}", self);
        None
    }

    fn symmetry_score(&self) -> Option<usize> {
        let row_symmetries = self.row_symmetries();
        let col_symmetries = self.col_symmetries();
        assert_eq!(1, row_symmetries.len() + col_symmetries.len());

        if row_symmetries.len() == 1 {
            return Some(row_symmetries.iter().sum());
        } else if row_symmetries.len() > 1 {
            return None;
        }
        if col_symmetries.len() == 1 {
            return Some(col_symmetries.iter().sum::<usize>() * 100);
        }
        None
    }

    fn is_row_symmetric(&self, (x, y): (usize, usize)) -> bool {
        let row = &self.rows[y];
        if x < 1 || x >= row.len() {
            return false;
        }

        let mut i = 0;
        while x + i < row.len() && x >= i + 1 {
            if row[x + i] != row[x - i - 1] {
                return false;
            }
            i += 1;
        }
        true
    }

    fn row_symmetries(&self) -> Vec<usize> {
        let width = self.rows[0].len();
        let height = self.rows.len();
        assert!(self.rows.iter().all(|row| row.len() == width));
        let mut result: Vec<usize> = (1..width).collect();
        for j in 0..height {
            result.retain(|&i| self.is_row_symmetric((i, j)));
        }
        result
    }

    fn is_col_symmetric(&self, (x, y): (usize, usize)) -> bool {
        let col: Vec<bool> = self.rows.iter().map(|row| row[x]).collect();
        if y < 1 || y >= col.len() {
            return false;
        }

        let mut j = 0;
        while y + j < col.len() && y >= j + 1 {
            if col[y + j] != col[y - j - 1] {
                return false;
            }
            j += 1;
        }
        true
    }

    fn col_symmetries(&self) -> Vec<usize> {
        let width = self.rows[0].len();
        assert!(self.rows.iter().all(|row| row.len() == width));
        let height = self.rows.len();

        let mut result: Vec<usize> = (1..height).collect();
        for i in 0..width {
            result.retain(|&j| self.is_col_symmetric((i, j)));
        }
        result
    }
}

fn _example1() -> Pattern {
    Pattern::from_lines(vec![
        "#.##..##.".to_string(),
        "..#.##.#.".to_string(),
        "##......#".to_string(),
        "##......#".to_string(),
        "..#.##.#.".to_string(),
        "..##..##.".to_string(),
        "#.#.##.#.".to_string(),
    ])
}

fn _example2() -> Pattern {
    Pattern::from_lines(vec![
        "#...##..#".to_string(),
        "#....#..#".to_string(),
        "..##..###".to_string(),
        "#####.##.".to_string(),
        "#####.##.".to_string(),
        "..##..###".to_string(),
        "#....#..#".to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "28651 25450");
    }

    #[test]
    fn test_from_lines() {
        assert_eq!(0, Pattern::from_lines(vec![]).rows.len());
        assert_eq!(1, Pattern::from_lines(vec!["".to_string()]).rows.len());

        let example1 = _example1();

        assert_eq!(7, example1.rows.len());
        assert_eq!(9, example1.rows.iter().map(|row| row.len()).min().unwrap());
        assert_eq!(9, example1.rows.iter().map(|row| row.len()).max().unwrap());
        assert_eq!(
            vec![true, false, true, true, false, false, true, true, false],
            example1.rows[0]
        );

        let example2 = _example2();
        assert_eq!(7, example2.rows.len());
        assert_eq!(9, example2.rows.iter().map(|row| row.len()).min().unwrap());
        assert_eq!(9, example2.rows.iter().map(|row| row.len()).max().unwrap());
        assert_eq!(
            vec![true, false, false, false, true, true, false, false, true],
            example2.rows[0]
        );
    }

    #[test]
    fn test_is_row_symmetric() {
        let example1 = _example1();

        let symmetries = vec![
            (5, 0),
            (7, 0),
            (1, 1),
            (5, 1),
            (1, 2),
            (5, 2),
            (1, 3),
            (5, 3),
            (1, 4),
            (5, 4),
            (1, 5),
            (3, 5),
            (5, 5),
            (7, 5),
            (5, 6),
        ];

        for j in 0..example1.rows.len() {
            for i in 0..example1.rows[j].len() {
                let coords = (i, j);
                let result = example1.is_row_symmetric(coords);
                if symmetries.contains(&coords) {
                    assert!(result, "{:?} is not symmetric", &coords);
                } else {
                    assert!(!result, "{:?} is symmetric", &coords);
                }
            }
        }
    }

    #[test]
    fn test_row_symmetries() {
        let example1 = _example1();
        let symmetries1 = example1.row_symmetries();
        assert_eq!(1, symmetries1.len());
        assert_eq!(5, symmetries1[0]);

        let example2 = _example2();
        let symmetries2 = example2.row_symmetries();
        assert_eq!(0, symmetries2.len());
    }

    #[test]
    fn test_col_symmetries() {
        let example1 = _example1();
        let symmetries1 = example1.col_symmetries();
        assert_eq!(0, symmetries1.len());

        let example2 = _example2();
        let symmetries2 = example2.col_symmetries();
        assert_eq!(1, symmetries2.len());
        assert_eq!(4, symmetries2[0]);
    }

    #[test]
    fn test_new_score() {
        let mut example = vec![_example1(), _example2()];
        assert_eq!(
            400,
            example
                .iter_mut()
                .map(|pattern| { pattern.new_symmetry_score().unwrap() })
                .sum::<usize>()
        )
    }
}
