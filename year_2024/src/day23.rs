use std::collections::HashMap;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day23.txt");
    let network = Network::from_lines(data);

    let triplets = network.triplets();

    let part1 = network.part1(&triplets);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

struct Network {
    node_name: Vec<String>,
    edges: Vec<Vec<bool>>,
}

impl Network {
    fn from_lines(lines: Vec<String>) -> Network {
        let mut node_name = vec![];
        let mut node_index = HashMap::new();

        let mut pairs = vec![];

        for line in lines.iter() {
            let (a_str, b_str) = line.split_once("-").unwrap();
            if !node_index.contains_key(a_str) {
                node_name.push(a_str.to_string());
                node_index.insert(a_str.to_string(), node_name.len() - 1);
            }
            if !node_index.contains_key(b_str) {
                node_name.push(b_str.to_string());
                node_index.insert(b_str.to_string(), node_name.len() - 1);
            }
            pairs.push((a_str.to_string(), b_str.to_string()));
        }

        let mut edges = vec![vec![false; node_name.len()]; node_name.len()];

        for (a, b) in pairs {
            edges[node_index[&a]][node_index[&b]] = true;
            edges[node_index[&b]][node_index[&a]] = true;
        }

        Network { node_name, edges }
    }

    fn triplets(&self) -> Vec<(usize, usize, usize)> {
        let mut result = vec![];

        for i_a in 0..self.edges.len() - 2 {
            for i_b in i_a + 1..self.edges.len() - 1 {
                for i_c in i_b + 1..self.edges.len() {
                    if self.edges[i_a][i_b] && self.edges[i_b][i_c] && self.edges[i_c][i_a] {
                        result.push((i_a, i_b, i_c));
                    }
                }
            }
        }

        result
    }

    fn part1(&self, triplets: &[(usize, usize, usize)]) -> usize {
        triplets
            .iter()
            .filter(|(a, b, c)| {
                self.node_name[*a].chars().nth(0).unwrap() == 't'
                    || self.node_name[*b].chars().nth(0).unwrap() == 't'
                    || self.node_name[*c].chars().nth(0).unwrap() == 't'
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1238 456");
    }

    #[test]
    fn test_from_lines() {
        let network = Network::from_lines(example());

        assert_eq!(network.node_name.len(), 16);
        assert_eq!(network.edges.len(), 16);
        assert_eq!(network.edges.iter().flatten().filter(|&e| *e).count(), 64);
    }

    #[test]
    fn test_triplets() {
        let network = Network::from_lines(example());
        let triplets = network.triplets();
        assert_eq!(triplets.len(), 12);

        assert_eq!(network.part1(&triplets), 7);
    }
    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day23-example.txt")
    }
}
