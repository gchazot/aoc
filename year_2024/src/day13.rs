pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day13.txt");
    let mut machines = ClawMachine::many_from_lines(data);

    let part1 = min_tokens(&machines);

    machines.iter_mut().for_each(|m| m.fix_prize());
    let part2 = min_tokens(&machines);

    format!("{} {}", part1, part2)
}

type Dimension = i64;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord(Dimension, Dimension);

impl Coord {
    fn from_text(s: &str) -> Coord {
        let parts = s.split_once(", ").unwrap();

        assert!(parts.0.starts_with("X"));
        assert!(parts.1.starts_with("Y"));

        let x = parts.0[2..].parse::<Dimension>().unwrap();
        let y = parts.1[2..].parse::<Dimension>().unwrap();

        Coord(x, y)
    }
}

impl PartialEq<(Dimension, Dimension)> for Coord {
    fn eq(&self, other: &(Dimension, Dimension)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

struct ClawMachine {
    a: Coord,
    b: Coord,
    p: Coord,
}

impl ClawMachine {
    fn from_lines(block: &[String]) -> Self {
        assert_eq!(block.len(), 3);
        let mut pairs = ["Button A", "Button B", "Prize"]
            .into_iter()
            .zip(block)
            .map(|(label, line)| {
                let full_label = format!("{}: ", label);
                assert!(line.starts_with(&full_label));
                Coord::from_text(&line[full_label.len()..])
            })
            .collect::<Vec<_>>();
        let p = pairs.pop().unwrap();
        let b = pairs.pop().unwrap();
        let a = pairs.pop().unwrap();

        assert_ne!(a.0 * b.1, a.1 * b.0);

        Self { a, b, p }
    }

    fn many_from_lines(lines: Vec<String>) -> Vec<ClawMachine> {
        lines
            .split(|l| l.trim().is_empty())
            .map(Self::from_lines)
            .collect()
    }

    fn find_presses(&self) -> Option<(Dimension, Dimension)> {
        let a = self.a;
        let b = self.b;
        let p = self.p;

        let a_den = b.0 * p.1 - b.1 * p.0;
        let a_num = b.0 * a.1 - b.1 * a.0;
        if a_den % a_num == 0 {
            let a_presses = a_den / a_num;
            let b_den = p.0 - a.0 * a_presses;
            if b_den % b.0 == 0 {
                return Some((a_presses, b_den / b.0));
            }
        }
        None
    }

    fn fix_prize(&mut self) {
        const OFFSET: Dimension = 10000000000000;
        self.p.0 += OFFSET;
        self.p.1 += OFFSET;
    }
}

fn min_tokens(machines: &Vec<ClawMachine>) -> Dimension {
    machines
        .iter()
        .filter_map(|c| c.find_presses().and_then(|(a, b)| Some(3 * a + b)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "26599 106228669504887");
    }

    #[test]
    fn test_min_tokens() {
        let mut machines = ClawMachine::many_from_lines(_example());
        assert_eq!(min_tokens(&machines), 480);

        machines.iter_mut().for_each(|m| m.fix_prize());
        assert_eq!(min_tokens(&machines), 875318608908);
    }

    #[test]
    fn test_find_presses() {
        let machines = ClawMachine::many_from_lines(_example());
        assert_eq!(machines[0].find_presses(), Some((80, 40)));
        assert_eq!(machines[1].find_presses(), None);
        assert_eq!(machines[2].find_presses(), Some((38, 86)));
        assert_eq!(machines[3].find_presses(), None);
    }
    #[test]
    fn test_from_lines() {
        let block = &[
            "Button A: X+94, Y+34".to_string(),
            "Button B: X+22, Y+67".to_string(),
            "Prize: X=8400, Y=5400".to_string(),
        ];
        let machine = ClawMachine::from_lines(block);
        assert_eq!(machine.a, (94, 34));
        assert_eq!(machine.b, (22, 67));
        assert_eq!(machine.p, (8400, 5400));
    }

    #[test]
    fn test_many_from_lines() {
        let machines = ClawMachine::many_from_lines(_example());
        assert_eq!(machines.len(), 4);
        assert_eq!(machines[0].p, (8400, 5400));
        assert_eq!(machines[1].a, (26, 66));
        assert_eq!(machines[3].b, (27, 71));
    }

    fn _example() -> Vec<String> {
        aoc_utils::read_lines("input/day13-example.txt")
    }
}
