use std::collections::{HashMap, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day11.txt");
    let tangle = from_lines(&data);

    let part1 = count_paths(&tangle, "you", "out");

    let part2 = count_paths_trough(
        &tangle,
        "svr",
        "out",
        vec!["fft".to_string(), "dac".to_string()],
    );

    format!("{} {}", part1, part2)
}

fn from_lines(lines: &[String]) -> HashMap<String, Vec<String>> {
    lines
        .iter()
        .map(|line| {
            let (from, tos) = line.split_once(": ").unwrap();
            (
                from.to_string(),
                tos.split(" ").map(|to| to.to_string()).collect(),
            )
        })
        .collect()
}

fn count_paths(tangle: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut to_explore = VecDeque::from([vec![start.to_string()]]);
    let mut paths = vec![];

    while let Some(current) = to_explore.pop_front() {
        let current_node = current.last().unwrap();
        if current_node == end {
            // println!("{:?}", current);
            paths.push(current);
        } else {
            let next_options = tangle.get(current_node);
            if next_options.is_some() {
                for next_option in next_options.unwrap() {
                    if !current.contains(next_option) {
                        let mut next = current.clone();
                        next.push(next_option.clone());
                        to_explore.push_back(next);
                    } else {
                        let mut looped = current.clone();
                        looped.push(next_option.clone());
                        println!("Loop {:?}", looped);
                    }
                }
            }
        }
    }

    paths.len()
}

fn count_paths_trough(
    tangle: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    through: Vec<String>,
) -> usize {
    dfs(
        tangle,
        &mut vec![start.to_string()],
        &mut HashMap::new(),
        end,
        &through,
    )
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Key {
    from: String,
    via: Vec<String>,
}

fn dfs(
    tangle: &HashMap<String, Vec<String>>,
    current: &mut Vec<String>,
    seen: &mut HashMap<Key, usize>,
    end: &str,
    through: &Vec<String>,
) -> usize {
    let here = current.last().unwrap();

    if here == end {
        return if through.is_empty() { 1 } else { 0 };
    }

    let next_through = if through.contains(here) {
        &through.iter().filter(|&t| t != here).cloned().collect()
    } else {
        through
    };

    let mut found = 0;
    for next_option in tangle.get(here).unwrap_or(&vec![]) {
        let key = Key {
            from: next_option.clone(),
            via: next_through.clone(),
        };
        let cached = seen.get(&key);
        let result = if cached.is_some() {
            *cached.unwrap()
        } else {
            current.push(next_option.clone());
            let result = dfs(tangle, current, seen, end, next_through);
            current.pop();
            seen.insert(key, result);
            result
        };
        found += result;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "508 315116216513280");
    }

    #[test]
    fn test_count_paths() {
        let tangle1 = from_lines(&example1());

        assert_eq!(count_paths(&tangle1, "you", "bbb"), 1);
        assert_eq!(count_paths(&tangle1, "you", "ddd"), 2);
        assert_eq!(count_paths(&tangle1, "you", "aaa"), 0);
        assert_eq!(count_paths(&tangle1, "you", "out"), 5);

        let tangle2 = from_lines(&example2());
        assert_eq!(count_paths(&tangle2, "svr", "out"), 8);
    }
    #[test]
    fn test_count_paths_trough() {
        let tangle2 = from_lines(&example2());
        assert_eq!(
            count_paths_trough(
                &tangle2,
                "svr",
                "out",
                vec!["fft".to_string(), "dac".to_string()],
            ),
            2
        );
    }
    #[test]
    fn test_from_lines() {
        let tangle = from_lines(&example1());
        assert_eq!(tangle.len(), 10);
        assert_eq!(tangle["aaa"].len(), 2);
        assert_eq!(tangle["you"].len(), 2);
        assert_eq!(tangle["hhh"], vec!["ccc", "fff", "iii"]);
    }
    fn example1() -> Vec<String> {
        vec![
            String::from("aaa: you hhh"),
            String::from("you: bbb ccc"),
            String::from("bbb: ddd eee"),
            String::from("ccc: ddd eee fff"),
            String::from("ddd: ggg"),
            String::from("eee: out"),
            String::from("fff: out"),
            String::from("ggg: out"),
            String::from("hhh: ccc fff iii"),
            String::from("iii: out"),
        ]
    }
    fn example2() -> Vec<String> {
        vec![
            String::from("svr: aaa bbb"),
            String::from("aaa: fft"),
            String::from("fft: ccc"),
            String::from("bbb: tty"),
            String::from("tty: ccc"),
            String::from("ccc: ddd eee"),
            String::from("ddd: hub"),
            String::from("hub: fff"),
            String::from("eee: dac"),
            String::from("dac: fff"),
            String::from("fff: ggg hhh"),
            String::from("ggg: out"),
            String::from("hhh: out"),
        ]
    }
}
