use aoc_utils as utils;
use std::collections::HashMap;

#[test]
fn test_mine() {
    crate::day12::execute()
}

pub fn execute() {
    let mine = utils::read_lines("input/day12.txt");
    assert_eq!(
        7204,
        mine.iter()
            .map(|line| SpringRow::from_line(line.clone()))
            .map(|row| row.count_valid_arrangements())
            .sum::<usize>()
    );
    // Too slow for now
    assert_eq!(
        1672318386674,
        mine.iter()
            .map(|line| SpringRow::from_line(line.clone()).unfold(5))
            .map(|row| row.count_valid_arrangements())
            .sum::<usize>()
    );
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    Damaged,
    Operational,
}

fn parse_condition(condition: char) -> Option<Condition> {
    match condition {
        '#' => Some(Condition::Damaged),
        '.' => Some(Condition::Operational),
        '?' => None,
        _ => panic!("invalid condition"),
    }
}

#[derive(Clone, Debug)]
struct SpringRow {
    condition: Vec<Option<Condition>>,
    checksum: Vec<usize>,
}

impl SpringRow {
    fn from_line(line: String) -> SpringRow {
        let (condition_text, checksum_text) = line.split_once(' ').unwrap();
        let condition = condition_text.chars().map(parse_condition).collect();
        let checksum = checksum_text
            .split(',')
            .map(|n| -> usize { n.parse::<usize>().unwrap() })
            .collect();
        SpringRow {
            condition,
            checksum,
        }
    }

    fn is_consistent(&self) -> bool {
        self.count_valid_arrangements() > 0
    }

    fn count_valid_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        self._count_valid_arrangements(0, 0, 0, &mut cache)
    }

    fn _count_valid_arrangements(
        &self,
        _condition_index: usize,
        _checksum_index: usize,
        _current_count: usize,
        _cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        let mut current_count = _current_count;
        let mut condition_index = _condition_index;
        let mut checksum_index = _checksum_index;

        let cache_key = (_condition_index, _checksum_index, _current_count);
        if _cache.contains_key(&cache_key) {
            return _cache[&cache_key];
        }

        while condition_index < self.condition.len() {
            let condition = self.condition[condition_index].as_ref();
            match condition {
                None => {
                    let mut count = 0;

                    if checksum_index < self.checksum.len()
                        && current_count < self.checksum[checksum_index]
                    {
                        count += self._count_valid_arrangements(
                            condition_index + 1,
                            checksum_index,
                            current_count + 1,
                            _cache,
                        );
                    }

                    if current_count == 0 {
                        count += self._count_valid_arrangements(
                            condition_index + 1,
                            checksum_index,
                            0,
                            _cache,
                        );
                    }

                    if checksum_index < self.checksum.len()
                        && current_count == self.checksum[checksum_index]
                    {
                        count += self._count_valid_arrangements(
                            condition_index + 1,
                            checksum_index + 1,
                            0,
                            _cache,
                        );
                    }

                    _cache.insert(cache_key, count);
                    return count;
                }
                Some(Condition::Damaged) => {
                    current_count += 1;
                    if checksum_index >= self.checksum.len()
                        || current_count > self.checksum[checksum_index]
                    {
                        _cache.insert(cache_key, 0);
                        return 0;
                    }
                }
                Some(Condition::Operational) => {
                    if current_count > 0 {
                        if current_count != self.checksum[checksum_index] {
                            _cache.insert(cache_key, 0);
                            return 0;
                        } else {
                            checksum_index += 1
                        }
                    }
                    current_count = 0;
                }
            }

            condition_index += 1;
        }

        if current_count > 0 {
            if current_count != self.checksum[checksum_index] {
                _cache.insert(cache_key, 0);
                return 0;
            } else {
                checksum_index += 1
            }
        }

        if checksum_index == self.checksum.len() {
            _cache.insert(cache_key, 1);
            1
        } else {
            _cache.insert(cache_key, 0);
            0
        }
    }

    fn unfold(&self, folds: usize) -> SpringRow {
        let sep = Option::<Condition>::None;
        let mut new_condition = Vec::<Option<Condition>>::new();
        for _ in 1..folds {
            new_condition.extend(self.condition.clone());
            new_condition.push(sep);
        }
        new_condition.extend(self.condition.clone());
        SpringRow {
            condition: new_condition,
            checksum: self.checksum.repeat(folds),
        }
    }
}

#[test]
fn test_parse_row() {
    let row1 = SpringRow::from_line("???.### 1,1,3".to_string());
    assert_eq!(7, row1.condition.len());
    assert!(matches!(row1.condition[0], None));
    assert!(matches!(row1.condition[1], None));
    assert!(matches!(row1.condition[2], None));
    assert!(matches!(row1.condition[3], Some(Condition::Operational)));
    assert!(matches!(row1.condition[4], Some(Condition::Damaged)));
    assert!(matches!(row1.condition[5], Some(Condition::Damaged)));
    assert!(matches!(row1.condition[6], Some(Condition::Damaged)));
    assert_eq!(3, row1.checksum.len());
    assert!(matches!(row1.checksum[0], 1));
    assert!(matches!(row1.checksum[1], 1));
    assert!(matches!(row1.checksum[2], 3));
}

#[test]
fn test_is_consistent() {
    check_consistent_line("# 1");
    check_consistent_line("#. 1");
    check_consistent_line("#.. 1");
    check_consistent_line(".# 1");
    check_consistent_line("..# 1");
    check_consistent_line(".#. 1");
    check_consistent_line("..#.. 1");

    check_consistent_line("## 2");
    check_consistent_line(".##. 2");

    check_consistent_line("#.# 1,1");
    check_consistent_line("#..# 1,1");
    check_consistent_line(".#..#. 1,1");
    check_consistent_line(".#..##..### 1,2,3");
    check_consistent_line("###.##..# 3,2,1");

    check_consistent_line("? 1");
    check_consistent_line(".? 1");
    check_consistent_line("?. 1");
    check_consistent_line("?# 1");
    check_consistent_line("#? 1");
    check_consistent_line(".?#. 1");
    check_consistent_line(".#?. 1");

    check_consistent_line("#?? 1");
    check_consistent_line(".???????? 1");
    check_consistent_line("????????? 1");
    check_consistent_line("????????? 1,1,1,1,1");
    check_consistent_line("????????? 9");

    check_inconsistent_line("? 2");
    check_inconsistent_line("#?# 2");
    check_inconsistent_line("#?# 2");
    check_inconsistent_line("#?# 2,1");
    check_inconsistent_line("????????? 10");
}

#[test]
fn test_example_all_consistent() {
    let example = utils::read_lines("input/day12-example.txt");
    assert!(example
        .iter()
        .map(|line| SpringRow::from_line(line.to_string()))
        .all(|row| row.is_consistent()));
}

#[test]
fn test_mine_all_consistent() {
    let mine = utils::read_lines("input/day12.txt");
    assert!(mine
        .iter()
        .map(|line| SpringRow::from_line(line.to_string()))
        .all(|row| row.is_consistent()));
}

fn check_consistent_line(line: &str) {
    assert!(SpringRow::from_line(line.to_string()).is_consistent());
}

fn check_inconsistent_line(line: &str) {
    assert!(!SpringRow::from_line(line.to_string()).is_consistent());
}

#[test]
fn test_count_valid_arrangements() {
    assert_eq!(1, test_valid_arangements("# 1"));
    assert_eq!(0, test_valid_arangements("## 1"));
    assert_eq!(1, test_valid_arangements("?. 1"));
    assert_eq!(1, test_valid_arangements("?# 1"));
    assert_eq!(2, test_valid_arangements("?? 1"));
    assert_eq!(3, test_valid_arangements("??? 1"));
    assert_eq!(1, test_valid_arangements("??? 1,1"));
}

fn test_valid_arangements(line: &str) -> usize {
    SpringRow::from_line(line.to_string()).count_valid_arrangements()
}

#[test]
fn test_example_valid_arrangements() {
    let example = utils::read_lines("input/day12-example.txt");
    assert_eq!(
        21,
        example
            .iter()
            .map(|line| SpringRow::from_line(line.clone()))
            .map(|row| row.count_valid_arrangements())
            .sum::<usize>()
    );
}

#[test]
fn test_unfold() {
    let row1 = SpringRow::from_line("# 1".to_string()).unfold(5);
    assert_eq!(9, row1.condition.len());
    assert_eq!(5, row1.checksum.len());

    let row2 = SpringRow::from_line("#?# 1,1".to_string()).unfold(5);
    assert_eq!(19, row2.condition.len());
    assert_eq!(10, row2.checksum.len());
}

#[test]
fn test_example_unfolded_valid_arrangements() {
    let example = utils::read_lines("input/day12-example.txt");
    assert_eq!(
        525152,
        example
            .iter()
            .map(|line| SpringRow::from_line(line.clone()).unfold(5))
            .map(|row| row.count_valid_arrangements())
            .sum::<usize>()
    );
}
