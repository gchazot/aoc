use std::collections::{HashMap, HashSet};
use std::ops::{Add, RemAssign};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day8.txt");
    let city = City::from_lines(data);

    let part1 = city.find_antinodes(false).len();
    let part2 = city.find_antinodes(true).len();

    format!("{} {}", part1, part2)
}

struct City {
    size: isize,
    antennas: HashMap<char, Vec<(isize, isize)>>,
}

impl City {
    fn from_lines(lines: Vec<String>) -> City {
        let size = lines.len() as isize;

        let mut antennas = HashMap::new();
        for (j, line) in lines.iter().enumerate() {
            assert_eq!(line.len() as isize, size);
            for (i, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((i as isize, j as isize));
                }
            }
        }

        City { size, antennas }
    }

    fn find_antinodes(&self, part2: bool) -> HashSet<(isize, isize)> {
        let mut result = HashSet::new();
        for (_freq, positions) in self.antennas.iter() {
            for a in positions {
                for b in positions {
                    if a == b {
                        continue;
                    }
                    let d = (b.0 - a.0, b.1 - a.1);

                    let (start, step) = if part2 {
                        let dtemp = d.clone();
                        let gcd = gcd(dtemp.0.abs(), dtemp.1.abs());
                        (a, (d.0 / gcd, d.1 / gcd))
                    } else {
                        (b, d)
                    };

                    let mut c = start.clone();
                    loop {
                        c.0 += step.0;
                        c.1 += step.1;

                        if c.0 < 0 || c.0 >= self.size || c.1 < 0 || c.1 >= self.size {
                            break;
                        }
                        result.insert(c.clone());
                        if !part2 {
                            break;
                        }
                    }
                }
            }
        }
        result
    }
}

pub fn gcd(mut n: isize, mut m: isize) -> isize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), String::from("259 927"));
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

        assert_eq!(city.find_antinodes(false).len(), 14);
        assert_eq!(city.find_antinodes(true).len(), 34);
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
