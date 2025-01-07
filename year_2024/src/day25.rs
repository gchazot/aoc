pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day25.txt");
    let (locks, keys) = locks_keys_from_lines(data);

    let part1 = part1(&locks, &keys);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

struct Part {
    is_lock: bool,
    pins: Vec<i8>,
}

impl Part {
    fn from_lines(lines: Vec<String>) -> Part {
        let is_lock = lines.first().unwrap().chars().all(|c| c == '#');
        let mut pins = vec![-1; lines[0].len()];
        for line in lines {
            assert!(line.len() == pins.len());
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => pins[i] += 1,
                    _ => continue,
                }
            }
        }
        Part { is_lock, pins }
    }

    fn matches(&self, other: &Self) -> bool {
        if self.is_lock == other.is_lock {
            false
        } else if self.pins.len() != other.pins.len() {
            false
        } else {
            // let maxi = self.max.max(other.max) + 1;
            self.pins
                .iter()
                .zip(other.pins.iter())
                .all(|(a, b)| a + b <= 5)
        }
    }
}

fn locks_keys_from_lines(lines: Vec<String>) -> (Vec<Part>, Vec<Part>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for block in lines.split(|l| l.is_empty()) {
        let part = Part::from_lines(block.to_vec());
        if part.is_lock {
            locks.push(part);
        } else {
            keys.push(part);
        }
    }

    (locks, keys)
}

fn part1(locks: &Vec<Part>, keys: &Vec<Part>) -> usize {
    let mut count = 0;
    for lock in locks {
        for key in keys {
            if lock.matches(key) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "3155 456");
    }

    #[test]
    fn test_from_lines() {
        let (locks, keys) = locks_keys_from_lines(example());

        assert_eq!(locks.len(), 2);
        assert_eq!(keys.len(), 3);

        assert_eq!(locks[0].pins, vec![0, 5, 3, 4, 3]);
        assert_eq!(locks[1].pins, vec![1, 2, 0, 5, 3]);

        assert_eq!(keys[0].pins, vec![5, 0, 2, 1, 3]);
        assert_eq!(keys[1].pins, vec![4, 3, 4, 0, 2]);
        assert_eq!(keys[2].pins, vec![3, 0, 2, 0, 1]);
    }

    #[test]
    fn test_matches() {
        let (locks, keys) = locks_keys_from_lines(example());

        assert!(!locks[0].matches(&keys[0]));
        assert!(!locks[0].matches(&keys[1]));
        assert!(locks[0].matches(&keys[2]));
        assert!(!locks[1].matches(&keys[0]));
        assert!(locks[1].matches(&keys[1]));
        assert!(locks[1].matches(&keys[2]));
    }

    #[test]
    fn test_part1() {
        let (locks, keys) = locks_keys_from_lines(example());
        assert_eq!(part1(&locks, &keys), 3);
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day25-example.txt")
    }
}
