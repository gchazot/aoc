use std::fs::read_to_string;

pub fn execute() -> String {
    let data = read_to_string("input/day3.txt").unwrap();
    let part1 = process_all(&data);
    let part2 = process_active(&data);

    format!("{} {}", part1, part2)
}

fn find_first_mul(input: &str) -> Option<(Option<(u32, u32)>, usize)> {
    const PAT1: &'static str = "mul(";
    const PAT2: &'static str = ")";
    const PAT3: &'static str = ",";

    let p1 = input.find(PAT1);

    if p1.is_none() {
        return None;
    }
    let pos1 = p1.unwrap() + PAT1.len();

    let p2 = input[pos1..].find(PAT2);
    if p2.is_none_or(|pos2| pos2 < 3 || pos2 > 7) {
        return Some((None, pos1));
    }
    let pos2 = p2.unwrap();

    let end = pos1 + pos2;

    let parts = input[pos1..end].split_once(PAT3);

    if parts.is_none() {
        return Some((None, pos1));
    }

    let (first, second) = parts.unwrap();
    let v1 = first.parse::<u32>();
    let v2 = second.parse::<u32>();
    if v1.is_ok() && v2.is_ok() {
        Some((Some((v1.unwrap(), v2.unwrap())), end + PAT2.len()))
    } else {
        Some((None, pos1))
    }
}

fn find_all(input: &str) -> Vec<(u32, u32)> {
    let mut results = Vec::new();
    let mut pos = 0;

    loop {
        let next = find_first_mul(&input[pos..]);
        if next.is_none() {
            return results;
        } else {
            let (values, delta) = next.unwrap();

            if values.is_some() {
                let r = values.unwrap();
                results.push((r.0, r.1));
            }
            pos += delta;
        }
    }
}

fn find_active(input: &str) -> Vec<(u32, u32)> {
    let mut results = Vec::new();

    let active_parts = input.split("do()").filter_map(|part| {
        part.split_once("don't()")
            .and_then(|(a, _)| Some(a))
            .or(Some(part))
    });

    for active_part in active_parts {
        let mut pos = 0;
        loop {
            let next = find_first_mul(&active_part[pos..]);
            if next.is_none() {
                break;
            } else {
                let (values, delta) = next.unwrap();

                if values.is_some() {
                    let r = values.unwrap();
                    results.push((r.0, r.1));
                }
                pos += delta;
            }
        }
    }
    results
}

fn process_all(input: &str) -> u32 {
    find_all(input).iter().map(|&(a, b)| a * b).sum()
}

fn process_active(input: &str) -> u32 {
    find_active(input).iter().map(|&(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "182780583 90772405");
    }

    #[test]
    fn test_find_next() {
        assert_eq!(None, find_first_mul("123456"));
        assert_eq!(Some((Some((2, 4)), 11)), find_first_mul("123mul(2,4)456"));
        assert_eq!(
            Some((Some((2, 4)), 11)),
            find_first_mul("123mul(2,4)45mul(8,9)6")
        );
        assert_eq!(
            Some((Some((8, 9)), 21 - 12)),
            find_first_mul(&"123mul(2,4)45mul(8,9)6"[12..])
        );
        assert_eq!(
            Some((Some((8, 9)), 21 - 4)),
            find_first_mul(&"123mul(2,4)45mul(8,9)6"[4..])
        );
        assert_eq!(
            Some((None, 5)),
            find_first_mul("+mul(32,64]then(mul(11,8)mul(8,5))")
        );
        assert_eq!(
            Some((Some((11, 8)), 20)),
            find_first_mul(&"+mul(32,64]then(mul(11,8)mul(8,5))"[5..])
        )
    }

    #[test]
    fn test_find_all() {
        assert_eq!(
            vec![(2, 4), (5, 5), (11, 8), (8, 5)],
            find_all(_example1().as_str())
        );
    }

    fn _example1() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    #[test]
    fn test_find_active() {
        assert_eq!(vec![(2, 4), (8, 5)], find_active(_example2().as_str()));
    }

    fn _example2() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    #[test]
    fn test_process() {
        assert_eq!(161, process_all(_example1().as_str()));
    }
}
