use aoc_utils as utils;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let universe = Universe::from_file("mine.txt", 2);
    assert_eq!(10422930, universe.sum_shortest_distance());
    let universe_expanding = Universe::from_file("mine.txt", 1000000);
    assert_eq!(699909023130, universe_expanding.sum_shortest_distance());
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
struct Position(usize, usize);

struct Universe {
    galaxies: Vec<Position>,
    empty_lines: HashSet<usize>,
    empty_columns: HashSet<usize>,
    expansion: usize,
}

impl Universe {
    fn from_lines(lines: Vec<String>, expansion: usize) -> Universe {
        let num_lines = lines.len();
        let num_cols = if num_lines > 0 { lines[0].len() } else { 0 };

        let mut empty_lines: HashSet<usize> = (1..num_lines + 1).collect();
        let mut empty_columns: HashSet<usize> = (1..num_cols + 1).collect();

        let mut galaxies = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), num_cols);
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    let pos = Position(j + 1, i + 1);
                    galaxies.push(pos);

                    empty_lines.remove(&(i + 1));
                    empty_columns.remove(&(j + 1));
                }
            }
        }
        Universe {
            galaxies,
            empty_lines,
            empty_columns,
            expansion,
        }
    }
    fn from_file(filename: &str, expansion: usize) -> Universe {
        let path = format!("src/day11/{}", filename);
        let lines = utils::read_lines(&path);
        Universe::from_lines(lines, expansion)
    }

    fn distance(&self, a: &Position, b: &Position) -> usize {
        let mut dist = 0;
        let minx = min(a.0, b.0);
        let maxx = max(a.0, b.0);
        for x in minx..maxx {
            dist += if self.empty_columns.contains(&x) {
                self.expansion
            } else {
                1
            };
        }
        let miny = min(a.1, b.1);
        let maxy = max(a.1, b.1);
        for y in miny..maxy {
            dist += if self.empty_lines.contains(&y) {
                self.expansion
            } else {
                1
            };
        }
        dist
    }

    fn galaxy_pairs(&self) -> Vec<(&Position, &Position)> {
        let mut pairs = Vec::new();

        let size = self.galaxies.len();
        if size > 0 {
            for (i, pos1) in self.galaxies[0..size - 1].iter().enumerate() {
                for pos2 in self.galaxies[i + 1..size].iter() {
                    pairs.push((pos1, pos2));
                }
            }
        }
        pairs
    }

    fn sum_shortest_distance(&self) -> usize {
        let mut total: usize = 0;
        for pair in self.galaxy_pairs() {
            total += self.distance(pair.0, pair.1);
        }
        total
    }
}

#[test]
fn test_parse_lines() {
    let no_line: Vec<String> = vec![];
    let no_line_universe: Universe = Universe::from_lines(no_line, 2);
    assert_eq!(0, no_line_universe.galaxies.len());

    let one_line_no_galaxy = vec!["......".to_string()];
    let one_line_no_galaxy_universe: Universe = Universe::from_lines(one_line_no_galaxy, 2);
    assert_eq!(0, one_line_no_galaxy_universe.galaxies.len());

    let no_galaxy = vec!["....".to_string(), "....".to_string(), "....".to_string()];
    let no_galaxy_universe: Universe = Universe::from_lines(no_galaxy, 2);
    assert_eq!(0, no_galaxy_universe.galaxies.len());

    let only_one_galaxy = vec!["#".to_string()];
    let only_one_galaxy_universe: Universe = Universe::from_lines(only_one_galaxy, 2);
    assert_eq!(1, only_one_galaxy_universe.galaxies.len());
    assert!(only_one_galaxy_universe.galaxies.contains(&Position(1, 1)));

    let one_line_one_galaxy = vec!["...#..".to_string()];
    let one_line_one_galaxy_universe: Universe = Universe::from_lines(one_line_one_galaxy, 2);
    assert_eq!(1, one_line_one_galaxy_universe.galaxies.len());
    assert!(one_line_one_galaxy_universe
        .galaxies
        .contains(&Position(4, 1)));

    let one_galaxy = vec!["....".to_string(), "....".to_string(), ".#..".to_string()];
    let one_galaxy_universe: Universe = Universe::from_lines(one_galaxy, 2);
    assert_eq!(1, one_galaxy_universe.galaxies.len());
    assert!(one_galaxy_universe.galaxies.contains(&Position(2, 3)));

    let multiple_galaxies = vec!["...#".to_string(), ".#..".to_string(), "#.#.".to_string()];
    let multiple_galaxies_universe: Universe = Universe::from_lines(multiple_galaxies, 2);
    assert_eq!(4, multiple_galaxies_universe.galaxies.len());
    assert!(multiple_galaxies_universe
        .galaxies
        .contains(&Position(4, 1)));
    assert!(multiple_galaxies_universe
        .galaxies
        .contains(&Position(2, 2)));
    assert!(multiple_galaxies_universe
        .galaxies
        .contains(&Position(1, 3)));
    assert!(multiple_galaxies_universe
        .galaxies
        .contains(&Position(3, 3)));
}

#[test]
fn test_parse_file() {
    let universe = Universe::from_file("test_input.txt", 2);
    assert_eq!(9, universe.galaxies.len());
    assert!(universe.galaxies.contains(&Position(4, 1)));
    assert!(universe.galaxies.contains(&Position(8, 2)));
    assert!(universe.galaxies.contains(&Position(1, 3)));
    assert!(universe.galaxies.contains(&Position(7, 5)));
    assert!(universe.galaxies.contains(&Position(2, 6)));
    assert!(universe.galaxies.contains(&Position(10, 7)));
    assert!(universe.galaxies.contains(&Position(8, 9)));
    assert!(universe.galaxies.contains(&Position(1, 10)));
    assert!(universe.galaxies.contains(&Position(5, 10)));
}

#[test]
fn test_distance() {
    let universe = Universe::from_file("test_input.txt", 2);
    assert_eq!(15, universe.distance(&Position(4, 1), &Position(8, 9)));
    assert_eq!(17, universe.distance(&Position(1, 3), &Position(10, 7)));
    assert_eq!(5, universe.distance(&Position(1, 10), &Position(5, 10)));
    assert_eq!(5, universe.distance(&Position(5, 10), &Position(1, 10)));
}

#[test]
fn test_galaxy_pairs() {
    let universe = Universe::from_file("test_input.txt", 2);
    let pairs_vec = universe.galaxy_pairs();
    let pairs_set: HashSet<(&Position, &Position)> = HashSet::from_iter(pairs_vec.iter().cloned());

    assert_eq!(pairs_vec.len(), pairs_set.len());
    assert_eq!(36, pairs_vec.len());
    assert!(pairs_vec.contains(&(&Position(4, 1), &Position(8, 2))));
    assert!(pairs_vec.contains(&(&Position(4, 1), &Position(1, 3))));
    assert!(pairs_vec.contains(&(&Position(8, 2), &Position(1, 3))));

    assert!(!pairs_vec.contains(&(&Position(8, 2), &Position(4, 1))));
    assert!(!pairs_vec.contains(&(&Position(4, 1), &Position(4, 1))));
    assert!(!pairs_vec.contains(&(&Position(8, 2), &Position(8, 2))));
}

#[test]
fn test_sum_shortest_distance() {
    let universe = Universe::from_file("test_input.txt", 2);
    assert_eq!(374, universe.sum_shortest_distance());
}

#[test]
fn test_sum_shortest_distance_with_expansion() {
    let universe_expand_10 = Universe::from_file("test_input.txt", 10);
    assert_eq!(1030, universe_expand_10.sum_shortest_distance());
    let universe_expand_100 = Universe::from_file("test_input.txt", 100);
    assert_eq!(8410, universe_expand_100.sum_shortest_distance());
}
