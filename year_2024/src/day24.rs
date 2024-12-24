use std::collections::HashMap;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day24.txt");

    let mut program = Program::from_lines(data);
    program.execute();
    let part1 = program.collect_result("z");
    let part2 = 456;

    format!("{} {}", part1, part2)
}

enum Operation {
    AND,
    OR,
    XOR,
}

struct Program {
    bits: HashMap<String, Option<bool>>,
    instructions: Vec<(String, Operation, String, String)>,
}

impl Program {
    fn from_lines(lines: Vec<String>) -> Program {
        let mut bits = HashMap::new();
        let mut instructions = Vec::new();

        let mut parsing_bits = true;
        for line in lines {
            if line.is_empty() {
                parsing_bits = false
            } else if parsing_bits {
                let (bit_name, status) = line.split_once(": ").unwrap();
                let previous = bits.insert(bit_name.to_string(), Some(status == "1"));
                assert!(previous.is_none());
            } else {
                let (statement, output_bit) = line.split_once(" -> ").unwrap();
                bits.entry(output_bit.to_string()).or_insert(None);
                let mut statement_parts = statement.splitn(3, " ");
                let bit_a = statement_parts.next().unwrap();
                let operation_str = statement_parts.next().unwrap();
                let bit_b = statement_parts.next().unwrap();
                let operation = match operation_str {
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    "XOR" => Operation::XOR,
                    _ => panic!("Unknown operation: {}", operation_str),
                };
                instructions.push((
                    bit_a.to_string(),
                    operation,
                    bit_b.to_string(),
                    output_bit.to_string(),
                ));
            }
        }
        Program { bits, instructions }
    }

    fn execute(&mut self) {
        while self.bits.values().filter(|bit| bit.is_none()).count() > 0 {
            for (bit_a, operation, bit_b, bit_o) in self.instructions.iter() {
                let a = self.bits.get(bit_a).unwrap();
                let b = self.bits.get(bit_b).unwrap();
                let output = self.bits.get(bit_o).unwrap();
                if output.is_none() && a.is_some() && b.is_some() {
                    let result = match operation {
                        Operation::AND => a.unwrap() && b.unwrap(),
                        Operation::OR => a.unwrap() || b.unwrap(),
                        Operation::XOR => a.unwrap() != b.unwrap(),
                    };
                    self.bits
                        .entry(bit_o.to_string())
                        .and_modify(|v| *v = Some(result));
                }
            }
        }
    }

    fn collect_result(&self, base_name: &str) -> i64 {
        let mut names = self
            .bits
            .keys()
            .filter(|name| name.starts_with(base_name))
            .collect::<Vec<&String>>();
        names.sort();

        let mut result = 0;
        for &bit in names.iter().rev() {
            let value = self.bits.get(bit).unwrap().unwrap();
            result <<= 1;
            if value {
                result |= 1;
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
        assert_eq!(execute(), "51837135476040 456");
    }
    #[test]
    fn test_from_lines() {
        let program = Program::from_lines(example());

        assert_eq!(program.bits.len(), 46);
        assert_eq!(
            program.bits.values().filter(|&bit| bit.is_some()).count(),
            10
        );
    }

    #[test]
    fn test_execute() {
        let mut program = Program::from_lines(example());
        program.execute();
        assert_eq!(
            program.bits.values().filter(|&bit| bit.is_none()).count(),
            0
        );

        assert_eq!(program.collect_result("z"), 2024)
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day24-example.txt")
    }
}
