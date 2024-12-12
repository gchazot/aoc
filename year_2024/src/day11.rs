use std::collections::HashMap;
use std::fs::read_to_string;

pub fn execute() -> String {
    let data = read_to_string("input/day11.txt").unwrap();

    let part1 = blink_many_times(data.clone(), 25);
    let part2 = blink_many_times(data.clone(), 75);

    format!("{} {}", part1, part2)
}

type Cache = HashMap<(String, u16), u128>;

fn process_part(input: &String) -> Vec<String> {
    _actual_process_part(input)
}
fn _actual_process_part(input: &String) -> Vec<String> {
    let mut result = vec![];

    if input == "0" {
        result.push("1".to_string());
    } else if input.len() % 2 == 0 {
        let (a, b) = input.split_at(input.len() / 2);
        result.push(a.to_string());
        let clean = b
            .find(|c| c != '0')
            .and_then(|n| Some(&b[n..]))
            .unwrap_or("0");
        result.push(clean.to_string());
    } else {
        let i = input.parse::<usize>();
        if i.is_err() {
            println!("{}", input);
        }
        result.push(format!("{}", i.unwrap() * 2024).to_string());
    }
    result
}

fn process_arrangement(input: &Vec<String>) -> Vec<String> {
    input.iter().flat_map(|s| process_part(s)).collect()
}

fn process_arrangement_deep(input: &Vec<String>, depth: u16, cache: &mut Cache) -> u128 {
    input.iter().map(|s| process_deep(s, depth, cache)).sum()
}

fn process_deep(input: &String, depth: u16, cache: &mut Cache) -> u128 {
    let cache_key = (input.clone(), depth);

    if cache.contains_key(&cache_key) {
        cache[&cache_key]
    } else {
        if depth == 0 {
            1
        } else {
            let parts = process_part(input);
            let result = parts
                .into_iter()
                .map(|part| process_deep(&part, depth - 1, cache))
                .sum();

            cache.insert(cache_key, result);

            result
        }
    }
}

fn blink_many_times(input: String, n: u16) -> u128 {
    let start = input.split(" ").map(|s| s.to_string()).collect();

    let mut cache = Cache::new();
    process_arrangement_deep(&start, n, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "186175 220566831337810");
    }

    #[test]
    fn test_process_part() {
        assert_eq!(process_part(&"0".to_string()), vec!["1".to_string()]);
        assert_eq!(
            process_part(&"12".to_string()),
            vec!["1".to_string(), "2".to_string()]
        );
        assert_eq!(process_part(&"1".to_string()), vec!["2024".to_string()]);
        assert_eq!(
            process_part(&"1001".to_string()),
            vec!["10".to_string(), "1".to_string()]
        );
    }

    #[test]
    fn test_process_arrangement() {
        assert_eq!(process_arrangement(&seq("125 17")), seq("253000 1 7"));
        assert_eq!(
            process_arrangement(&seq("253000 1 7")),
            seq("253 0 2024 14168")
        );
        assert_eq!(
            process_arrangement(&seq("253 0 2024 14168")),
            seq("512072 1 20 24 28676032")
        );
        assert_eq!(
            process_arrangement(&seq("512072 1 20 24 28676032")),
            seq("512 72 2024 2 0 2 4 2867 6032")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(blink_many_times("125 17".to_string(), 6), 22);
        assert_eq!(blink_many_times("125 17".to_string(), 25), 55312);
    }

    #[test]
    fn test_process_deep() {
        let mut cache = HashMap::new();
        let result = process_deep(&"5688".to_string(), 75, &mut cache);
        println!("{:?}", cache.len());
        assert_eq!(result, 50932594354084)
    }

    #[test]
    fn test_process_arrangement_deep() {
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 0, &mut cache), 2);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 1, &mut cache), 3);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 2, &mut cache), 4);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 3, &mut cache), 5);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 4, &mut cache), 9);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 5, &mut cache), 13);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 6, &mut cache), 22);
        let mut cache = HashMap::new();
        assert_eq!(process_arrangement_deep(&seq("125 17"), 7, &mut cache), 31);
        let mut cache = HashMap::new();
        assert_eq!(
            process_arrangement_deep(&seq("125 17"), 25, &mut cache),
            55312
        );
    }

    fn seq(input: &str) -> Vec<String> {
        input.split(" ").map(|s| s.to_string()).collect()
    }

    fn _example() -> String {
        "125 17".to_string()
    }
}
