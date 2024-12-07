pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day7.txt");
    let calculations = Calculation::from_lines(data);
    let part1 = part1(calculations);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

fn part1(calculations: Vec<Calculation>) -> i64 {
    calculations
        .iter()
        .filter_map(|c| c.find_valid_operation().and_then(|_| Some(c.result)))
        .sum()
}

struct Calculation {
    result: i64,
    operand: Vec<i64>,
}

impl Calculation {
    fn from_lines(lines: Vec<String>) -> Vec<Calculation> {
        lines
            .into_iter()
            .map(|l| Calculation::from_line(l))
            .collect()
    }

    fn from_line(line: String) -> Calculation {
        let (result_str, operands_str) = line.split_once(":").unwrap();
        let result = result_str.parse::<i64>().unwrap();
        let operand = operands_str
            .trim()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        Calculation { result, operand }
    }

    fn find_valid_operation(&self) -> Option<usize> {
        let max = 2usize.pow(self.operand.len() as u32);

        for i in 0..max {
            if self.operate(i) == self.result {
                return Some(i);
            }
        }
        None
    }

    fn operate(&self, operation: usize) -> i64 {
        let mut result = self.operand[0];
        for i in 1..self.operand.len() {
            let operation_bit = operation & 1 << (self.operand.len() - i - 1);
            if operation_bit == 0 {
                result += self.operand[i];
            } else {
                result *= self.operand[i];
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
        assert_eq!(execute(), "8401132154762 456");
    }

    #[test]
    fn test_from_line() {
        let calc = Calculation::from_line("3267: 81 40 27".to_string());
        assert_eq!(calc.result, 3267);
        assert_eq!(calc.operand[0], 81);
        assert_eq!(calc.operand[1], 40);
        assert_eq!(calc.operand[2], 27);
    }

    #[test]
    fn test_operate() {
        let calc = Calculation::from_line("0: 10 19".to_string());
        assert_eq!(calc.operate(0b00), 29);
        assert_eq!(calc.operate(0b01), 190);
        assert_eq!(calc.operate(0b10), 29);
        assert_eq!(calc.operate(0b11), 190);

        let calc = Calculation::from_line("0: 1 5 11".to_string());
        assert_eq!(calc.operate(0b00), 17);
        assert_eq!(calc.operate(0b01), 66);
        assert_eq!(calc.operate(0b10), 16);
        assert_eq!(calc.operate(0b11), 55);
    }

    #[test]
    fn test_find_operations() {
        let calc = Calculation::from_line("190: 10 19".to_string());
        assert_eq!(calc.find_valid_operation(), Some(0b1));

        let calc = Calculation::from_line("3267: 81 40 27".to_string());
        assert_eq!(calc.find_valid_operation(), Some(0b01));

        let calc = Calculation::from_line("292: 11 6 16 20".to_string());
        assert_eq!(calc.find_valid_operation(), Some(0b010));
    }

    #[test]
    fn test_part1() {
        let example = Calculation::from_lines(_example());
        assert_eq!(part1(example), 3749);
    }
    fn _example() -> Vec<String> {
        vec![
            String::from("190: 10 19"),
            String::from("3267: 81 40 27"),
            String::from("83: 17 5"),
            String::from("156: 15 6"),
            String::from("7290: 6 8 6 15"),
            String::from("161011: 16 10 13"),
            String::from("192: 17 8 14"),
            String::from("21037: 9 7 18 13"),
            String::from("292: 11 6 16 20"),
        ]
    }
}
