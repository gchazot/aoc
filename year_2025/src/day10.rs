use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use std::collections::{HashSet, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day10.txt");

    let machines = Machine::from_lines(data);

    let part1 = machines
        .iter()
        .map(|m| m.find_least_presses_to_leds().unwrap())
        .sum::<usize>();

    let part2 = machines
        .iter()
        .map(|m| m.find_least_presses_to_joltages().unwrap())
        .sum::<usize>();

    format!("{} {}", part1, part2)
}

type LEDs = u16;
type Joltage = u16;

struct Machine {
    target: LEDs,
    buttons: Vec<LEDs>,
    joltages: Vec<Joltage>,
}

impl Machine {
    fn from_lines(lines: Vec<String>) -> Vec<Self> {
        lines.into_iter().map(Self::from_line).collect()
    }

    fn from_line(line: String) -> Self {
        let elements: Vec<&str> = line.split(" ").collect();

        let target_str = elements[0][1..elements[0].len() - 1].to_string();
        let target = target_str
            .chars()
            .enumerate()
            .map(|(i, c)| if c == '.' { 0 } else { 1 << i })
            .sum::<LEDs>();

        let buttons = elements[1..elements.len() - 1]
            .iter()
            .map(|button_str| {
                button_str[1..button_str.len() - 1]
                    .split(',')
                    .map(|wire| wire.parse::<u8>().unwrap())
                    .map(|wire| 1 << wire)
                    .sum()
            })
            .collect();

        let joltages_str = *elements.last().unwrap();
        let joltages = joltages_str[1..joltages_str.len() - 1]
            .split(',')
            .map(|s| s.parse::<Joltage>().unwrap())
            .collect();

        Self {
            target,
            buttons,
            joltages,
        }
    }

    fn find_least_presses_to_leds(&self) -> Option<usize> {
        let mut to_explore = VecDeque::new();
        to_explore.push_back((0usize, 0 as LEDs));

        let mut seen = HashSet::new();

        while let Some((cost, current)) = to_explore.pop_front() {
            if current == self.target {
                return Some(cost);
            }

            if seen.contains(&current) {
                continue;
            }
            seen.insert(current);

            for button in self.buttons.iter() {
                let next = current ^ *button;
                let next_cost = cost + 1;
                to_explore.push_back((next_cost, next));
            }
        }
        None
    }
    fn find_least_presses_to_joltages(&self) -> Option<usize> {
        let mut vars = variables!();

        let button_vars = (0..self.buttons.len())
            .into_iter()
            .map(|i| vars.add(variable().integer().min(0).name(format!("Button{i}"))))
            .collect::<Vec<_>>();

        let joltage_constraints = self
            .joltages
            .iter()
            .enumerate()
            .map(|(i, joltage)| {
                let mask = 1 << i;
                let expression = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(j, b)| (mask & *b != 0).then_some(button_vars[j]))
                    .sum::<Expression>();
                (expression, joltage)
            })
            .map(|(joltage_expression, joltage)| constraint!(joltage_expression == *joltage))
            .collect::<Vec<_>>();

        let objective = button_vars.iter().sum::<Expression>();

        let solution = vars
            .minimise(objective)
            .using(default_solver)
            .with_all(joltage_constraints)
            .solve()
            .unwrap();

        Some(button_vars.iter().map(|b| solution.value(*b)).sum::<f64>() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "428 16613");
    }

    #[test]
    fn test_find_least_presses_to_leds() {
        let machines = Machine::from_lines(example());
        assert_eq!(machines[0].find_least_presses_to_leds(), Some(2));
        assert_eq!(machines[1].find_least_presses_to_leds(), Some(3));
        assert_eq!(machines[2].find_least_presses_to_leds(), Some(2));
    }

    #[test]
    fn test_find_least_presses_to_joltages() {
        let machines = Machine::from_lines(example());
        assert_eq!(machines[0].find_least_presses_to_joltages(), Some(10));
        assert_eq!(machines[1].find_least_presses_to_joltages(), Some(12));
        assert_eq!(machines[2].find_least_presses_to_joltages(), Some(11));
    }

    #[test]
    fn test_from_lines() {
        let machines = Machine::from_lines(example());

        assert_eq!(machines.len(), 3);

        assert_eq!(machines[0].target, 0b0110);
        assert_eq!(machines[1].target, 0b01000);
        assert_eq!(machines[2].target, 0b101110);

        assert_eq!(
            machines[0].buttons,
            vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]
        );
        assert_eq!(
            machines[1].buttons,
            vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110]
        );
        assert_eq!(
            machines[2].buttons,
            vec![0b011111, 0b011001, 0b110111, 0b000110]
        );
    }

    fn example() -> Vec<String> {
        vec![
            String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            String::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            String::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
        ]
    }
}
