use std::iter::zip;

pub fn execute() -> String {
    let mine = aoc_utils::read_lines("input/day1.txt");
    let (list1, list2) = build_lists(mine.as_slice());
    let part1 = total_distance(list1.as_slice(), list2.as_slice());
    let part2 = similarity_score(list1.as_slice(), list2.as_slice());

    format!("{} {}", part1, part2)
}

fn build_lists(lines: &[String]) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in lines {
        let (str1, str2) = line.split_once(' ').unwrap();
        let int1 = str1.trim().parse::<u32>().unwrap();
        let int2 = str2.trim().parse::<u32>().unwrap();

        list1.push(int1);
        list2.push(int2);
    }
    (list1, list2)
}

fn count_in_list(number: u32, list: &[u32]) -> u32 {
    let mut result = 0;
    for &i in list {
        if i == number {
            result += 1;
        }
    }
    result
}

fn total_distance(list1: &[u32], list2: &[u32]) -> u32 {
    assert_eq!(list1.len(), list2.len());
    let mut sorted1 = Vec::from(list1);
    let mut sorted2 = Vec::from(list2);
    sorted1.sort();
    sorted2.sort();

    zip(sorted1, sorted2)
        .into_iter()
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn similarity_score(list1: &[u32], list2: &[u32]) -> u32 {
    list1.iter().map(|&a| a * count_in_list(a, list2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), String::from("1506483 23126924"));
    }

    #[test]
    fn test_build_lists() {
        let (list1, list2) = build_lists(_example().as_slice());
        assert_eq!(list1.len(), 6);
        assert_eq!(list2.len(), 6);

        assert_eq!(list1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_total_distance() {
        let (list1, list2) = build_lists(_example().as_slice());
        assert_eq!(total_distance(list1.as_slice(), list2.as_slice()), 11);
    }

    #[test]
    fn test_count_in_list() {
        let (_list1, list2) = build_lists(_example().as_slice());

        assert_eq!(count_in_list(3, &list2), 3);
        assert_eq!(count_in_list(4, &list2), 1);
        assert_eq!(count_in_list(2, &list2), 0);
        assert_eq!(count_in_list(1, &list2), 0);
        assert_eq!(count_in_list(3, &list2), 3);
    }

    #[test]
    fn test_similarity_score() {
        let (list1, list2) = build_lists(_example().as_slice());
        assert_eq!(similarity_score(list1.as_slice(), list2.as_slice()), 31);
    }

    fn _example() -> Vec<String> {
        vec![
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ]
    }
}
