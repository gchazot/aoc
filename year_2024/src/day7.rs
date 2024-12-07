pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day7.txt");
    let calculations = Calculation::from_lines(data);
    let part1 = part1(&calculations);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

fn part1(calculations: &Vec<Calculation>) -> i64 {
    calculations
        .iter()
        .filter_map(|c| {
            c.find_valid_operation::<OperatorPart1>()
                .and_then(|_| Some(c.result))
        })
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

    fn find_valid_operation<Op: Operator>(&self) -> Option<Operation<Op>> {
        let mut operation = Operation::new(self.operand.len());

        loop {
            if self.operate(&operation) == self.result {
                return Some(operation);
            }
            if !operation.next() {
                break;
            }
        }
        None
    }

    fn operate<Op: Operator>(&self, operation: &Operation<Op>) -> i64 {
        let mut result = self.operand[0];
        for i in 1..self.operand.len() {
            let operator = operation.operators[i - 1];
            result = operator.execute(result, self.operand[i]);
        }
        result
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Operators {
    Add,
    Mul,
}

trait Operator: Sized + Clone + Copy + Eq + PartialEq {
    fn new() -> Self;
    fn next(&self) -> Option<Self>;
    fn execute(&self, lhs: i64, rhs: i64) -> i64;
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct OperatorPart1 {
    current: Operators,
}

impl Operator for OperatorPart1 {
    fn new() -> Self {
        Self {
            current: Operators::Add,
        }
    }

    fn next(&self) -> Option<Self> {
        match self.current {
            Operators::Add => Some(Self {
                current: Operators::Mul,
            }),
            Operators::Mul => None,
            _ => unreachable!(),
        }
    }

    fn execute(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Operation<Op: Operator> {
    operators: Vec<Op>,
}

impl<Op: Operator> Operation<Op> {
    fn new(num_operands: usize) -> Self {
        Operation {
            operators: vec![Op::new(); num_operands - 1],
        }
    }

    fn next(&mut self) -> bool {
        for i in (0..self.operators.len()).rev() {
            let op = self.operators[i].next();
            if op.is_some() {
                self.operators[i] = op.unwrap();
                return true;
            } else {
                self.operators[i] = Operator::new();
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Operators::*;

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

    #[macro_export]
    macro_rules! ope {
        ( $( $op:ident ),* ) => {
            Operation::<OperatorPart1> { operators: vec![$( OperatorPart1{current: $op} ),*] }
        };
    }

    #[test]
    fn test_operate() {
        let calc = Calculation::from_line("0: 10 19".to_string());
        assert_eq!(calc.operate(&ope![Add]), 29);
        assert_eq!(calc.operate(&ope![Mul]), 190);

        let calc = Calculation::from_line("0: 1 5 11".to_string());
        assert_eq!(calc.operate(&ope![Add, Add]), 17);
        assert_eq!(calc.operate(&ope![Add, Mul]), 66);
        assert_eq!(calc.operate(&ope![Mul, Add]), 16);
        assert_eq!(calc.operate(&ope![Mul, Mul]), 55);
    }

    #[test]
    fn test_find_operations() {
        let calc = Calculation::from_line("190: 10 19".to_string());
        assert_eq!(calc.find_valid_operation(), Some(ope![Mul]));

        let calc = Calculation::from_line("3267: 81 40 27".to_string());
        assert_eq!(calc.find_valid_operation(), Some(ope![Add, Mul]));

        let calc = Calculation::from_line("292: 11 6 16 20".to_string());
        assert_eq!(calc.find_valid_operation(), Some(ope![Add, Mul, Add]));
    }

    #[test]
    fn test_part1() {
        let example = Calculation::from_lines(_example());
        assert_eq!(part1(&example), 3749);
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
