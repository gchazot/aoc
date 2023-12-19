use crate::utils;
use std::collections::HashMap;
use std::ops::Index;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let data = utils::read_lines("src/day3/mine.txt");
    let my_map = CharMap::from_text(&data);

    assert_eq!(507214, sum_part_numbers(&my_map));
    assert_eq!(72553319, sum_gear_ratios(&my_map));
}

#[test]
fn test_sum_part_numbers() {
    let example = utils::read_lines("src/day3/example.txt");
    let example_map = CharMap::from_text(&example);

    assert_eq!(4361, sum_part_numbers(&example_map))
}

fn sum_part_numbers(map: &CharMap) -> u32 {
    let part_numbers = map.find_part_numbers();
    return part_numbers.iter().sum();
}

#[test]
fn test_sum_gear_ratios() {
    let example = utils::read_lines("src/day3/example.txt");
    let example_map = CharMap::from_text(&example);

    assert_eq!(467835, sum_gear_ratios(&example_map));
}

fn sum_gear_ratios(map: &CharMap) -> u32 {
    let possible_gears: HashMap<(usize, usize), Vec<u32>> = map.find_possible_gears();
    return possible_gears
        .iter()
        .map(|(_gear, numbers)| numbers)
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().product::<u32>())
        .sum();
}

struct CharMap {
    chars: Vec<String>,
}

impl CharMap {
    fn from_text(text: &Vec<String>) -> CharMap {
        return CharMap {
            chars: text.clone(),
        };
    }

    fn find_part_numbers(&self) -> Vec<u32> {
        let mut result = Vec::new();

        let all_numbers = self.find_numbers();
        for (y, x, l, num) in all_numbers {
            if self.is_part_number((y, x, l)) {
                result.push(num);
            }
        }
        return result;
    }

    pub(crate) fn find_possible_gears(&self) -> HashMap<(usize, usize), Vec<u32>> {
        let mut result = HashMap::new();

        let all_numbers = self.find_numbers();
        for (y, x, l, num) in all_numbers {
            let stars = self.neighbour_stars((y, x, l));
            for star in stars {
                result.entry(star).or_insert(Vec::new()).push(num);
            }
        }

        return result;
    }

    fn find_numbers(&self) -> Vec<(usize, usize, usize, u32)> {
        let mut result = Vec::new();

        for r in 0..self.chars.len() {
            let row = &self.chars[r];

            let mut i = 0;
            let mut j = 0;
            let mut last = 0;
            while (i + j) < row.len() {
                let parsed = row[i..i + j + 1].parse::<u32>();
                if parsed.is_ok() {
                    j += 1;
                    last = parsed.unwrap();
                } else {
                    if j > 0 {
                        result.push((r, i, j, last));
                    }
                    i = i + j + 1;
                    j = 0;
                }
            }
            if j > 0 {
                result.push((r, i, j, last));
            }
        }
        return result;
    }

    fn is_part_number(&self, (y, x, l): (usize, usize, usize)) -> bool {
        let to_check = self.get_surroundings(y, x, l);

        for pos in to_check {
            if self.is_symbol(pos) {
                return true;
            }
        }

        return false;
    }

    fn neighbour_stars(&self, (y, x, l): (usize, usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let to_check = self.get_surroundings(y, x, l);

        for pos in to_check {
            if &self[pos] == "*" {
                result.push(pos);
            }
        }

        return result;
    }

    fn get_surroundings(&self, row: usize, start: usize, length: usize) -> Vec<(usize, usize)> {
        let mut to_check = Vec::new();

        let mut rows_to_check = Vec::new();
        if row > 0 {
            rows_to_check.push(row - 1);
        }
        if row + 1 < self.chars.len() {
            rows_to_check.push(row + 1);
        }

        let mut cols_to_check = Vec::new();
        if start > 0 {
            cols_to_check.push(start - 1);
            to_check.push((start - 1, row));
        } else {
            cols_to_check.push(start);
        }

        if start + length < self.chars[0].len() {
            cols_to_check.push(start + length + 1);
            to_check.push((start + length, row));
        } else {
            cols_to_check.push(start + length);
        }

        for row in rows_to_check.iter() {
            for col in cols_to_check[0]..cols_to_check[1] {
                to_check.push((col.clone(), row.clone()));
            }
        }
        to_check
    }

    fn is_symbol(&self, pos: (usize, usize)) -> bool {
        if self[pos].chars().all(char::is_numeric) || &self[pos] == "." {
            return false;
        }
        return true;
    }
}

impl Index<(usize, usize)> for CharMap {
    type Output = str;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        return &self.chars[y][x..x + 1];
    }
}
