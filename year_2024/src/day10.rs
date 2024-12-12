use std::collections::{HashMap, HashSet};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day10.txt");
    let topo = Topography::from_lines(&data);

    let part1 = topo.count_all_reachable_peaks();
    let part2 = topo.count_all_possible_routes();

    format!("{} {}", part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(u32, u32);

struct Topography {
    size: u32,
    altitudes: Vec<u32>,
}
impl Topography {
    fn from_lines(lines: &Vec<String>) -> Topography {
        let size = lines.len() as u32;

        let mut result = Topography {
            size,
            altitudes: vec![0; (size * size) as usize],
        };

        for (j, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), size as usize);

            for (i, c) in line.chars().enumerate() {
                let altitude = c.to_digit(10).unwrap();
                let position = Position(i as u32, j as u32);
                let offset = result.offset(position);
                result.altitudes[offset] = altitude;
            }
        }

        result
    }

    fn altitude(&self, position: Position) -> u32 {
        self.altitudes[self.offset(position)]
    }

    fn offset(&self, position: Position) -> usize {
        position.0 as usize * self.size as usize + position.1 as usize
    }

    fn position(&self, offset: usize) -> Position {
        let j = offset % self.size as usize;
        let i = offset / self.size as usize;
        Position(i as u32, j as u32)
    }

    fn count_all_reachable_peaks(&self) -> usize {
        let heads = self.trailheads();

        let head_peaks: Vec<Vec<Position>> = heads
            .iter()
            .map(|&reachable| self.reachable_peaks(reachable))
            .collect();

        head_peaks.iter().map(|peaks| peaks.len()).sum()
    }

    fn reachable_peaks(&self, start: Position) -> Vec<Position> {
        assert_eq!(self.altitude(start), 0);

        let mut reachable = HashSet::from([start]);
        for _ in 0..9 {
            reachable = reachable
                .iter()
                .flat_map(|&reached| self.next_reachable(reached))
                .collect();
        }
        reachable.into_iter().collect()
    }

    fn next_reachable(&self, from: Position) -> Vec<Position> {
        let mut result = vec![];

        let offset = self.offset(from);
        let altitude = self.altitude(from) + 1;

        if from.0 > 0 && self.altitudes[offset - self.size as usize] == altitude {
            result.push(Position(from.0 - 1, from.1));
        }
        if from.0 < self.size - 1 && self.altitudes[offset + self.size as usize] == altitude {
            result.push(Position(from.0 + 1, from.1));
        }
        if from.1 > 0 && self.altitudes[offset - 1] == altitude {
            result.push(Position(from.0, from.1 - 1));
        }
        if from.1 < self.size - 1 && self.altitudes[offset + 1] == altitude {
            result.push(Position(from.0, from.1 + 1));
        }
        result
    }

    fn count_all_possible_routes(&self) -> usize {
        let heads = self.trailheads();

        let head_routes: Vec<Vec<(Position, usize)>> = heads
            .iter()
            .map(|&reachable| self.possible_routes(reachable))
            .collect();

        head_routes
            .iter()
            .map(|routes| routes.iter().map(|&(_dest, count)| count).sum::<usize>())
            .sum()
    }

    fn possible_routes(&self, start: Position) -> Vec<(Position, usize)> {
        let mut reachable = vec![(start, 1)];
        for _ in 0..9 {
            let mut next_reachable = HashMap::new();
            reachable.iter().for_each(|&(reached, prev_count)| {
                let next_positions = self.next_reachable(reached);
                for position in next_positions {
                    next_reachable
                        .entry(position)
                        .and_modify(|count| *count += prev_count)
                        .or_insert(prev_count);
                }
            });
            reachable = next_reachable.into_iter().collect();
        }
        reachable.into_iter().collect()
    }

    fn trailheads(&self) -> Vec<Position> {
        self.altitudes
            .iter()
            .enumerate()
            .filter_map(|(i, &alt)| (alt == 0).then_some(self.position(i)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "582 1302");
    }

    #[test]
    fn test_from_lines() {
        let topo = Topography::from_lines(&_example());

        assert_eq!(topo.size, 8);
        assert_eq!(topo.altitude(Position(0, 0)), 8);
        assert_eq!(topo.altitude(Position(1, 1)), 8);
        assert_eq!(topo.altitude(Position(1, 0)), 9);
        assert_eq!(topo.altitude(Position(0, 1)), 7);
        assert_eq!(topo.altitude(Position(0, 6)), 0);
        assert_eq!(topo.altitude(Position(0, 7)), 1);
        assert_eq!(topo.altitude(Position(6, 0)), 2);
        assert_eq!(topo.altitude(Position(7, 0)), 3);
        assert_eq!(topo.altitude(Position(6, 6)), 0);
        assert_eq!(topo.altitude(Position(7, 7)), 2);
    }

    #[test]
    fn test_reachable_peaks() {
        let topo = Topography::from_lines(&_example());

        assert_eq!(topo.reachable_peaks(Position(2, 0)).len(), 5);
        assert_eq!(topo.count_all_reachable_peaks(), 36);
    }
    #[test]
    fn test_possible_routes() {
        let topo = Topography::from_lines(&_example());

        let possible_routes = topo.possible_routes(Position(2, 0));
        assert_eq!(possible_routes.len(), 5);
        assert_eq!(
            possible_routes
                .iter()
                .map(|&(_dest, count)| count)
                .sum::<usize>(),
            20
        );

        assert_eq!(topo.count_all_possible_routes(), 81);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("89010123"),
            String::from("78121874"),
            String::from("87430965"),
            String::from("96549874"),
            String::from("45678903"),
            String::from("32019012"),
            String::from("01329801"),
            String::from("10456732"),
        ]
    }
}
