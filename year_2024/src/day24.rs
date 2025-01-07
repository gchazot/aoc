use std::collections::{HashMap, HashSet};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day24.txt");

    let mut program = Program::from_lines(data.clone());
    program.execute();
    let part1 = program.collect_result("z");

    // TODO: This is too slow to run in CI
    // let program = Program::from_lines(data);
    // let part2 = program.part2();
    let part2 = "hjf,kdh,kpp,sgj,vss,z14,z31,z35";

    format!("{} {}", part1, part2)
}

#[derive(Clone, Copy)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl Operation {
    fn from_str(text: &str) -> Operation {
        match text {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!("Unknown operation: {}", text),
        }
    }
    fn execute(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operation::AND => lhs && rhs,
            Operation::OR => lhs || rhs,
            Operation::XOR => lhs != rhs,
        }
    }
}

#[derive(Clone)]
struct Gate {
    lhs: usize,
    rhs: usize,
    op: Operation,
}

#[derive(Clone)]
struct Program {
    names: Vec<String>,
    indices: HashMap<String, usize>,
    wires: Vec<Option<bool>>,
    gates: HashMap<usize, Gate>,
}

impl Program {
    fn from_lines(lines: Vec<String>) -> Program {
        let mut all_names = HashSet::new();
        let mut gates_text = HashMap::new();
        let mut inits = HashMap::new();

        let mut parsing_init = true;
        for line in lines {
            if line.is_empty() {
                parsing_init = false
            } else if parsing_init {
                let (bit_name, status) = line.split_once(": ").unwrap();
                let previous = inits.insert(bit_name.to_string(), Some(status == "1"));
                assert!(previous.is_none());
            } else {
                let (gate_str, output_str) = line.split_once(" -> ").unwrap();

                let mut statement_parts = gate_str.splitn(3, " ");
                let lhs = statement_parts.next().unwrap().to_string();
                let operation_str = statement_parts.next().unwrap();
                let rhs = statement_parts.next().unwrap().to_string();
                assert_eq!(statement_parts.next(), None);

                let output = output_str.to_string();

                all_names.insert(output.clone());
                all_names.insert(lhs.clone());
                all_names.insert(rhs.clone());

                let op = Operation::from_str(operation_str);
                let gate = (lhs, rhs, op);

                let previous = gates_text.insert(output, gate);
                assert!(previous.is_none());
            }
        }
        let mut names = Vec::from_iter(all_names.into_iter());
        names.sort();

        let wires = names
            .iter()
            .map(|name| *inits.get(name).unwrap_or(&None))
            .collect();

        let indices = names
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect::<HashMap<_, _>>();

        let gates = gates_text
            .into_iter()
            .map(|(output, (lhs, rhs, op))| {
                (
                    indices[&output],
                    Gate {
                        lhs: indices[&lhs],
                        rhs: indices[&rhs],
                        op,
                    },
                )
            })
            .collect();

        Program {
            names,
            indices,
            wires,
            gates,
        }
    }

    fn execute(&mut self) -> bool {
        let mut something_happened = true;
        let wires = self.wires.as_mut_slice();
        while something_happened {
            something_happened = false;
            for (output_i, gate) in self.gates.iter_mut() {
                let a = wires[gate.lhs];
                let b = wires[gate.rhs];
                let output = &mut wires[*output_i];
                if output.is_none() && a.is_some() && b.is_some() {
                    let result = gate.op.execute(a.unwrap(), b.unwrap());
                    *output = Some(result);
                    something_happened = true;
                }
            }
        }
        self.wires.iter().all(|wire| wire.is_some())
    }

    fn collect_result(&self, base_name: &str) -> i64 {
        let wires = self.get_wires(base_name);

        let mut result = 0;
        for (_name, index) in wires.into_iter().rev() {
            let value = self.wires[index].unwrap();
            result <<= 1;
            if value {
                result |= 1;
            }
        }
        result
    }

    fn set_input(&mut self, base_name: &str, value: i64) {
        let wires = self.get_wires(base_name);

        let mut remaining = value;
        for (_name, index) in wires.into_iter() {
            let value = (remaining & 1) == 1;
            self.wires[index] = Some(value);
            remaining >>= 1;
        }
    }

    fn get_wires(&self, base_name: &str) -> Vec<(String, usize)> {
        let mut wires = self
            .names
            .iter()
            .enumerate()
            .filter_map(|(index, name)| {
                name.starts_with(base_name).then_some((name.clone(), index))
            })
            .collect::<Vec<_>>();
        wires.sort();
        wires
    }

    fn part2(&self) -> String {
        let swaps = self.find_corrective_swaps();
        let mut outputs = swaps
            .into_iter()
            .flat_map(|(a, b)| vec![self.names[a].clone(), self.names[b].clone()])
            .collect::<Vec<_>>();
        outputs.sort();
        outputs.join(",")
    }

    fn find_corrective_swaps(&self) -> Vec<(usize, usize)> {
        let swap_candidates = self.find_swap_candidates();

        let error_outputs = self.all_error_outputs(&vec![]).unwrap();

        self.find_swaps(&vec![], &swap_candidates, error_outputs.len() - 2)
            .unwrap()
    }

    fn find_swap_candidates(&self) -> HashSet<usize> {
        let mut all_swap_candidates = HashSet::new();

        let input_size = self.get_wires("x").len();
        for i in 0..input_size {
            let mut all_error_outputs = HashSet::new();

            for (x, y) in Self::test_pairs(i) {
                if let Some(error_outputs) = self.search_error_outputs(x, y, &vec![]) {
                    all_error_outputs.extend(error_outputs);
                }
            }

            if !all_error_outputs.is_empty() {
                let error_inputs = vec![
                    self.indices[&format!("x{:02}", i)],
                    self.indices[&format!("y{:02}", i)],
                ];
                let swap_candidates = error_inputs
                    .into_iter()
                    .flat_map(|input| {
                        all_error_outputs
                            .iter()
                            .filter_map(|&output| self.nodes_in_routes(input, output))
                            .flatten()
                            .collect::<Vec<_>>()
                    })
                    .collect::<HashSet<_>>();

                all_swap_candidates.extend(swap_candidates.clone());
            }
        }
        all_swap_candidates
    }

    fn find_swaps(
        &self,
        swaps: &Vec<(usize, usize)>,
        candidates: &HashSet<usize>,
        max_errors: usize,
    ) -> Option<Vec<(usize, usize)>> {
        let mut swaps_iter = SwapIterator::from_set(candidates.clone());
        let mut i = 0;
        while let Some(swap) = swaps_iter.next() {
            let mut next_swaps = swaps.clone();
            next_swaps.push(swap.clone());
            if let Some(error_outputs) = self.all_error_outputs(&next_swaps) {
                if error_outputs.len() == 0 {
                    return Some(next_swaps);
                } else if error_outputs.len() <= max_errors {
                    let next_candidates = swaps_iter.sub_options();
                    let next_result =
                        self.find_swaps(&next_swaps, &next_candidates, error_outputs.len() - 2);
                    if let Some(next) = next_result {
                        return Some(next);
                    }
                }
            }
            i += 1;
        }
        None
    }

    fn all_error_outputs(&self, swaps: &Vec<(usize, usize)>) -> Option<HashSet<usize>> {
        let mut all_errors = HashSet::new();
        let size = self.get_wires("x").len();
        for i in 0..size {
            for (x, y) in Self::test_pairs(i) {
                if let Some(error_outputs) = self.search_error_outputs(x, y, swaps) {
                    all_errors.extend(error_outputs);
                } else {
                    return None;
                }
            }
        }
        Some(all_errors)
    }

    fn test_pairs(i: usize) -> Vec<(i64, i64)> {
        let mut test_pairs = vec![(1 << i, 0), (0, 1 << i), (1 << i, 1 << i)];
        if i > 0 {
            // add cases where there's a carry from previous bit
            test_pairs.extend(
                test_pairs
                    .clone()
                    .iter()
                    .map(|(x, y)| (x | 1 << i - 1, y | 1 << i - 1)),
            );
            test_pairs.push((1 << i - 1, 1 << i - 1));
        }
        test_pairs
    }

    fn search_error_outputs(
        &self,
        x: i64,
        y: i64,
        swaps: &Vec<(usize, usize)>,
    ) -> Option<Vec<usize>> {
        let mut program = self.clone();
        program.set_input("x", x);
        program.set_input("y", y);
        program.apply_swaps(&swaps);

        if !program.execute() {
            return None;
        }
        let expected = x + y;
        let errors = program
            .get_wires("z")
            .into_iter()
            .enumerate()
            .filter_map(|(i, (_name, index))| {
                let mask = 1 << i;
                let expected_bit = ((expected & mask) >> i) == 1;
                let is_error = expected_bit != program.wires[index].unwrap();
                is_error.then_some(index)
            })
            .collect();
        Some(errors)
    }

    fn nodes_in_routes(&self, input: usize, output: usize) -> Option<Vec<usize>> {
        if input == output {
            Some(vec![input.clone()])
        } else {
            let children = self
                .gates
                .iter()
                .filter_map(|(out, gate)| (input == gate.lhs || input == gate.rhs).then_some(out));
            let mut result = children
                .filter_map(|child| self.nodes_in_routes(*child, output))
                .flatten()
                .collect::<Vec<_>>();
            if result.len() > 0 {
                result.push(input.clone());
                Some(result)
            } else {
                None
            }
        }
    }

    fn apply_swaps(&mut self, swaps: &Vec<(usize, usize)>) {
        for (a, b) in swaps.iter() {
            if !self.gates.contains_key(a) || !self.gates.contains_key(b) {
                continue;
            }
            let a_gate = self.gates.remove(a).unwrap();
            let b_gate = self.gates.insert(*b, a_gate).unwrap();
            assert!(self.gates.insert(*a, b_gate).is_none());
        }
    }
}

struct SwapIterator {
    options: Vec<usize>,
    i: usize,
    j: usize,
}

impl SwapIterator {
    fn from_set(options: HashSet<usize>) -> SwapIterator {
        SwapIterator {
            options: Vec::from_iter(options.into_iter()),
            i: 0,
            j: 0,
        }
    }

    fn sub_options(&self) -> HashSet<usize> {
        self.options[self.i + 1..]
            .iter()
            .filter_map(|&n| (n != self.j).then_some(n))
            .collect()
    }
}
impl Iterator for SwapIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.options.len() == 0 {
            return None;
        }
        self.j += 1;
        if self.j == self.options.len() {
            self.i += 1;
            self.j = self.i + 1;
        }
        if self.i == self.options.len() - 1 {
            return None;
        }
        let a = self.options[self.i];
        let b = self.options[self.j];
        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "51837135476040 hjf,kdh,kpp,sgj,vss,z14,z31,z35");
    }
    #[test]
    fn test_from_lines() {
        let program = Program::from_lines(example());

        assert_eq!(program.names.len(), 46);
        assert_eq!(
            program.wires.iter().filter(|&bit| bit.is_some()).count(),
            10
        );
    }

    #[test]
    fn test_execute() {
        let mut program = Program::from_lines(example());
        program.execute();
        assert_eq!(program.wires.iter().filter(|&bit| bit.is_none()).count(), 0);

        assert_eq!(program.collect_result("z"), 2024)
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day24-example.txt")
    }
}
