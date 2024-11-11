use crate::utils;
use std::collections::{HashMap, HashSet};

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let garden = GardenPatch::from_lines(utils::read_lines("src/day21/mine.txt"));

    assert_eq!(3847, garden.count_part_1(64));
}

struct GardenPatch {
    plots: HashSet<Coordinates>,
    start: Coordinates,
    width: i64,
    height: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinates(i64, i64);

impl GardenPatch {
    fn from_lines(lines: Vec<String>) -> GardenPatch {
        let mut plots = HashSet::new();
        let mut start = Coordinates(0, 0);
        let height = lines.len() as i64;
        let width = lines[0].len() as i64;
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), width as usize);
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
        assert_eq!(width, height);
        GardenPatch {
            plots,
            start,
            width,
            height,
        }
    }

    fn count_part_1(&self, max_steps: i64) -> usize {
        let distances = self.navigate(&self.start, max_steps);

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

    fn navigate(&self, from: &Coordinates, max_steps: i64) -> HashMap<Coordinates, i64> {
        let mut navigator = PatchNavigator::new(self, from.clone());
        navigator.navigate(Some(max_steps));
        navigator.distances
    }
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

struct PatchNavigator<'a> {
    garden: &'a GardenPatch,
    distances: HashMap<Coordinates, i64>,
    frontline: HashSet<Coordinates>,
}
impl PatchNavigator<'_> {
    fn new<'a>(garden: &'a GardenPatch, start: Coordinates) -> PatchNavigator<'a> {
        PatchNavigator {
            garden,
            distances: HashMap::from([(start.clone(), 0)]),
            frontline: HashSet::from([start]),
        }
    }

    fn navigate(&mut self, max_steps: Option<i64>) {
        while !self.frontline.is_empty() {
            let mut new_frontline = HashSet::new();

            for position in self.frontline.iter() {
                let distance = *self.distances.get(&position).unwrap() + 1;
                if max_steps.is_some_and(|max_steps| distance > max_steps) {
                    continue;
                }

                for next_pos in [
                    Coordinates(position.0 + 1, position.1),
                    Coordinates(position.0 - 1, position.1),
                    Coordinates(position.0, position.1 + 1),
                    Coordinates(position.0, position.1 - 1),
                ] {
                    if self.garden.plots.contains(&next_pos) {
                        if !self.distances.contains_key(&next_pos) {
                            self.distances.insert(next_pos.clone(), distance);
                        } else if distance < *self.distances.get(&next_pos).unwrap() {
                            self.distances.insert(next_pos.clone(), distance);
                        } else {
                            continue;
                        }

                        new_frontline.insert(next_pos);
                    }
                }
            }

            self.frontline = new_frontline;
        }
    }

    fn print(&self, width: usize) {
        for y in 0..self.garden.height {
            let line = (0..self.garden.width)
                .map(|x| {
                    let coordinates = Coordinates(x, y);
                    let distance = self.distances.get(&coordinates);

                    if distance.is_some() {
                        format!("{:^width$}", distance.unwrap())
                    } else if self.garden.plots.contains(&coordinates) {
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
