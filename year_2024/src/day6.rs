use std::collections::HashSet;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day6.txt");
    let (guard, direction, obstacles, size) = from_lines(data);
    let (route, _has_loop) = guard_route(guard, direction, &obstacles, size);
    let opportunities = obstacle_opportunities(guard, direction, &obstacles, size);

    let part1 = route.len();
    let part2 = opportunities.len();

    format!("{} {}", part1, part2)
}
type Coordinates = (i16, i16);
const UP: Coordinates = (0, -1);
const DOWN: Coordinates = (0, 1);
const LEFT: Coordinates = (-1, 0);
const RIGHT: Coordinates = (1, 0);

fn from_lines(lines: Vec<String>) -> (Coordinates, Coordinates, HashSet<Coordinates>, i16) {
    let direction = (0, -1);
    let mut guard = (-1, -1);
    let mut obstacles = HashSet::new();

    let size = lines.len() as i16;

    for (j, line) in lines.iter().enumerate() {
        assert_eq!(line.len() as i16, size);
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((i as i16, j as i16));
                }
                '^' => {
                    guard = (i as i16, j as i16);
                }
                _ => {}
            }
        }
    }
    (guard, direction, obstacles, size)
}

fn step(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &HashSet<Coordinates>,
    size: i16,
) -> Option<(Coordinates, Coordinates)> {
    let next = (guard.0 + direction.0, guard.1 + direction.1);
    if next.0 >= 0 && next.1 >= 0 && next.0 < size && next.1 < size {
        if obstacles.contains(&next) {
            let new_direction = (-direction.1, direction.0);
            step(guard, new_direction, obstacles, size)
        } else {
            Some((next, direction))
        }
    } else {
        None
    }
}

fn guard_route(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &HashSet<Coordinates>,
    size: i16,
) -> (HashSet<Coordinates>, bool) {
    let mut result = HashSet::from([guard]);
    let mut seen = HashSet::from([(guard, direction)]);

    let mut cur_guard = guard;
    let mut cur_direction = direction;

    while let Some((next, direction)) = step(cur_guard, cur_direction, obstacles, size) {
        result.insert(next);
        cur_guard = next;
        cur_direction = direction;
        if !seen.insert((cur_guard, cur_direction)) {
            return (result, true);
        }
    }
    (result, false)
}

fn obstacle_opportunities(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &HashSet<Coordinates>,
    size: i16,
) -> HashSet<Coordinates> {
    let (initial_route, has_loop) = guard_route(guard, direction, &obstacles, size);
    assert!(!has_loop);

    let mut options = initial_route.clone();
    options.remove(&guard);

    let mut option_obstacles = obstacles.clone();
    let mut result = HashSet::new();
    for option in options {
        assert!(option_obstacles.insert(option));
        let (_route, has_loop) = guard_route(guard, direction, &option_obstacles, size);
        assert!(option_obstacles.remove(&option));

        if has_loop {
            result.insert(option);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "5153 1711");
    }

    #[test]
    fn test_from_lines() {
        let (guard, direction, obstacles, size) = from_lines(_example());
        assert_eq!(obstacles.len(), 8);
        assert_eq!(direction, (0, -1));
        assert_eq!(guard, (4, 6));
        assert_eq!(size, 10)
    }

    #[test]
    fn test_step_without_obstacle() {
        let no_obstacle = HashSet::new();
        assert_eq!(Some(((4, 3), UP)), step((4, 4), UP, &no_obstacle, 10));
        assert_eq!(None, step((4, 0), UP, &no_obstacle, 10));

        assert_eq!(Some(((4, 5), DOWN)), step((4, 4), DOWN, &no_obstacle, 10));
        assert_eq!(None, step((4, 9), DOWN, &no_obstacle, 10));

        assert_eq!(Some(((5, 4), RIGHT)), step((4, 4), RIGHT, &no_obstacle, 10));
        assert_eq!(None, step((9, 4), RIGHT, &no_obstacle, 10));

        assert_eq!(Some(((3, 4), LEFT)), step((4, 4), LEFT, &no_obstacle, 10));
        assert_eq!(None, step((0, 4), LEFT, &no_obstacle, 10));
    }

    #[test]
    fn test_step_with_obstacle() {
        let mid_block = HashSet::from([(4, 4)]);

        assert_eq!(Some(((5, 5), RIGHT)), step((4, 5), UP, &mid_block, 10));
        assert_eq!(Some(((3, 3), LEFT)), step((4, 3), DOWN, &mid_block, 10));
        assert_eq!(Some(((3, 5), DOWN)), step((3, 4), RIGHT, &mid_block, 10));
        assert_eq!(Some(((5, 3), UP)), step((5, 4), LEFT, &mid_block, 10));

        //    ..#<.
        //    v....
        //    #...#
        //    ....^
        //    .>#..
        let right_block = HashSet::from([(4, 2)]);
        assert_eq!(None, step((4, 3), UP, &right_block, 5));
        let left_block = HashSet::from([(0, 2)]);
        assert_eq!(None, step((0, 1), DOWN, &left_block, 5));
        let down_block = HashSet::from([(2, 4)]);
        assert_eq!(None, step((1, 4), RIGHT, &down_block, 5));
        let up_block = HashSet::from([(2, 0)]);
        assert_eq!(None, step((3, 0), LEFT, &up_block, 5));
    }

    #[test]
    fn test_guard_route() {
        let (guard, direction, obstacles, size) = from_lines(_example());
        let (route, has_loop) = guard_route(guard, direction, &obstacles, size);

        assert_eq!(route.len(), 41);
        assert!(!has_loop);
    }

    #[test]
    fn test_guard_route_with_loop() {
        let (guard, direction, mut obstacles, size) = from_lines(_example());
        obstacles.insert((3, 6));
        let (route, has_loop) = guard_route(guard, direction, &obstacles, size);

        assert_eq!(route.len(), 18);
        assert!(has_loop);
    }

    #[test]
    fn test_obstacle_opportunities() {
        let (guard, direction, obstacles, size) = from_lines(_example());
        let opportunities = obstacle_opportunities(guard, direction, &obstacles, size);
        assert_eq!(opportunities.len(), 6);
        assert!(opportunities.contains(&(3, 6)));
        assert!(opportunities.contains(&(6, 7)));
        assert!(opportunities.contains(&(7, 7)));
        assert!(opportunities.contains(&(1, 8)));
        assert!(opportunities.contains(&(3, 8)));
        assert!(opportunities.contains(&(7, 9)));
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ]
    }
}
