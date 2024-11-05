use crate::utils;
use std::cmp::min;
use std::collections::HashMap;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let example_city = City::from_lines(utils::read_lines("src/day17/mine.txt"));

    let mut nav1 = Navigator::new(1, 3);
    assert_eq!(698, nav1.solve(&example_city));

    let mut nav2 = Navigator::new(4, 10);
    assert_eq!(825, nav2.solve(&example_city))
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

struct City {
    heat_loss: HashMap<Coord, u8>,
    width: u32,
    height: u32,
    factory: Coord,
}

impl City {
    fn from_lines(lines: Vec<String>) -> City {
        let heat_loss = HashMap::from_iter(
            lines
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            (
                                Coord {
                                    x: x as u32,
                                    y: y as u32,
                                },
                                c.to_string().parse::<u8>().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .fold(Vec::new(), |mut result, mut next| {
                    result.append(&mut next);
                    result
                }),
        );

        let factory = heat_loss
            .keys()
            .max_by_key(|coord| (coord.x, coord.y))
            .cloned()
            .unwrap();

        City {
            heat_loss,
            width: factory.x + 1,
            height: factory.y + 1,
            factory,
        }
    }
}

#[test]
fn test_from_lines() {
    let example = City::from_lines(_example());
    assert_eq!(example.heat_loss[&Coord { x: 0, y: 0 }], 2);
    assert_eq!(example.heat_loss[&Coord { x: 1, y: 0 }], 4);
    assert_eq!(example.heat_loss[&Coord { x: 2, y: 0 }], 1);
    assert_eq!(example.heat_loss[&Coord { x: 0, y: 1 }], 3);
    assert_eq!(example.heat_loss[&Coord { x: 0, y: 2 }], 3);

    assert_eq!(example.heat_loss[&Coord { x: 12, y: 0 }], 3);
    assert_eq!(example.heat_loss[&Coord { x: 0, y: 12 }], 4);
    assert_eq!(example.heat_loss[&Coord { x: 12, y: 12 }], 3);
}

fn _example() -> Vec<String> {
    vec![
        "2413432311323".to_string(),
        "3215453535623".to_string(),
        "3255245654254".to_string(),
        "3446585845452".to_string(),
        "4546657867536".to_string(),
        "1438598798454".to_string(),
        "4457876987766".to_string(),
        "3637877979653".to_string(),
        "4654967986887".to_string(),
        "4564679986453".to_string(),
        "1224686865563".to_string(),
        "2546548887735".to_string(),
        "4322674655533".to_string(),
    ]
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct NavPoint {
    coord: Coord,
    direction: Direction,
}

struct Navigator {
    nav_points: Vec<NavPoint>,
    costs: HashMap<NavPoint, u32>,
    crucible_min_steps: u32,
    crucible_max_steps: u32,
}

impl Navigator {
    fn new(crucible_min_steps: u32, crucible_max_steps: u32) -> Navigator {
        let mut nav_points = Vec::new();
        let mut costs = HashMap::new();

        use Direction::*;

        let start_east = NavPoint {
            coord: Coord { x: 0, y: 0 },
            direction: Horizontal,
        };
        nav_points.push(start_east.clone());
        costs.insert(start_east, 0);

        let start_south = NavPoint {
            coord: Coord { x: 0, y: 0 },
            direction: Vertical,
        };
        nav_points.push(start_south.clone());
        costs.insert(start_south, 0);

        Navigator {
            nav_points,
            costs,
            crucible_min_steps,
            crucible_max_steps,
        }
    }

    fn solve(&mut self, city: &City) -> u32 {
        while self.nav_points.len() > 0 {
            self.progress_all(city);
        }

        self.costs
            .iter()
            .filter_map(|(&ref nav, &cost)| {
                if nav.coord == city.factory {
                    Some(cost)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }
    fn progress_all(&mut self, city: &City) {
        let mut new_nav_points = vec![];
        for point in self.nav_points.iter() {
            let cost = self.costs[&point];
            let new_points = self.progress(city, &point);
            for (new_point, added_cost) in new_points {
                let new_cost = cost + added_cost;
                if self
                    .costs
                    .get(&new_point)
                    .is_none_or(|&previous_cost| previous_cost > new_cost)
                {
                    self.costs.insert(new_point.clone(), new_cost);
                    if new_point.coord != city.factory {
                        new_nav_points.push(new_point);
                    }
                }
            }
        }
        self.nav_points = new_nav_points;
    }

    fn progress(&self, city: &City, point: &NavPoint) -> Vec<(NavPoint, u32)> {
        use Direction::*;
        let mut results = vec![];

        let next_direction = match point.direction {
            Vertical => Horizontal,
            Horizontal => Vertical,
        };

        let max_steps = match next_direction {
            Vertical => [
                min(self.crucible_max_steps, point.coord.y),
                min(self.crucible_max_steps, city.height - 1 - point.coord.y),
            ],
            Horizontal => [
                min(self.crucible_max_steps, point.coord.x),
                min(self.crucible_max_steps, city.width - 1 - point.coord.x),
            ],
        };

        for (increase, index) in [(false, 0), (true, 1)] {
            for steps in self.crucible_min_steps..(max_steps[index] + 1) {
                let mut coord = point.coord.clone();
                let mut cost = 0;
                for _ in 0..steps {
                    match next_direction {
                        Vertical => {
                            if increase {
                                coord.y += 1
                            } else {
                                coord.y -= 1
                            }
                        }
                        Horizontal => {
                            if increase {
                                coord.x += 1
                            } else {
                                coord.x -= 1
                            }
                        }
                    };
                    cost += city.heat_loss[&coord];
                }

                results.push((
                    NavPoint {
                        coord,
                        direction: next_direction.clone(),
                    },
                    cost as u32,
                ))
            }
        }

        results
    }
}

#[test]
fn test_progress() {
    use Direction::*;

    let example_city = City::from_lines(_example());
    let nav = Navigator::new(1, 3);

    let p1 = NavPoint {
        coord: Coord { x: 0, y: 0 },
        direction: Horizontal,
    };
    let p1_next = nav.progress(&example_city, &p1);
    assert_eq!(p1_next.len(), 3);
    assert_eq!(
        HashMap::from([
            (Coord { x: 0, y: 1 }, 3),
            (Coord { x: 0, y: 2 }, 6),
            (Coord { x: 0, y: 3 }, 9),
        ]),
        p1_next
            .iter()
            .map(|(point, cost)| (point.coord.clone(), *cost))
            .collect::<HashMap<_, _>>(),
    );

    let p2 = NavPoint {
        coord: Coord { x: 0, y: 0 },
        direction: Vertical,
    };
    let p2_next = nav.progress(&example_city, &p2);
    assert_eq!(p2_next.len(), 3);
    assert_eq!(
        HashMap::from([
            (Coord { x: 1, y: 0 }, 4),
            (Coord { x: 2, y: 0 }, 5),
            (Coord { x: 3, y: 0 }, 8),
        ]),
        p2_next
            .iter()
            .map(|(point, cost)| (point.coord.clone(), *cost))
            .collect::<HashMap<_, _>>(),
    );
}

#[test]
fn test_progress_all() {
    let example_city = City::from_lines(_example());
    let mut nav = Navigator::new(1, 3);

    nav.progress_all(&example_city);

    assert_eq!(nav.nav_points.len(), 6);
    assert_eq!(
        HashMap::from([
            (Coord { x: 0, y: 1 }, 3),
            (Coord { x: 0, y: 2 }, 6),
            (Coord { x: 0, y: 3 }, 9),
            (Coord { x: 1, y: 0 }, 4),
            (Coord { x: 2, y: 0 }, 5),
            (Coord { x: 3, y: 0 }, 8),
        ]),
        nav.nav_points
            .iter()
            .map(|point| (point.coord.clone(), *nav.costs.get(point).unwrap()))
            .collect::<HashMap<_, _>>(),
    );

    assert_eq!(nav.costs.len(), 8);
}

#[test]
fn test_solve() {
    let example_city = City::from_lines(_example());

    let mut nav1 = Navigator::new(1, 3);
    assert_eq!(102, nav1.solve(&example_city));

    let mut nav2 = Navigator::new(4, 10);
    assert_eq!(94, nav2.solve(&example_city));
}
