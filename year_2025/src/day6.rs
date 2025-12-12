pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day6.txt");

    let homework = Homework::from_lines(&data);
    let part1 = homework.row_math();
    let cephalopod_homework = Homework::from_columns(&data);
    let part2 = cephalopod_homework.column_math();

    format!("{} {}", part1, part2)
}

struct Homework {
    operands: Vec<Vec<u64>>,
    operators: Vec<char>,
}

impl Homework {
    fn from_lines(lines: &Vec<String>) -> Self {
        Self {
            operands: lines[0..lines.len() - 1]
                .iter()
                .map(|line| {
                    line.trim()
                        .split_whitespace()
                        .map(|operand| operand.parse::<u64>().unwrap())
                        .collect()
                })
                .collect(),
            operators: lines
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|operator| operator.chars().nth(0).unwrap())
                .collect(),
        }
    }

    fn row_math(&self) -> u64 {
        self.operators
            .iter()
            .enumerate()
            .map(|(i, operator)| -> u64 {
                match operator {
                    '*' => self.operands.iter().map(|operands| operands[i]).product(),
                    '+' => self.operands.iter().map(|operands| operands[i]).sum(),
                    _ => panic!("Invalid operator {operator}"),
                }
            })
            .sum()
    }

    fn from_columns(lines: &Vec<String>) -> Self {
        let width = lines.iter().map(|line| line.len()).max().unwrap();
        let starts = lines
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c != ' ')
            .map(|(i, _c)| i)
            .collect::<Vec<usize>>();

        let rows: Vec<_> = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        let mut operands = vec![vec![]; starts.len()];
        for operation_index in 0..starts.len() {
            let start = starts[operation_index];
            let end = if operation_index < starts.len() - 1 {
                starts[operation_index + 1] - 1
            } else {
                width
            };

            for operand_index in 0..end - start {
                let mut operand = 0;
                for row in 0..lines.len() - 1 {
                    let c = rows[row][start + operand_index];
                    if c != ' ' {
                        operand = operand * 10 + (c as u8 - b'0') as u64;
                    }
                }
                assert_ne!(
                    operand, 0,
                    "Invalid operand at index {operand_index} in operation {operation_index} ({end}-{start})"
                );
                operands[operation_index].push(operand);
            }
        }

        Self {
            operands,
            operators: lines
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|operator| operator.chars().nth(0).unwrap())
                .collect(),
        }
    }

    fn column_math(&self) -> u64 {
        self.operators
            .iter()
            .zip(self.operands.iter())
            .map(|(operator, operands)| -> u64 {
                match operator {
                    '*' => operands.iter().product(),
                    '+' => operands.iter().sum(),
                    _ => panic!("Invalid operator {operator}"),
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "5595593539811 10153315705125");
    }

    #[test]
    fn test_from_lines() {
        let homework = Homework::from_lines(&example());

        assert_eq!(homework.operands.len(), 3);
        for operands in homework.operands.iter() {
            assert_eq!(operands.len(), 4);
        }
        assert_eq!(homework.operands[0], vec![123, 328, 51, 64]);
        assert_eq!(homework.operands[1], vec![45, 64, 387, 23]);
        assert_eq!(homework.operands[2], vec![6, 98, 215, 314]);

        assert_eq!(homework.operators.len(), 4);
        assert_eq!(homework.operators, vec!['*', '+', '*', '+']);
    }

    #[test]
    fn test_row_math() {
        let homework = Homework::from_lines(&example());

        assert_eq!(homework.row_math(), 4277556);
    }

    #[test]
    fn test_from_columns() {
        let homework = Homework::from_columns(&example());

        assert_eq!(homework.operands.len(), 4);

        assert_eq!(homework.operands[0], vec![1, 24, 356]);
        assert_eq!(homework.operands[1], vec![369, 248, 8]);
        assert_eq!(homework.operands[2], vec![32, 581, 175]);
        assert_eq!(homework.operands[3], vec![623, 431, 4]);

        assert_eq!(homework.operators.len(), 4);
        assert_eq!(homework.operators, vec!['*', '+', '*', '+']);
    }

    #[test]
    fn test_column_math() {
        let homework = Homework::from_columns(&example());

        assert_eq!(homework.column_math(), 3263827);
    }
    fn example() -> Vec<String> {
        vec![
            String::from("123 328  51 64 "),
            String::from(" 45 64  387 23 "),
            String::from("  6 98  215 314"),
            String::from("*   +   *   + "),
        ]
    }
}
