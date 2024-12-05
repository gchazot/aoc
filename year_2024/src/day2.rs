pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day2.txt");
    let reports = read_reports(data);
    let part1 = count_safe(&reports, 0);
    let part2 = count_safe(&reports, 1);

    format!("{} {}", part1, part2)
}

fn count_safe(reports: &Vec<Vec<i32>>, max_problems: u32) -> usize {
    reports
        .iter()
        .filter(|&report| is_safe(report.as_slice(), max_problems))
        .count()
}

fn is_safe(list: &[i32], max_problems: u32) -> bool {
    assert!(list.len() >= 2);

    let mut increases = 0;
    let mut decreases = 0;

    for i in 1..list.len() {
        let diff = list[i] - list[i - 1];
        if diff > 0 {
            increases += 1;
        } else if diff < 0 {
            decreases += 1;
        }
    }

    let direction = increases > decreases;

    for i in 1..list.len() {
        let diff = list[i] - list[i - 1];
        if diff == 0 || ((diff > 0) != direction) || diff.abs() > 3 {
            if max_problems > 0 {
                let mut try1 = Vec::from(list);
                try1.remove(i - 1);
                let mut try2 = Vec::from(list);
                try2.remove(i);
                return is_safe(&try1, max_problems - 1) || is_safe(&try2, max_problems - 1);
            } else {
                return false;
            }
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
        assert_eq!(execute(), "282 349");
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
        assert!(is_safe(&[1, 2], 0));
        assert!(is_safe(&[1, 3], 0));
        assert!(is_safe(&[1, 4], 0));
        assert!(!is_safe(&[1, 5], 0));

        assert!(is_safe(&[2, 1], 0));
        assert!(is_safe(&[3, 1], 0));
        assert!(is_safe(&[4, 1], 0));
        assert!(!is_safe(&[5, 1], 0));

        assert!(!is_safe(&[1, 1], 0));
        assert!(!is_safe(&[5, 5], 0));

        assert!(is_safe(&[1, 2, 5], 0));
        assert!(!is_safe(&[1, 2, 6], 0));
        assert!(!is_safe(&[1, 2, 1], 0));

        assert!(is_safe(&[6, 5, 2], 0));
        assert!(!is_safe(&[6, 5, 1], 0));
        assert!(!is_safe(&[6, 5, 6], 0));

        assert!(is_safe(&[1, 2, 5], 1));
        assert!(is_safe(&[1, 2, 6], 1));
        assert!(is_safe(&[1, 2, 1], 1));

        assert!(is_safe(&[6, 5, 2], 1));
        assert!(is_safe(&[6, 5, 1], 1));
        assert!(is_safe(&[6, 5, 6], 1));

        assert!(is_safe(&[5, 5, 6], 1));
        assert!(is_safe(&[5, 5, 4], 1));
        assert!(is_safe(&[1, 5, 4, 3], 1));
        assert!(is_safe(&[8, 5, 6, 7], 1));
    }

    #[test]
    fn test_count_safe() {
        let examples = read_reports(_example());
        assert_eq!(count_safe(&examples, 0), 2);
        assert_eq!(count_safe(&examples, 1), 4);
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
