pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day4.txt");
    let parsed = parse_data(&data);

    let part1 = count_accessible_spaces(&parsed);

    let (all_removed, _) = remove_all_accessible_spaces(get_counts_map(&parsed));
    let part2 = all_removed;

    format!("{} {}", part1, part2)
}

fn parse_data(data: &[String]) -> Vec<Vec<bool>> {
    data.iter()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_accessible_spaces(data: &Vec<Vec<bool>>) -> usize {
    let counts = get_counts_map(data);
    counts
        .iter()
        .flat_map(|row| row.iter().filter(|&count| count.is_some_and(|c| c < 4)))
        .count()
}

fn get_counts_map(data: &Vec<Vec<bool>>) -> Vec<Vec<Option<usize>>> {
    let mut counts = vec![vec![None; data[0].len()]; data.len()];
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] {
                counts[y][x] = Some(count_surrounding_rolls(data, x, y));
            }
        }
    }
    counts
}

fn count_surrounding_rolls(data: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in 0..=2 {
        // 0 means -1
        for dy in 0..=2 {
            if dx == 1 && dy == 1 {
                continue;
            }
            if x == 0 && dx == 0 {
                continue;
            }
            if y == 0 && dy == 0 {
                continue;
            }
            let new_x = x + dx - 1;
            let new_y = y + dy - 1;
            if new_y >= data.len() || new_x >= data[new_y].len() {
                continue;
            }
            if data[new_y][new_x] {
                count += 1;
            }
        }
    }
    count
}

fn remove_all_accessible_spaces(
    mut counts: Vec<Vec<Option<usize>>>,
) -> (usize, Vec<Vec<Option<usize>>>) {
    let mut removed = 0;
    let mut removed_now: usize;
    loop {
        (removed_now, counts) = remove_accessible_spaces(counts);
        removed += removed_now;
        if removed_now == 0 {
            break;
        }
    }
    (removed, counts)
}
fn remove_accessible_spaces(
    mut counts: Vec<Vec<Option<usize>>>,
) -> (usize, Vec<Vec<Option<usize>>>) {
    let mut to_remove = vec![];
    for y in 0..counts.len() {
        for x in 0..counts[y].len() {
            if counts[y][x].is_some_and(|c| c < 4) {
                to_remove.push((x, y));
            }
        }
    }
    for &(x, y) in to_remove.iter() {
        counts[y][x] = None;
        for dx in 0..=2 {
            for dy in 0..=2 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                if x == 0 && dx == 0 {
                    continue;
                }
                if y == 0 && dy == 0 {
                    continue;
                }
                let new_x = x + dx - 1;
                let new_y = y + dy - 1;
                if new_y >= counts.len() || new_x >= counts[new_y].len() {
                    continue;
                }
                if counts[new_y][new_x].is_some() {
                    let new_count = counts[new_y][new_x].unwrap() - 1;
                    counts[new_y][new_x] = Some(new_count);
                }
            }
        }
    }

    (to_remove.len(), counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1489 8890");
    }

    #[test]
    fn test_parse_data() {
        let parsed = parse_data(example().as_slice());
        assert_eq!(parsed.len(), 10);
        for row in parsed.iter() {
            assert_eq!(row.len(), 10);
        }
        assert!(!parsed[0][0]);
        assert!(!parsed[0][1]);
        assert!(parsed[0][2]);
        assert!(parsed[1][0]);
        assert!(parsed[2][0]);
        assert!(parsed[9][8]);
        assert!(!parsed[8][9]);
        assert!(!parsed[9][9]);
    }

    #[test]
    fn test_example() {
        let parsed = parse_data(example().as_slice());
        assert_eq!(count_accessible_spaces(&parsed), 13);

        let counts_map = get_counts_map(&parsed);

        let (removed, counts_map) = remove_accessible_spaces(counts_map);
        assert_eq!(removed, 13);
        assert_eq!(counts_map[0][2], None);
        assert_eq!(counts_map[0][3], None);
        assert_eq!(counts_map[1][0], None);
        assert_eq!(counts_map[2][0], Some(3));
        assert_eq!(counts_map[8][8], Some(4));
        assert_eq!(counts_map[9][8], None);

        let (removed, counts_map) = remove_accessible_spaces(counts_map);
        assert_eq!(removed, 12);
        assert_eq!(counts_map[2][0], None);
        assert_eq!(counts_map[8][8], Some(4));

        let counts_map = get_counts_map(&parsed);
        let (removed, counts_map) = remove_all_accessible_spaces(counts_map);
        assert_eq!(removed, 43);
    }

    #[test]
    fn test_count_surrounding_rolls() {
        let parsed = parse_data(example().as_slice());
        assert_eq!(count_surrounding_rolls(&parsed, 0, 0), 2);
        assert_eq!(count_surrounding_rolls(&parsed, 1, 0), 4);
        assert_eq!(count_surrounding_rolls(&parsed, 2, 0), 3);
        assert_eq!(count_surrounding_rolls(&parsed, 0, 1), 3);
        assert_eq!(count_surrounding_rolls(&parsed, 0, 2), 4);
        assert_eq!(count_surrounding_rolls(&parsed, 8, 9), 2);
        assert_eq!(count_surrounding_rolls(&parsed, 9, 8), 4);
        assert_eq!(count_surrounding_rolls(&parsed, 9, 9), 2);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("..@@.@@@@."),
            String::from("@@@.@.@.@@"),
            String::from("@@@@@.@.@@"),
            String::from("@.@@@@..@."),
            String::from("@@.@@@@.@@"),
            String::from(".@@@@@@@.@"),
            String::from(".@.@.@.@@@"),
            String::from("@.@@@.@@@@"),
            String::from(".@@@@@@@@."),
            String::from("@.@.@@@.@."),
        ]
    }
}
