use std::fs::read_to_string;

pub fn execute() -> String {
    let data = read_to_string("input/day11.txt").unwrap();

    let part1 = blink_many_times(data.clone(), 25);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

fn process_part(input: &String) -> Vec<String> {
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

fn blink_many_times(input: String, n: usize) -> usize {
    let mut result = input.split(" ").map(|s| s.to_string()).collect();
    for _ in 0..n {
        result = process_arrangement(&result);
    }
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "186175 456");
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

    fn seq(input: &str) -> Vec<String> {
        input.split(" ").map(|s| s.to_string()).collect()
    }

    fn _example() -> String {
        "125 17".to_string()
    }
}
