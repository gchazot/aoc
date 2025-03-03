pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day17.txt");

    let mut program = Program::from_lines(data.clone());
    program.execute();
    let part1 = program.computer.format_outputs();

    let program = Program::from_lines(data);
    let part2 = find_program_reproduction(&program);

    format!("{} {}", part1, part2)
}

#[derive(Clone)]
struct Program {
    computer: Computer,
    instructions: Vec<u8>,
}

type Register = u64;

#[derive(Clone)]
struct Computer {
    reg_a: Register,
    reg_b: Register,
    reg_c: Register,
    outputs: Vec<Register>,
}

impl Program {
    fn from_lines(lines: Vec<String>) -> Program {
        assert_eq!(lines.len(), 5);

        let reg_a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
        let reg_b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
        let reg_c = lines[2].split_once(": ").unwrap().1.parse().unwrap();

        let computer = Computer {
            reg_a,
            reg_b,
            reg_c,
            outputs: Vec::new(),
        };

        let instructions = lines[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Program {
            computer,
            instructions,
        }
    }

    fn execute(&mut self) {
        let mut instruction_pointer = 0;

        while instruction_pointer < self.instructions.len() - 1 {
            let instruction = self.instructions[instruction_pointer];
            let operand = self.instructions[instruction_pointer + 1];

            if let Some(jump) = self.computer.execute(instruction, operand) {
                instruction_pointer = jump;
            } else {
                instruction_pointer += 2;
            }
        }
    }
}

impl Computer {
    fn execute(&mut self, instruction: u8, operand: u8) -> Option<usize> {
        match instruction {
            0 => {
                // adv
                let numerator = self.reg_a;
                let denominator =
                    (2 as Register).pow(self.combo_value(operand).try_into().unwrap());
                self.reg_a = numerator / denominator
            }
            1 => {
                // bxl
                self.reg_b ^= operand as Register;
            }
            2 => {
                // bst
                self.reg_b = self.combo_value(operand) % 8;
            }
            3 => {
                // jnz
                if self.reg_a != 0 {
                    return Some(operand as usize);
                }
            }
            4 => {
                // bxc
                self.reg_b ^= self.reg_c;
            }
            5 => {
                // out
                self.outputs.push(self.combo_value(operand) % 8);
            }
            6 => {
                // bdv
                let numerator = self.reg_a;
                let denominator =
                    (2 as Register).pow(self.combo_value(operand).try_into().unwrap());
                self.reg_b = numerator / denominator
            }
            7 => {
                // cdv
                let numerator = self.reg_a;
                let denominator =
                    (2 as Register).pow(self.combo_value(operand).try_into().unwrap());
                self.reg_c = numerator / denominator
            }
            _ => unreachable!("Invalid instruction: {}", instruction),
        }
        None
    }
    fn combo_value(&self, operand: u8) -> Register {
        if operand <= 3 {
            operand as Register
        } else if operand == 4 {
            self.reg_a
        } else if operand == 5 {
            self.reg_b
        } else if operand == 6 {
            self.reg_c
        } else {
            panic!("Invalid operand: {}", operand);
        }
    }

    fn format_outputs(&self) -> String {
        self.outputs
            .iter()
            .map(|output| format!("{}", output))
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn find_program_reproduction(original: &Program) -> Register {
    let mut attempt_program = original.clone();
    let expected_output = original.instructions.as_slice();

    let mut options = vec![0];

    for current_entry in 0..expected_output.len() {
        let mut next_options = vec![];
        for option in options.iter() {
            for i in 0..0b1000 {
                let a = (option << 3) | i;

                attempt_program.computer.reg_a = a;
                attempt_program.computer.reg_b = original.computer.reg_b;
                attempt_program.computer.reg_c = original.computer.reg_c;
                attempt_program.computer.outputs.clear();

                attempt_program.execute();
                let outputs = &attempt_program.computer.outputs;

                if outputs.len() <= current_entry
                    || outputs[0]
                        != expected_output[expected_output.len() - 1 - current_entry] as Register
                {
                    continue;
                }
                next_options.push(a);
            }
        }
        options = next_options;
    }
    options[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "4,3,7,1,5,3,0,5,4 190384615275535");
    }

    #[test]
    fn test_from_lines() {
        let program = Program::from_lines(example1());
        assert_eq!(program.computer.reg_a, 729);
        assert_eq!(program.computer.reg_b, 0);
        assert_eq!(program.computer.reg_c, 0);
        assert_eq!(program.instructions, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_execute() {
        let mut program = Program::from_lines(example1());
        program.execute();
        let result = program.computer.format_outputs();

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_find_self_reproduction() {
        let program = Program::from_lines(example2());
        let result = find_program_reproduction(&program);

        assert_eq!(result, 117440);
    }

    fn example1() -> Vec<String> {
        aoc_utils::read_lines("input/day17-example1.txt")
    }
    fn example2() -> Vec<String> {
        aoc_utils::read_lines("input/day17-example2.txt")
    }
}
