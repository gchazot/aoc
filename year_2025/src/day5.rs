pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day5.txt");
    let inventory = Inventory::from_lines(data);

    let part1 = inventory.count_available_fresh();
    let part2 = inventory.count_all_fresh();

    format!("{} {}", part1, part2)
}

struct Inventory {
    fresh_ranges: Vec<(usize, usize)>,
    available: Vec<usize>,
}

impl Inventory {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut fresh_ranges = vec![];
        let mut available = vec![];

        let mut found_separator = false;

        for line in lines {
            if line.is_empty() {
                found_separator = true;
            } else if !found_separator {
                let split = line.split_once('-').unwrap();
                let a = split.0.parse::<usize>().unwrap();
                let b = split.1.parse::<usize>().unwrap();
                assert!(a <= b, "Invalid range: {}-{}", a, b);
                fresh_ranges.push((a, b));
            } else {
                available.push(line.parse::<usize>().unwrap());
            }
        }
        Self {
            fresh_ranges,
            available,
        }
    }

    fn count_available_fresh(&self) -> usize {
        let mut count = 0;
        for &available in self.available.iter() {
            if self
                .fresh_ranges
                .iter()
                .any(|range| range.0 <= available && available <= range.1)
            {
                count += 1;
            }
        }
        count
    }

    fn count_all_fresh(&self) -> usize {
        let simplified = self.simplify();
        simplified
            .fresh_ranges
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<usize>()
    }

    fn simplify(&self) -> Self {
        let mut to_process = self.fresh_ranges.clone();
        to_process.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut fresh_ranges = vec![];

        let mut i = 0;
        let mut current = (0usize, 0usize);
        while i < to_process.len() {
            if i == 0 {
                current = to_process[i];
            } else {
                if to_process[i].0 <= current.1 + 1 {
                    if to_process[i].1 > current.1 {
                        current = (current.0, to_process[i].1);
                    }
                } else {
                    fresh_ranges.push(current);
                    current = to_process[i];
                }
            }
            i += 1;
        }
        fresh_ranges.push(current);

        Self {
            fresh_ranges,
            available: self.available.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "698 352807801032167");
    }

    #[test]
    fn test_count_all_fresh() {
        let inventory = Inventory::from_lines(example());
        assert_eq!(inventory.count_all_fresh(), 14);
    }

    #[test]
    fn test_simplify() {
        let inventory = Inventory::from_lines(example());
        let simplified = inventory.simplify();

        assert_eq!(simplified.fresh_ranges, vec![(3, 5), (10, 20)]);
        assert_eq!(simplified.count_available_fresh(), 3);
    }

    #[test]
    fn test_count_available_fresh() {
        let inventory = Inventory::from_lines(example());
        assert_eq!(inventory.count_available_fresh(), 3);
    }

    #[test]
    fn test_from_lines() {
        let inventory = Inventory::from_lines(example());
        assert_eq!(
            inventory.fresh_ranges,
            vec![(3, 5), (10, 14), (16, 20), (12, 18)]
        );
        assert_eq!(inventory.available, vec![1, 5, 8, 11, 17, 32]);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("3-5"),
            String::from("10-14"),
            String::from("16-20"),
            String::from("12-18"),
            String::from(""),
            String::from("1"),
            String::from("5"),
            String::from("8"),
            String::from("11"),
            String::from("17"),
            String::from("32"),
        ]
    }
}
