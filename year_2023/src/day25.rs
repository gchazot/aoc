use std::collections::{HashMap, HashSet, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day25.txt");
    let network = Graph::from_lines(data);

    let mut residual = network.clone();
    residual.max_flow(50, 1002);

    let g1 = residual.reachable(50).len();
    let g2 = residual.v_name.len() - g1;

    let part1 = g1 * g2;
    let part2 = 456;

    format!("{} {}", part1, part2)
}

#[derive(Debug, Clone)]
struct Graph {
    v_index: HashMap<String, usize>,
    v_name: Vec<String>,
    edge: Vec<HashMap<usize, i32>>,
}

impl Graph {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut pairs = vec![];
        let mut components = HashSet::new();

        for line in lines {
            let (source, targets_str) = line.split_once(": ").unwrap();
            components.insert(source.to_string());
            let targets = targets_str.split(" ");
            for target in targets {
                pairs.push((source.to_string(), target.to_string()));
                components.insert(target.to_string());
            }
        }

        let v_names = Vec::from_iter(components.into_iter());
        let vertices = HashMap::from_iter(v_names.iter().enumerate().map(|(i, c)| (c.clone(), i)));

        let mut edges = vec![HashMap::new(); vertices.len()];
        for (a, b) in pairs.iter() {
            let i_a = vertices[a];
            let i_b = vertices[b];
            *edges[i_a].entry(i_b).or_default() = 1;
            *edges[i_b].entry(i_a).or_default() = 1;
        }

        Graph {
            v_index: vertices,
            v_name: v_names,
            edge: edges,
        }
    }

    fn dfs(&self, start: usize, to: usize, seen: &mut Vec<usize>, visited: &mut Vec<bool>) -> bool {
        seen.push(start);
        visited[start] = true;
        if start == to {
            return true;
        }

        let next_edges = &self.edge[start];
        for (&next, &capacity) in next_edges.iter() {
            if capacity > 0
                && !visited[next]
                && !seen.contains(&next)
                && self.dfs(next, to, seen, visited)
            {
                return true;
            }
        }

        seen.pop();
        false
    }

    fn reachable(&self, start: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        visited.insert(start);

        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            let edges = &self.edge[current];
            for (&next, &capacity) in edges {
                if capacity > 0 && visited.insert(next) {
                    to_visit.push_back(next);
                }
            }
        }

        visited
    }

    fn max_flow(&mut self, from: usize, to: usize) -> usize {
        let mut count = 0;
        let mut seen = vec![];
        let mut visited = vec![false; self.v_index.len()];
        while self.dfs(from, to, &mut seen, &mut visited) {
            for i in 0..seen.len() - 1 {
                let a = seen[i];
                let b = seen[i + 1];
                self.inc_capacity(a, b, -1);
                self.inc_capacity(b, a, 1);
            }

            seen.clear();
            visited = vec![false; self.v_index.len()];
            count += 1;
        }
        count
    }

    fn inc_capacity(&mut self, from: usize, to: usize, value: i32) {
        let current = self.edge[from].entry(to).or_default();
        *current += value;

        if *current == 0 {
            self.edge[from].remove(&to);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    impl Graph {
        fn num_edges(&self) -> usize {
            self.edge
                .iter()
                .map(|v| v.values().filter(|&v| *v >= 1).count())
                .sum::<usize>()
        }
    }

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "603368 456");
    }

    #[test]
    fn test_from_lines() {
        let network = Graph::from_lines(example());
        let v = |vertice: &str| -> &usize { &network.v_index[vertice] };

        assert_eq!(network.v_index.len(), 15);
        assert_eq!(network.num_edges(), 66);

        assert_ne!(network.edge[*v("cmg")][v("bvb")], 0);
        assert_ne!(network.edge[*v("bvb")][v("cmg")], 0);
    }

    #[test]
    fn test_dfs() {
        let network = Graph::from_lines(example());
        let v = |vertice: &str| -> usize { network.v_index[vertice] };

        let mut seen = vec![];
        let mut visited = vec![false; network.v_index.len()];
        assert!(network.dfs(v("jqt"), v("jqt"), &mut seen, &mut visited));
        assert_eq!(seen.len(), 1);
        assert_eq!(seen, vec![v("jqt")]);

        let mut seen = vec![];
        let mut visited = vec![false; network.v_index.len()];
        assert!(network.dfs(v("jqt"), v("xhk"), &mut seen, &mut visited));
        assert_eq!(*seen.first().unwrap(), v("jqt"));
        assert_eq!(*seen.last().unwrap(), v("xhk"));
    }

    #[test]
    fn test_max_flow() {
        let network = Graph::from_lines(example());
        let v = |vertex: &str| -> usize { network.v_index[vertex] };

        for v1 in [
            "cmg", "frs", "lhk", "lsr", "nvd", "pzl", "qnr", "rsh", "rzs",
        ] {
            for v2 in ["bvb", "hfx", "jqt", "ntq", "rhn", "xhk"] {
                let flow1 = network.clone().max_flow(v(v1), v(v2));
                let flow2 = network.clone().max_flow(v(v2), v(v1));
                assert_eq!(
                    flow1, flow2,
                    "{0} -> {1}: {2} || {1} -> {0}: {3}",
                    v1, v2, flow1, flow2
                );
                assert_eq!(flow1, 3, "{} -> {}: {}", v1, v2, flow1);
                assert_eq!(flow2, 3, "{} -> {}: {}", v1, v2, flow2);
                println!("{0} -> {1}: {2} || {1} -> {0}: {3}", v1, v2, flow1, flow2);
            }
        }
    }

    #[test]
    fn test_reachable() {
        let network = Graph::from_lines(example());
        let v = |vertex: &str| -> usize { network.v_index[vertex] };

        for v1 in [
            "cmg", "frs", "lhk", "lsr", "nvd", "pzl", "qnr", "rsh", "rzs",
        ] {
            for v2 in ["bvb", "hfx", "jqt", "ntq", "rhn", "xhk"] {
                let mut g1 = network.clone();
                g1.max_flow(v(v1), v(v2));
                let s1 = g1.reachable(v(v1));
                assert_eq!(s1.len(), 9);

                let mut g2 = network.clone();
                g2.max_flow(v(v2), v(v1));
                let s2 = g2.reachable(v(v2));
                assert_eq!(s2.len(), 6);
            }
        }
    }

    #[test]
    fn test_assumptions() {
        let example_pairs = pairs_from_lines(example());

        let uniq_pairs = HashSet::<(String, String)>::from_iter(example_pairs.clone().into_iter());
        assert_eq!(example_pairs.len(), 33);
        assert_eq!(uniq_pairs.len(), example_pairs.len());

        let mut components = HashSet::new();
        for pair in uniq_pairs {
            components.insert(pair.0);
            components.insert(pair.1);
        }
        assert_eq!(components.len(), 15);

        for component in components {
            let degree = example_pairs
                .iter()
                .filter(|(a, b)| *a == component || *b == component)
                .count();
            println!("{:?}: {}", component, degree);
        }
    }

    #[test]
    fn test_my_assumptions() {
        let data = aoc_utils::read_lines("input/day25.txt");

        let my_pairs = pairs_from_lines(data);
        let my_uniq_pairs = HashSet::<(String, String)>::from_iter(my_pairs.clone().into_iter());
        assert_eq!(my_pairs.len(), 3490);
        assert_eq!(my_uniq_pairs.len(), my_pairs.len());

        let mut components = HashSet::new();
        for pair in my_uniq_pairs {
            components.insert(pair.0);
            components.insert(pair.1);
        }
        assert_eq!(components.len(), 1554);

        for component in components {
            let degree = my_pairs
                .iter()
                .filter(|(a, b)| *a == component || *b == component)
                .count();
            println!("{:?}: {}", component, degree);
        }
    }

    fn pairs_from_lines(lines: Vec<String>) -> Vec<(String, String)> {
        let mut pairs = vec![];
        for line in lines {
            let (source, targets_str) = line.split_once(": ").unwrap();
            let targets = targets_str.split(" ");
            for target in targets {
                pairs.push((source.to_string(), target.to_string()));
            }
        }
        pairs
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day25-example.txt")
    }
}
