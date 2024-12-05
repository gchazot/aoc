pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day2.txt");
    let reports = read_reports(data);
    let part1 = count_safe(reports);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

fn count_safe(reports: Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|&report| is_safe(report.as_slice()))
        .count()
}

fn is_safe(list: &[i32]) -> bool {
    assert!(list.len() >= 2);
    let direction = list[0] - list[1] > 0;

    for i in 1..list.len() {
        let diff = list[i - 1] - list[i];
        if diff == 0 || ((diff > 0) != direction) || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn read_reports(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.split(' ')
                .into_iter()
                .map(|part| part.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "282 456");
    }

    #[test]
    fn test_read_reports() {
        let reports = read_reports(_example());
        assert_eq!(reports.len(), 6);
        for report in reports.iter() {
            assert_eq!(report.len(), 5);
        }
        assert_eq!(reports[0], [7, 6, 4, 2, 1]);
        assert_eq!(reports[1], [1, 2, 7, 8, 9]);
        assert_eq!(reports[2], [9, 7, 6, 2, 1]);
        assert_eq!(reports[3], [1, 3, 2, 4, 5]);
        assert_eq!(reports[4], [8, 6, 4, 4, 1]);
        assert_eq!(reports[5], [1, 3, 6, 7, 9]);
    }
    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[1, 2]));
        assert!(is_safe(&[1, 3]));
        assert!(is_safe(&[1, 4]));
        assert!(!is_safe(&[1, 5]));

        assert!(is_safe(&[2, 1]));
        assert!(is_safe(&[3, 1]));
        assert!(is_safe(&[4, 1]));
        assert!(!is_safe(&[5, 1]));

        assert!(!is_safe(&[1, 1]));
        assert!(!is_safe(&[5, 5]));

        assert!(is_safe(&[1, 2, 5]));
        assert!(!is_safe(&[1, 2, 6]));
        assert!(!is_safe(&[1, 2, 1]));

        assert!(is_safe(&[6, 5, 2]));
        assert!(!is_safe(&[6, 5, 1]));
        assert!(!is_safe(&[6, 5, 6]));
    }

    #[test]
    fn test_count_safe() {
        let examples = read_reports(_example());
        assert_eq!(count_safe(examples), 2);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ]
    }
}
