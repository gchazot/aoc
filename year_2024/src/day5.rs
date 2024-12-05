pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day5.txt");
    let po = PrintOrder::from_lines(data);
    let part1 = po.part1();
    let part2 = 456;

    format!("{} {}", part1, part2)
}

struct PrintOrder {
    rules: Vec<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

impl PrintOrder {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut rules_lines = vec![];
        let mut updates_lines = vec![];
        let mut foundsep = false;
        for line in lines.into_iter() {
            if line.is_empty() {
                foundsep = true;
            } else if !foundsep {
                rules_lines.push(line);
            } else {
                updates_lines.push(line);
            }
        }

        let rules = rules_lines
            .iter()
            .map(|line| {
                let (a, b) = line.split_once("|").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        let updates = updates_lines
            .iter()
            .map(|line| line.split(",").map(|p| p.parse().unwrap()).collect())
            .collect();

        Self { rules, updates }
    }

    fn is_ordered(&self, update: &Vec<u8>) -> bool {
        let len = update.len();
        (0..len - 1).all(|i| {
            let a = update[i];
            (i + 1..len).all(|j| {
                let b = update[j];
                let has_rule = self.rules.contains(&(b, a));
                !has_rule
            })
        })
    }

    fn middle_page(update: &Vec<u8>) -> u8 {
        let len = update.len();
        assert!(len % 2 == 1);
        update[len / 2]
    }

    fn part1(&self) -> u32 {
        self.updates
            .iter()
            .filter_map(|update| {
                self.is_ordered(update)
                    .then(|| Self::middle_page(update) as u32)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "6267 456");
    }

    #[test]
    fn test_from_lines() {
        let po = PrintOrder::from_lines(_example());
        assert_eq!(po.rules.len(), 21);
        assert_eq!(po.updates.len(), 6);

        assert_eq!(po.rules[0], (47, 53));
        assert_eq!(po.rules[20], (53, 13));

        assert_eq!(po.updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(po.updates[4], vec![61, 13, 29]);
    }

    #[test]
    fn test_is_ordered() {
        let po = PrintOrder::from_lines(_example());

        assert!(po.is_ordered(&po.updates[0]));
        assert!(po.is_ordered(&po.updates[1]));
        assert!(po.is_ordered(&po.updates[2]));
        assert!(!po.is_ordered(&po.updates[3]));
        assert!(!po.is_ordered(&po.updates[4]));
        assert!(!po.is_ordered(&po.updates[5]));
    }

    #[test]
    fn test_get_middle_page() {
        let po = PrintOrder::from_lines(_example());

        assert_eq!(PrintOrder::middle_page(&po.updates[0]), 61);
        assert_eq!(PrintOrder::middle_page(&po.updates[1]), 53);
        assert_eq!(PrintOrder::middle_page(&po.updates[2]), 29);
        assert_eq!(PrintOrder::middle_page(&po.updates[3]), 47);
        assert_eq!(PrintOrder::middle_page(&po.updates[4]), 13);
        assert_eq!(PrintOrder::middle_page(&po.updates[5]), 75);
    }

    #[test]
    fn test_part_1() {
        let po = PrintOrder::from_lines(_example());

        assert_eq!(po.part1(), 143);
    }

    fn _example() -> Vec<String> {
        aoc_utils::read_lines("input/day5-example.txt")
    }
}
