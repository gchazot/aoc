use crate::utils;
use std::collections::{HashMap, HashSet};

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let garden = GardenPatch::from_lines(utils::read_lines("src/day21/mine.txt"));

    assert_eq!(3847, garden.navigate(64));
}

struct GardenPatch {
    plots: Vec<Coordinates>,
    start: Coordinates,
    width: i64,
    height: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinates(i64, i64);

impl GardenPatch {
    fn from_lines(lines: Vec<String>) -> GardenPatch {
        let mut plots = Vec::new();
        let mut start = Coordinates(0, 0);
        let height = lines.len() as i64;
        let width = lines[0].len() as i64;
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), width as usize);
            for (x, c) in line.chars().enumerate() {
                let coords = Coordinates(x as i64, y as i64);
                match c {
                    '.' => {
                        plots.push(coords);
                    }
                    'S' => {
                        start = coords.clone();
                        plots.push(coords);
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

    fn navigate(&self, max_steps: i64) -> usize {
        let mut distances = HashMap::new();
        let mut frontline = HashSet::from([self.start.clone()]);
        let mut distance: i64 = 0;

        while !frontline.is_empty() && distance <= max_steps {
            let mut new_frontline = HashSet::new();

            for position in frontline.into_iter() {
                if !distances.contains_key(&position) {
                    distances.insert(position.clone(), distance);
                } else if distance < *distances.get(&position).unwrap() {
                    distances.insert(position.clone(), distance);
                } else {
                    continue;
                }

                for next_pos in [
                    Coordinates(position.0 + 1, position.1),
                    Coordinates(position.0 - 1, position.1),
                    Coordinates(position.0, position.1 + 1),
                    Coordinates(position.0, position.1 - 1),
                ] {
                    if self.plots.contains(&next_pos) {
                        new_frontline.insert(next_pos);
                    }
                }
            }

            frontline = new_frontline;
            distance += 1;
        }

        distances
            .iter()
            .filter(|(_coord, &dist)| dist % 2 == max_steps % 2)
            .count()
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

    assert_eq!(16, garden.navigate(6));
}
