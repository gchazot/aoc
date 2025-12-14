pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day7.txt");
    let manifold = Manifold::from_lines(data);

    let part1 = manifold.count_splits();
    let part2 = manifold.count_quantum_timelines();

    format!("{} {}", part1, part2)
}

struct Manifold {
    start: usize,
    layers: Vec<Vec<usize>>,
}

impl Manifold {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut start = 0;
        let mut layers = vec![];
        for (i, line) in lines.iter().enumerate() {
            let layer: Vec<usize> = line
                .chars()
                .enumerate()
                .filter_map(|(j, c)| (c != '.').then_some(j))
                .collect();
            assert!(i % 2 == 0 || layer.len() == 0, "Layer {i} is not empty");
            if i == 0 {
                assert_eq!(layer.len(), 1);
                start = layer[0];
            } else if layer.len() > 0 {
                layers.push(layer);
            }
        }
        Manifold { start, layers }
    }

    fn count_splits(&self) -> usize {
        let mut beams = vec![self.start];
        let mut splits = 0;
        for layer in self.layers.iter() {
            let mut new_beams = vec![];
            for beam in beams.iter() {
                if layer.contains(beam) {
                    new_beams.push(*beam - 1);
                    new_beams.push(*beam + 1);
                    splits += 1;
                } else {
                    new_beams.push(*beam);
                }
            }
            new_beams.sort();
            new_beams.dedup();
            beams = new_beams;
        }
        splits
    }

    fn count_quantum_timelines(&self) -> usize {
        let mut routes = vec![(self.start, 1)];
        for layer in self.layers.iter() {
            let mut new_routes = vec![];
            for (beam, count) in routes.iter() {
                if layer.contains(beam) {
                    new_routes.push((*beam - 1, *count));
                    new_routes.push((*beam + 1, *count));
                } else {
                    new_routes.push((*beam, *count));
                }
            }
            new_routes.sort();

            let mut i = 0;
            routes.clear();
            while i < new_routes.len() - 1 {
                if new_routes[i].0 == new_routes[i + 1].0 {
                    new_routes[i + 1] = (new_routes[i + 1].0, new_routes[i].1 + new_routes[i + 1].1)
                } else {
                    routes.push(new_routes[i]);
                }
                i += 1;
            }
            routes.push(new_routes[i]);
        }
        routes.iter().map(|(_, count)| count).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1613 48021610271997");
    }

    #[test]
    fn test_count_splits() {
        let manifold = Manifold::from_lines(example());
        assert_eq!(manifold.count_splits(), 21);
    }

    #[test]
    fn test_count_quantum_timelines() {
        let manifold = Manifold::from_lines(example());
        assert_eq!(manifold.count_quantum_timelines(), 40);
    }

    #[test]
    fn test_from_lines() {
        let manifold = Manifold::from_lines(example());
        assert_eq!(manifold.start, 7);
        assert_eq!(manifold.layers.len(), 7);
    }
    fn example() -> Vec<String> {
        vec![
            String::from(".......S......."),
            String::from("..............."),
            String::from(".......^......."),
            String::from("..............."),
            String::from("......^.^......"),
            String::from("..............."),
            String::from(".....^.^.^....."),
            String::from("..............."),
            String::from("....^.^...^...."),
            String::from("..............."),
            String::from("...^.^...^.^..."),
            String::from("..............."),
            String::from("..^...^.....^.."),
            String::from("..............."),
            String::from(".^.^.^.^.^...^."),
            String::from("..............."),
        ]
    }
}
