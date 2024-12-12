use std::collections::{HashMap, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day12.txt");
    let plots = from_lines(&data);

    let part1 = calculate_cost(&plots);
    let part2 = 456;

    format!("{} {}", part1, part2)
}

fn from_lines(lines: &Vec<String>) -> Vec<Vec<char>> {
    let size = lines.len();
    lines
        .iter()
        .map(|line| {
            assert_eq!(line.len(), size);
            line.chars().collect()
        })
        .collect()
}

fn to_areas(plots: &Vec<Vec<char>>) -> Vec<Vec<u16>> {
    let size = plots.len();
    let mut id = 0u16;

    let mut seen = vec![vec![false; size]; size];
    let mut seen_count = 0;
    let mut result = vec![vec![id; size]; size];

    while seen_count < size * size {
        id += 1;

        let start = first_unseen(&seen).unwrap();
        let plot_type = plots[start.1][start.0];
        let mut leads = VecDeque::from([start]);

        while !leads.is_empty() {
            let lead = leads.pop_front().unwrap();
            if seen[lead.1][lead.0] {
                continue;
            }

            seen[lead.1][lead.0] = true;
            seen_count += 1;
            result[lead.1][lead.0] = id;
            if lead.0 > 0 && !seen[lead.1][lead.0 - 1] && plots[lead.1][lead.0 - 1] == plot_type {
                leads.push_back((lead.0 - 1, lead.1));
            }
            if lead.0 < size - 1
                && !seen[lead.1][lead.0 + 1]
                && plots[lead.1][lead.0 + 1] == plot_type
            {
                leads.push_back((lead.0 + 1, lead.1));
            }
            if lead.1 > 0 && !seen[lead.1 - 1][lead.0] && plots[lead.1 - 1][lead.0] == plot_type {
                leads.push_back((lead.0, lead.1 - 1));
            }
            if lead.1 < size - 1
                && !seen[lead.1 + 1][lead.0]
                && plots[lead.1 + 1][lead.0] == plot_type
            {
                leads.push_back((lead.0, lead.1 + 1));
            }
        }
    }
    result
}

fn first_unseen(seen: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    for (j, row) in seen.iter().enumerate() {
        for (i, seen_plot) in row.iter().enumerate() {
            if !seen_plot {
                return Some((i, j));
            }
        }
    }
    None
}

fn calculate_cost(plots: &Vec<Vec<char>>) -> u64 {
    let plots = to_areas(plots);
    let size = plots.len();
    let mut area = HashMap::<u16, u64>::new();
    let mut perimeter = HashMap::<u16, u64>::new();

    for (j, row) in plots.iter().enumerate() {
        for (i, &id) in row.iter().enumerate() {
            *area.entry(id).or_default() += 1;
            if i == 0 || row[i - 1] != id {
                *perimeter.entry(id).or_default() += 1;
            }
            if i == size - 1 || row[i + 1] != id {
                *perimeter.entry(id).or_default() += 1;
            }
            if j == 0 || plots[j - 1][i] != id {
                *perimeter.entry(id).or_default() += 1;
            }
            if j == size - 1 || plots[j + 1][i] != id {
                *perimeter.entry(id).or_default() += 1;
            }
        }
    }
    perimeter.iter().map(|(id, &p)| p * area[id]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1477924 456");
    }

    #[test]
    fn test_from_lines() {
        let e1 = from_lines(&example1());
        assert_eq!(e1.len(), 4);
        let e2 = from_lines(&example3());
        assert_eq!(e2.len(), 10);
    }

    #[test]
    fn test_calculate_cost() {
        let e1 = from_lines(&example1());
        assert_eq!(calculate_cost(&e1), 140);

        let e2 = from_lines(&example2());
        assert_eq!(calculate_cost(&e2), 772);

        let e3 = from_lines(&example3());
        assert_eq!(calculate_cost(&e3), 1930);
    }

    fn example1() -> Vec<String> {
        vec![
            String::from("AAAA"),
            String::from("BBCD"),
            String::from("BBCC"),
            String::from("EEEC"),
        ]
    }

    fn example2() -> Vec<String> {
        aoc_utils::read_lines("input/day12-example2.txt")
    }
    fn example3() -> Vec<String> {
        aoc_utils::read_lines("input/day12-example3.txt")
    }
}
