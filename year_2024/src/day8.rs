use std::collections::{HashMap, HashSet};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day8.txt");
    let city = City::from_lines(data);

    let part1 = city.find_antinodes().len();
    let part2 = 456;

    format!("{} {}", part1, part2)
}

struct City {
    size: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl City {
    fn from_lines(lines: Vec<String>) -> City {
        let size = lines.len();

        let mut antennas = HashMap::new();
        for (j, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), size);
            for (i, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_insert_with(Vec::new).push((i, j));
                }
            }
        }

        City { size, antennas }
    }

    fn find_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        for (_freq, positions) in self.antennas.iter() {
            for a in positions {
                for b in positions {
                    if a == b || (b.0 * 2 < a.0) || (b.1 * 2 < a.1) {
                        continue;
                    }
                    let c = (b.0 * 2 - a.0, b.1 * 2 - a.1);
                    if c.0 >= self.size || c.1 >= self.size {
                        continue;
                    }
                    result.insert(c);
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), String::from("259 456"));
    }

    #[test]
    fn test_from_lines() {
        let city = City::from_lines(_example());
        assert_eq!(city.size, 12);
        assert_eq!(city.antennas.len(), 2);
        assert_eq!(city.antennas[&'0'].len(), 4);
        assert_eq!(city.antennas[&'A'].len(), 3);
    }

    #[test]
    fn test_find_antinodes() {
        let city = City::from_lines(_example());
        let antinodes = city.find_antinodes();

        assert_eq!(antinodes.len(), 14);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("............"),
            String::from("........0..."),
            String::from(".....0......"),
            String::from(".......0...."),
            String::from("....0......."),
            String::from("......A....."),
            String::from("............"),
            String::from("............"),
            String::from("........A..."),
            String::from(".........A.."),
            String::from("............"),
            String::from("............"),
        ]
    }
}
