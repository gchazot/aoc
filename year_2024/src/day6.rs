use std::collections::HashSet;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day6.txt");
    let (guard, direction, obstacles, size) = from_lines(data);
    let (route_len, _has_loop) = guard_route_length(guard, direction, &obstacles, size);
    let opportunities = obstacle_opportunities(guard, direction, &obstacles, size);

    let part1 = route_len;
    let part2 = opportunities.len();

    format!("{} {}", part1, part2)
}
type Coordinates = (i16, i16);

fn coord_to_offset(coordinates: Coordinates, size: i16) -> usize {
    coordinates.0 as usize * size as usize + coordinates.1 as usize
}

const UP: Coordinates = (0, -1);
const DOWN: Coordinates = (0, 1);
const LEFT: Coordinates = (-1, 0);
const RIGHT: Coordinates = (1, 0);
fn direction_to_offset(direction: Coordinates) -> usize {
    match direction {
        UP => 0,
        DOWN => 1,
        LEFT => 2,
        RIGHT => 3,
        _ => unreachable!("Invalid direction {:?}", direction),
    }
}

fn from_lines(lines: Vec<String>) -> (Coordinates, Coordinates, Vec<bool>, i16) {
    let direction = (0, -1);
    let mut guard = (-1, -1);

    let size = lines.len() as i16;
    let mut obstacles = vec![false; coord_to_offset((size, 0), size)];

    for (j, line) in lines.iter().enumerate() {
        assert_eq!(line.len() as i16, size);
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles[coord_to_offset((i as i16, j as i16), size)] = true;
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
    obstacles: &Vec<bool>,
    size: i16,
) -> Option<(Coordinates, Coordinates)> {
    let next = (guard.0 + direction.0, guard.1 + direction.1);
    if next.0 >= 0 && next.1 >= 0 && next.0 < size && next.1 < size {
        if obstacles[coord_to_offset(next, size)] {
            let new_direction = (-direction.1, direction.0);
            step(guard, new_direction, obstacles, size)
        } else {
            Some((next, direction))
        }
    } else {
        None
    }
}

fn guard_route_length(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &Vec<bool>,
    size: i16,
) -> (usize, bool) {
    let (route, has_loop) = guard_route(guard, direction, obstacles, size);
    (route.len(), has_loop)
}

fn guard_route(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &Vec<bool>,
    size: i16,
) -> (Vec<Coordinates>, bool) {
    let mut route = vec![guard];

    let mut visited = vec![false; coord_to_offset((size, 0), size)];
    visited[coord_to_offset(guard, size)] = true;

    let mut seen = vec![false; coord_to_offset((size, 0), size) * 4];
    seen[coord_to_offset(guard, size) * 4 + direction_to_offset(direction)] = true;

    let mut cur_guard = guard;
    let mut cur_direction = direction;

    let mut has_loop = false;

    while let Some((next, direction)) = step(cur_guard, cur_direction, obstacles, size) {
        if !visited[coord_to_offset(next, size)] {
            visited[coord_to_offset(next, size)] = true;
            route.push(next);
        }
        cur_guard = next;
        cur_direction = direction;
        if seen[(coord_to_offset(cur_guard, size) * 4) + direction_to_offset(cur_direction)] {
            has_loop = true;
            break;
        }
        seen[coord_to_offset(cur_guard, size) * 4 + direction_to_offset(cur_direction)] = true;
    }
    (route, has_loop)
}

fn obstacle_opportunities(
    guard: Coordinates,
    direction: Coordinates,
    obstacles: &Vec<bool>,
    size: i16,
) -> HashSet<Coordinates> {
    let (initial_route, has_loop) = guard_route(guard, direction, &obstacles, size);
    assert!(!has_loop);

    let mut options = initial_route.clone();
    options.remove(0);

    let mut already_seen = HashSet::new();

    let mut previous_guard = guard;

    let mut already_tried = HashSet::new();
    let mut option_obstacles = obstacles.clone();
    let mut result = HashSet::new();
    for option in options {
        let mut previous_direction = (option.0 - previous_guard.0, option.1 - previous_guard.1);
        if previous_direction.0.abs() > 1 {
            previous_direction = (
                previous_direction.0 / previous_direction.0.abs(),
                previous_direction.1,
            );
        }
        if previous_direction.1.abs() > 1 {
            previous_direction = (
                previous_direction.0,
                previous_direction.1 / previous_direction.1.abs(),
            );
        }

        already_seen.insert((previous_guard, previous_direction));

        if already_tried.insert(option) {
            assert!(!option_obstacles[coord_to_offset(option, size)]);
            option_obstacles[coord_to_offset(option, size)] = true;
            let (_route, has_loop) =
                guard_route(previous_guard, previous_direction, &option_obstacles, size);
            option_obstacles[coord_to_offset(option, size)] = false;

            if has_loop {
                result.insert(option);
            }
        }

        previous_guard = option;
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
        assert_eq!(direction, (0, -1));
        assert_eq!(guard, (4, 6));
        assert_eq!(size, 10);
        assert_eq!(
            obstacles
                .iter()
                .map(|&v| if v { 1 } else { 0 })
                .sum::<i32>(),
            8
        );
    }

    #[test]
    fn test_step_without_obstacle() {
        let size = 10;
        let no_obstacle = vec![false; coord_to_offset((size, 0), size)];

        assert_eq!(Some(((4, 3), UP)), step((4, 4), UP, &no_obstacle, size));
        assert_eq!(None, step((4, 0), UP, &no_obstacle, size));

        assert_eq!(Some(((4, 5), DOWN)), step((4, 4), DOWN, &no_obstacle, size));
        assert_eq!(None, step((4, 9), DOWN, &no_obstacle, size));

        assert_eq!(
            Some(((5, 4), RIGHT)),
            step((4, 4), RIGHT, &no_obstacle, size)
        );
        assert_eq!(None, step((9, 4), RIGHT, &no_obstacle, size));

        assert_eq!(Some(((3, 4), LEFT)), step((4, 4), LEFT, &no_obstacle, size));
        assert_eq!(None, step((0, 4), LEFT, &no_obstacle, size));
    }

    #[test]
    fn test_step_with_obstacle() {
        let size = 10;
        let no_obstacle = vec![false; coord_to_offset((size, 0), size)];

        let mut mid_block = no_obstacle.clone();
        mid_block[coord_to_offset((4, 4), size)] = true;

        assert_eq!(Some(((5, 5), RIGHT)), step((4, 5), UP, &mid_block, 10));
        assert_eq!(Some(((3, 3), LEFT)), step((4, 3), DOWN, &mid_block, 10));
        assert_eq!(Some(((3, 5), DOWN)), step((3, 4), RIGHT, &mid_block, 10));
        assert_eq!(Some(((5, 3), UP)), step((5, 4), LEFT, &mid_block, 10));

        //    ..#<.
        //    v....
        //    #...#
        //    ....^
        //    .>#..
        let size = 5;
        let base = vec![false; coord_to_offset((size, 0), size)];

        let mut right_block = base.clone();
        right_block[coord_to_offset((4, 2), size)] = true;
        assert_eq!(None, step((4, 3), UP, &right_block, 5));

        let mut left_block = base.clone();
        left_block[coord_to_offset((0, 2), size)] = true;
        assert_eq!(None, step((0, 1), DOWN, &left_block, 5));

        let mut down_block = base.clone();
        down_block[coord_to_offset((2, 4), size)] = true;
        assert_eq!(None, step((1, 4), RIGHT, &down_block, 5));

        let mut up_block = base.clone();
        up_block[coord_to_offset((2, 0), size)] = true;
        assert_eq!(None, step((3, 0), LEFT, &up_block, 5));
    }

    #[test]
    fn test_guard_route() {
        let (guard, direction, obstacles, size) = from_lines(_example());
        let (route_len, has_loop) = guard_route_length(guard, direction, &obstacles, size);

        assert_eq!(route_len, 41);
        assert!(!has_loop);
    }

    #[test]
    fn test_guard_route_with_loop() {
        let (guard, direction, mut obstacles, size) = from_lines(_example());
        obstacles[coord_to_offset((3, 6), size)] = true;
        let (route_len, has_loop) = guard_route_length(guard, direction, &obstacles, size);

        assert_eq!(route_len, 18);
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
