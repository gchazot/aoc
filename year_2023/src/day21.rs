use std::collections::{HashMap, HashSet};

pub fn execute() -> String {
    let garden = GardenPatch::from_lines(aoc_utils::read_lines("input/day21.txt"));

    let part1 = garden.count_part_1(64);
    let part2 = garden.count_part_2(26501365);

    format!("{} {}", part1, part2)
}

struct GardenPatch {
    plots: HashSet<Coordinates>,
    start: Coordinates,
    side: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinates(i64, i64);

impl GardenPatch {
    fn from_lines(lines: Vec<String>) -> GardenPatch {
        let side_len = lines.len() as i64;
        assert!(lines.iter().all(|l| l.len() == side_len as usize));

        let mut plots = HashSet::new();
        let mut start = Coordinates(0, 0);

        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), side_len as usize);
            for (x, c) in line.chars().enumerate() {
                let coords = Coordinates(x as i64, y as i64);
                match c {
                    '.' => {
                        plots.insert(coords);
                    }
                    'S' => {
                        start = coords.clone();
                        plots.insert(coords);
                    }
                    '#' => {}
                    _ => unreachable!(),
                }
            }
        }
        GardenPatch {
            plots,
            start,
            side: side_len,
        }
    }

    fn count_part_1(&self, max_steps: i64) -> usize {
        let from = &self.start;
        let mut navigator = PatchNavigator::new(from.clone());
        navigator.navigate(self, Some(max_steps));

        let distances = navigator.distances;

        let even = distances
            .iter()
            .filter(|(_coord, &dist)| dist % 2 == 0)
            .count();
        let odd = distances.len() - even;

        if max_steps % 2 == 0 {
            even
        } else {
            odd
        }
    }

    fn count_part_2(&self, max_steps: i64) -> usize {
        let mut same_blocks = 1usize;
        let mut other_blocks = 0usize;
        let mut trimmings = 0usize;

        let mut start_nav = PatchNavigator::new(self.start.clone());
        let (even, odd) = start_nav.count_even_and_odd_steps(self, max_steps);
        let (same, other) = same_other(max_steps, even, odd);

        for next_op in vec![
            PatchNavigator::next_left,
            PatchNavigator::next_right,
            PatchNavigator::next_up,
            PatchNavigator::next_down,
        ] {
            trimmings += _explore_direction(self, &start_nav, next_op, max_steps)
        }

        for corner in vec![
            Coordinates(0, 0),
            Coordinates(self.side - 1, 0),
            Coordinates(0, self.side - 1),
            Coordinates(self.side - 1, self.side - 1),
        ] {
            let distance = start_nav.distances.get(&corner);
            if distance.is_some() && max_steps > *distance.unwrap() {
                let remaining_steps = max_steps - distance.unwrap();

                let next_start = Coordinates(self.side - 1 - corner.0, self.side - 1 - corner.1);

                let same_diagonals = remaining_steps / (2 * self.side);
                let same_steps = remaining_steps - 2 * same_diagonals * self.side;

                let mut same_nav = PatchNavigator::new(next_start.clone());
                let (even, odd) = same_nav.count_even_and_odd_steps(self, same_steps - 1);
                let (same, _other) = same_other(max_steps, even, odd);

                same_blocks += (same_diagonals * same_diagonals) as usize;
                trimmings += same * (1 + 2 * same_diagonals) as usize;

                let other_diagonals = (remaining_steps - self.side) / (2 * self.side);
                let other_steps = remaining_steps - (2 * other_diagonals + 1) * self.side;

                let mut other_nav = PatchNavigator::new(next_start);
                let (even, odd) = other_nav.count_even_and_odd_steps(self, other_steps - 1);
                let (_same, other) = same_other(max_steps, even, odd);

                other_blocks += (other_diagonals * (other_diagonals + 1)) as usize;
                trimmings += other * 2 * (1 + other_diagonals) as usize;
            }
        }

        same_blocks * same + other_blocks * other + trimmings
    }
}

fn same_other(max_steps: i64, even: usize, odd: usize) -> (usize, usize) {
    if max_steps % 2 == 0 {
        (even, odd)
    } else {
        (odd, even)
    }
}

fn _explore_direction(
    garden: &GardenPatch,
    nav: &PatchNavigator,
    next_op: fn(&mut PatchNavigator, &GardenPatch) -> bool,
    max_steps: i64,
) -> usize {
    let mut result = 0;

    let mut next_nav = nav.clone();

    if next_op(&mut next_nav, garden) {
        let (even, odd) = next_nav.count_even_and_odd_steps(garden, max_steps);
        let (same, other) = same_other(max_steps, even, odd);

        result += same;

        if next_nav.distances.iter().all(|(coord, &dist)| {
            nav.distances
                .get(coord)
                .is_some_and(|&d| dist == (d + garden.side))
        }) {
            let max_here = next_nav.distances.values().max().unwrap();

            let remaining_full_patches = (max_steps - max_here) / garden.side;
            result += same * (remaining_full_patches / 2) as usize;
            result += other * (remaining_full_patches / 2 + remaining_full_patches % 2) as usize;
            next_nav
                .distances
                .iter_mut()
                .for_each(|(_coord, dist)| *dist += remaining_full_patches * garden.side);
        }

        result += _explore_direction(garden, &next_nav, next_op, max_steps);
    }
    result
}

#[derive(Clone)]
struct PatchNavigator {
    distances: HashMap<Coordinates, i64>,
    frontline: HashSet<Coordinates>,
}
impl PatchNavigator {
    fn new(start: Coordinates) -> PatchNavigator {
        PatchNavigator {
            distances: HashMap::from([(start.clone(), 0)]),
            frontline: HashSet::from([start]),
        }
    }

    fn next_left(&mut self, garden: &GardenPatch) -> bool {
        let start_points = (0..garden.side)
            .filter_map(|y| {
                let previous = Coordinates(0, y);
                let next = Coordinates(garden.side - 1, y);

                if self.distances.contains_key(&previous) {
                    let previous_steps = self._distance(&previous);
                    Some((next, previous_steps + 1))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.frontline = start_points.keys().cloned().collect();
        self.distances = start_points;

        !self.distances.is_empty()
    }
    fn next_right(&mut self, garden: &GardenPatch) -> bool {
        let start_points = (0..garden.side)
            .filter_map(|y| {
                let previous = Coordinates(garden.side - 1, y);
                let next = Coordinates(0, y);

                if self.distances.contains_key(&previous) {
                    let previous_steps = self._distance(&previous);
                    Some((next, previous_steps + 1))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.frontline = start_points.keys().cloned().collect();
        self.distances = start_points;

        !self.distances.is_empty()
    }
    fn next_up(&mut self, garden: &GardenPatch) -> bool {
        let start_points = (0..garden.side)
            .filter_map(|x| {
                let previous = Coordinates(x, 0);
                let next = Coordinates(x, garden.side - 1);

                if self.distances.contains_key(&previous) {
                    let previous_steps = self._distance(&previous);
                    Some((next, previous_steps + 1))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.frontline = start_points.keys().cloned().collect();
        self.distances = start_points;

        !self.distances.is_empty()
    }
    fn next_down(&mut self, garden: &GardenPatch) -> bool {
        let start_points = (0..garden.side)
            .filter_map(|x| {
                let previous = Coordinates(x, garden.side - 1);
                let next = Coordinates(x, 0);

                if self.distances.contains_key(&previous) {
                    let previous_steps = self._distance(&previous);
                    Some((next, previous_steps + 1))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        self.frontline = start_points.keys().cloned().collect();
        self.distances = start_points;

        !self.distances.is_empty()
    }

    fn count_even_and_odd_steps(&mut self, garden: &GardenPatch, max_steps: i64) -> (usize, usize) {
        self.navigate(garden, Some(max_steps));

        let even = self
            .distances
            .iter()
            .filter(|(_coord, &dist)| dist <= max_steps && dist % 2 == 0)
            .count();
        let odd = self
            .distances
            .iter()
            .filter(|(_coord, &dist)| dist <= max_steps && dist % 2 != 0)
            .count();

        (even, odd)
    }

    fn navigate(&mut self, garden: &GardenPatch, max_steps: Option<i64>) {
        while !self.frontline.is_empty() {
            let min_dist = self
                .frontline
                .iter()
                .map(|position| self._distance(position))
                .reduce(i64::min)
                .unwrap();

            let frontline = self
                .frontline
                .iter()
                .filter(|&position| self._distance(position) == min_dist)
                .cloned()
                .collect::<Vec<_>>();

            for position in frontline.iter() {
                self.frontline.remove(&position);

                let distance = self._distance(&position) + 1;
                if max_steps.is_some_and(|max_steps| distance > max_steps) {
                    continue;
                }

                for next_pos in [
                    Coordinates(position.0 + 1, position.1),
                    Coordinates(position.0 - 1, position.1),
                    Coordinates(position.0, position.1 + 1),
                    Coordinates(position.0, position.1 - 1),
                ] {
                    if garden.plots.contains(&next_pos) {
                        if !self.distances.contains_key(&next_pos) {
                            self.distances.insert(next_pos.clone(), distance);
                        } else if distance < self._distance(&next_pos) {
                            self.distances.insert(next_pos.clone(), distance);
                        } else {
                            continue;
                        }

                        self.frontline.insert(next_pos);
                    }
                }
            }
        }
    }

    fn _distance(&self, position: &Coordinates) -> i64 {
        *self.distances.get(position).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "3847 637537341306357");
    }

    #[test]
    fn test_from_lines() {
        let lines = _example();

        let example = GardenPatch::from_lines(lines);

        assert_eq!(81, example.plots.len());
        assert_eq!(Coordinates(5, 5), example.start);
    }

    fn _example() -> Vec<String> {
        vec![
            "...........".to_string(),
            ".....###.#.".to_string(),
            ".###.##..#.".to_string(),
            "..#.#...#..".to_string(),
            "....#.#....".to_string(),
            ".##..S####.".to_string(),
            ".##..#...#.".to_string(),
            ".......##..".to_string(),
            ".##.#.####.".to_string(),
            ".##..##.##.".to_string(),
            "...........".to_string(),
        ]
    }

    #[test]
    fn test_example() {
        let garden = GardenPatch::from_lines(_example());

        assert_eq!(16, garden.count_part_1(6));

        assert_eq!(50, garden.count_part_2(10));
        assert_eq!(1594, garden.count_part_2(50));
        assert_eq!(6536, garden.count_part_2(100));
        assert_eq!(167004, garden.count_part_2(500));
        assert_eq!(668697, garden.count_part_2(1000));
        assert_eq!(16733044, garden.count_part_2(5000));
    }

    impl PatchNavigator {
        fn _print(&self, garden: &GardenPatch, width: usize) {
            for y in 0..garden.side {
                let line = (0..garden.side)
                    .map(|x| {
                        let coordinates = Coordinates(x, y);
                        let distance = self.distances.get(&coordinates);

                        if distance.is_some() {
                            format!("{:^width$}", distance.unwrap())
                        } else if garden.plots.contains(&coordinates) {
                            format!("{:^width$}", ".")
                        } else {
                            format!("{:^width$}", "#")
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", line);
            }
        }
    }
}
