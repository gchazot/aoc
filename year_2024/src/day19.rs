use std::collections::HashMap;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day19.txt");
    let (designs, patterns) = from_lines(data);

    let part1 = count_possible_designs(&designs, &patterns);

    let mut cache = HashMap::new();
    let part2 = count_all_possible_designs(&designs, &patterns, &mut cache);

    format!("{} {}", part1, part2)
}

fn from_lines(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let patterns = lines[0]
        .split(", ")
        .map(|pat| pat.to_string())
        .collect::<Vec<_>>();
    let designs = lines[2..].to_vec();

    (designs, patterns)
}

fn count_possible_designs(designs: &Vec<String>, patterns: &Vec<String>) -> usize {
    designs
        .iter()
        .filter(|&design| is_design_possible(design, patterns))
        .count()
}

fn is_design_possible(design: &str, patterns: &Vec<String>) -> bool {
    if design.len() == 0 {
        return true;
    }
    for pattern in patterns {
        if design.starts_with(pattern) {
            if is_design_possible(&design[pattern.len()..], patterns) {
                return true;
            }
        }
    }
    false
}

fn count_all_possible_designs(
    designs: &Vec<String>,
    patterns: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    designs
        .iter()
        .map(|design| ways_design_is_possible(design.as_str(), patterns, cache))
        .sum()
}

fn ways_design_is_possible(
    design: &str,
    patterns: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if cache.contains_key(design) {
        return cache[design];
    }

    let result = patterns
        .iter()
        .filter_map(|pattern| {
            design
                .starts_with(pattern)
                .then(|| ways_design_is_possible(&design[pattern.len()..], patterns, cache))
        })
        .sum();
    cache.insert(design.to_string(), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "358 600639829400603");
    }

    #[test]
    fn test_is_design_possible() {
        let possible = |design: &str, patterns: &[&str]| {
            is_design_possible(
                &String::from(design),
                &patterns
                    .iter()
                    .map(|&p| String::from(p))
                    .collect::<Vec<String>>(),
            )
        };

        assert!(possible("r", &["r"]));
        assert!(possible("rrrr", &["r"]));
        assert!(!possible("r", &["g"]));
        assert!(!possible("rrrr", &["g"]));

        assert!(possible("rg", &["r", "g"]));
        assert!(possible("gr", &["r", "g"]));
        assert!(possible("rgb", &["r", "g", "b"]));
        assert!(!possible("g", &["r", "b"]));
        assert!(!possible("rgb", &["r", "b"]));

        assert!(possible("rg", &["rg", "b"]));
        assert!(possible("b", &["rg", "b"]));
        assert!(possible("rgb", &["rg", "b"]));
        assert!(possible("brg", &["rg", "b"]));
        assert!(possible("brgb", &["rg", "b"]));
        assert!(!possible("br", &["rg", "b"]));
    }

    #[test]
    fn test_count_possible_designs() {
        let (designs, patterns) = from_lines(example());
        assert_eq!(count_possible_designs(&designs, &patterns), 6);
    }

    #[test]
    fn test_count_all_possible_designs() {
        let (designs, patterns) = from_lines(example());
        let mut cache = HashMap::new();
        assert_eq!(
            count_all_possible_designs(&designs, &patterns, &mut cache),
            16
        );
    }

    fn example() -> Vec<String> {
        vec![
            String::from("r, wr, b, g, bwu, rb, gb, br"),
            String::from(""),
            String::from("brwrr"),
            String::from("bggr"),
            String::from("gbbr"),
            String::from("rrbgbr"),
            String::from("ubwu"),
            String::from("bwurrg"),
            String::from("brgr"),
            String::from("bbrgwb"),
        ]
    }
}
