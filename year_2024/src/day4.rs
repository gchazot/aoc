use std::fmt::{Debug, Display, Formatter};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day4.txt");
    let mine = Puzzle::from_lines(data);
    let part1 = mine.count_all();
    let part2 = 456;

    format!("{} {}", part1, part2)
}

#[derive(Debug, Clone)]
struct Puzzle {
    size: usize,
    rows: Vec<Vec<char>>,
}

impl Puzzle {
    fn from_lines(lines: Vec<String>) -> Self {
        let rows = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = rows.len();
        for row in rows.iter() {
            assert_eq!(row.len(), size);
        }
        Self { size, rows }
    }

    fn rotated(&self) -> Self {
        let mut result = self.clone();
        for i in 0..self.size {
            for j in 0..self.size {
                result.rows[i][j] = self.rows[j][self.size - 1 - i];
            }
        }
        result
    }

    fn count_in_rows(&self) -> usize {
        const WORD: &'static str = "XMAS";
        self.rows
            .iter()
            .map(|row| {
                (0..self.size - WORD.len() + 1)
                    .map(|i| {
                        if WORD.chars().enumerate().all(|(u, cw)| row[i + u] == cw) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn count_in_diag(&self) -> usize {
        const WORD: &'static str = "XMAS";
        let mut total = 0;

        for i in 0..self.size - WORD.len() + 1 {
            'a: for j in i..self.size - WORD.len() + 1 {
                for (u, c) in WORD.chars().enumerate() {
                    if self.rows[i + u][j + u] != c {
                        continue 'a;
                    }
                }
                total += 1;
            }
        }
        for i in 1..self.size - WORD.len() + 1 {
            'a: for j in 0..i {
                for (u, c) in WORD.chars().enumerate() {
                    if self.rows[i + u][j + u] != c {
                        continue 'a;
                    }
                }
                total += 1;
            }
        }
        total
    }

    fn count_all(&self) -> usize {
        let mut total = 0;

        total += self.count_in_rows();
        total += self.count_in_diag();

        let rotated1 = self.rotated();
        total += rotated1.count_in_rows();
        total += rotated1.count_in_diag();

        let rotated2 = rotated1.rotated();
        total += rotated2.count_in_rows();
        total += rotated2.count_in_diag();

        let rotated3 = rotated2.rotated();
        total += rotated3.count_in_rows();
        total += rotated3.count_in_diag();

        total
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines = self.rows.iter().map(|row| row.iter().collect::<String>());
        let result = lines.collect::<Vec<_>>().join("\n");
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        // 2416 too low
        // 2425 too low
        // 2455 too low
        assert_eq!(execute(), "2562 456");
    }

    #[test]
    fn test_from_lines() {
        let example = Puzzle::from_lines(_example());
        assert_eq!(example.rows.len(), 10);
        let i = 1;
        for (j, c) in "MSAMXMSMSA".chars().enumerate() {
            assert_eq!(
                example.rows[1][j], c,
                "{},{} should be {} but is {}",
                i, j, c, example.rows[i][j]
            );
        }
    }

    #[test]
    fn test_rotated() {
        let example = Puzzle::from_lines(_example());
        let rotated = example.rotated();

        assert_eq!(example.rows.len(), 10);
        let i = 1;
        for (j, c) in "SSMMMMSAMS".chars().enumerate() {
            assert_eq!(
                rotated.rows[i][j], c,
                "{},{} should be {} but is {}",
                i, j, c, example.rows[i][j]
            );
        }
    }

    #[test]
    fn test_count_in_rows() {
        let example = Puzzle::from_lines(_example());
        assert_eq!(example.count_in_rows(), 3);
        let rotated1 = example.rotated();
        assert_eq!(rotated1.count_in_rows(), 1);
        let rotated2 = rotated1.rotated();
        assert_eq!(rotated2.count_in_rows(), 2);
        let rotated3 = rotated2.rotated();
        assert_eq!(rotated3.count_in_rows(), 2);
    }
    #[test]
    fn test_count_in_diag() {
        let example = Puzzle::from_lines(_example());
        assert_eq!(example.count_in_diag(), 1);
        let rotated1 = example.rotated();
        assert_eq!(rotated1.count_in_diag(), 1);
        let rotated2 = rotated1.rotated();
        assert_eq!(rotated2.count_in_diag(), 4);
        let rotated3 = rotated2.rotated();
        assert_eq!(rotated3.count_in_diag(), 4);
    }

    #[test]
    fn test_count_all() {
        let example = Puzzle::from_lines(_example());
        assert_eq!(example.count_all(), 18);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ]
    }
}
