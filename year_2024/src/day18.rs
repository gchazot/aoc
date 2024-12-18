use std::collections::VecDeque;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day18.txt");

    let map = corruption_map_from_lines(data.clone(), 71, 1024);
    let part1 = dijkstra(&map, 71, (0, 0), (70, 70)).unwrap();

    let coords = coords_from_lines(data);
    let part2 = find_first_blocking_byte(coords, 71);

    format!("{} {},{}", part1, part2.0, part2.1)
}

fn corruption_map_from_lines(lines: Vec<String>, size: usize, limit: usize) -> Vec<Vec<bool>> {
    let coords = coords_from_lines(lines);
    corruption_map_from_coords(&coords, size, limit)
}

fn coords_from_lines(lines: Vec<String>) -> Vec<(usize, usize)> {
    lines
        .iter()
        .map(|l| {
            let (x_str, y_str) = l.split_once(",").unwrap();
            (
                x_str.parse::<usize>().unwrap(),
                y_str.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn corruption_map_from_coords(
    coords: &Vec<(usize, usize)>,
    size: usize,
    limit: usize,
) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; size]; size];

    for (x, y) in coords.iter().take(limit) {
        if *x < size && *y < size {
            result[*y][*x] = true;
        }
    }
    result
}

fn dijkstra(
    map: &[Vec<bool>],
    size: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut distance = vec![vec![usize::MAX; size]; size];

    while let Some((pos, dist)) = queue.pop_front() {
        if dist < distance[pos.1][pos.0] {
            distance[pos.1][pos.0] = dist;
        } else {
            continue;
        }

        if pos == end {
            break;
        }

        for next in valid_steps(pos, size) {
            if !map[next.1][next.0] {
                queue.push_back((next, dist + 1));
            }
        }
    }

    (distance[end.1][end.0] != usize::MAX).then_some(distance[end.1][end.0])
}

fn valid_steps(pos: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut next = vec![];
    if pos.0 > 0 {
        next.push((pos.0 - 1, pos.1));
    }
    if pos.0 < size - 1 {
        next.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        next.push((pos.0, pos.1 - 1));
    }
    if pos.1 < size - 1 {
        next.push((pos.0, pos.1 + 1));
    }
    next
}

fn find_first_blocking_byte(coords: Vec<(usize, usize)>, size: usize) -> (usize, usize) {
    let mut min = 0usize;
    let mut max = coords.len() - 1;

    while max > min + 1 {
        let mid = (max + min) / 2;
        let map = corruption_map_from_coords(&coords, size, mid);
        let result = dijkstra(&map, size, (0, 0), (size - 1, size - 1));
        if result.is_some() {
            min = mid;
        } else {
            max = mid;
        }
    }
    coords[max - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "296 28,44");
    }
    #[test]
    fn test_dijkstra() {
        let map = corruption_map_from_lines(example(), 7, 12);
        assert_eq!(dijkstra(&map, 7, (0, 0), (6, 6)), Some(22));
    }
    #[test]
    fn test_find_first_blocking_byte() {
        let coords = coords_from_lines(example());
        assert_eq!(find_first_blocking_byte(coords.clone(), 7), (6, 1));
    }
    #[test]
    fn test_from_lines() {
        let map = corruption_map_from_lines(example(), 7, 12);
        for (j, row) in map.iter().enumerate() {
            for (i, pixel) in row.iter().enumerate() {
                assert_eq!(
                    *pixel,
                    [
                        (5, 4),
                        (4, 2),
                        (4, 5),
                        (3, 0),
                        (2, 1),
                        (6, 3),
                        (2, 4),
                        (1, 5),
                        (0, 6),
                        (3, 3),
                        (2, 6),
                        (5, 1),
                    ]
                    .contains(&(i, j)),
                    "Mismatch ({},{})",
                    i,
                    j
                );
            }
        }
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day18-example.txt")
    }
}
