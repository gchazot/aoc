use std::collections::{HashMap, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day12.txt");
    let plots = from_lines(&data);
    let areas = to_areas(&plots);

    let part1 = calculate_cost_part1(&areas);
    let part2 = calculate_cost_part2(&areas);

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

fn calculate_cost_part1(plots: &Vec<Vec<u16>>) -> u64 {
    let size = plots.len();
    let mut area = HashMap::<u16, u64>::new();
    let mut perimeter = HashMap::<u16, u64>::new();

    for (j, row) in plots.iter().enumerate() {
        for (i, &id) in row.iter().enumerate() {
            *area.entry(id).or_default() += 1;

            let mut borders = 0;
            if i == 0 || row[i - 1] != id {
                borders += 1;
            }
            if i == size - 1 || row[i + 1] != id {
                borders += 1;
            }
            if j == 0 || plots[j - 1][i] != id {
                borders += 1;
            }
            if j == size - 1 || plots[j + 1][i] != id {
                borders += 1;
            }

            if borders > 0 {
                *perimeter.entry(id).or_default() += borders;
            }
        }
    }
    perimeter.iter().map(|(id, &p)| p * area[id]).sum()
}
fn calculate_cost_part2(plots: &Vec<Vec<u16>>) -> u64 {
    let size = plots.len();
    let mut area = HashMap::<u16, u64>::new();
    let mut sides = HashMap::<u16, u64>::new();

    for (j, row) in plots.iter().enumerate() {
        for (i, &id) in row.iter().enumerate() {
            *area.entry(id).or_default() += 1;

            let mut corners = 0;

            // 0 1 2
            // 3 X 4
            // 5 6 7
            let neighbour = [
                !(i == 0 || j == 0 || plots[j - 1][i - 1] != id),
                !(j == 0 || plots[j - 1][i] != id),
                !(i == size - 1 || j == 0 || plots[j - 1][i + 1] != id),
                !(i == 0 || row[i - 1] != id),
                !(i == size - 1 || row[i + 1] != id),
                !(i == 0 || j == size - 1 || plots[j + 1][i - 1] != id),
                !(j == size - 1 || plots[j + 1][i] != id),
                !(i == size - 1 || j == size - 1 || plots[j + 1][i + 1] != id),
            ];

            // top-left
            if (neighbour[1] && neighbour[3] && !neighbour[0]) || (!neighbour[1] && !neighbour[3]) {
                corners += 1;
            }
            // top-right
            if (neighbour[1] && neighbour[4] && !neighbour[2]) || (!neighbour[1] && !neighbour[4]) {
                corners += 1;
            }
            // bottom-left
            if (neighbour[6] && neighbour[3] && !neighbour[5]) || (!neighbour[6] && !neighbour[3]) {
                corners += 1;
            }
            // bottom-right
            if (neighbour[6] && neighbour[4] && !neighbour[7]) || (!neighbour[6] && !neighbour[4]) {
                corners += 1;
            }

            if corners > 0 {
                *sides.entry(id).or_default() += corners;
            }
        }
    }
    sides.iter().map(|(id, &p)| p * area[id]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1477924 841934");
    }

    #[test]
    fn test_from_lines() {
        let e1 = from_lines(&example1());
        assert_eq!(e1.len(), 4);
        let e2 = from_lines(&example3());
        assert_eq!(e2.len(), 10);
    }

    #[test]
    fn test_calculate_cost_part1() {
        let data1 = from_lines(&example1());
        let areas1 = to_areas(&data1);
        assert_eq!(calculate_cost_part1(&areas1), 140);

        let data2 = from_lines(&example2());
        let areas2 = to_areas(&data2);
        assert_eq!(calculate_cost_part1(&areas2), 772);

        let data3 = from_lines(&example3());
        let areas3 = to_areas(&data3);
        assert_eq!(calculate_cost_part1(&areas3), 1930);
    }

    #[test]
    fn test_calculate_cost_part2() {
        let data1 = from_lines(&example1());
        let areas1 = to_areas(&data1);
        assert_eq!(calculate_cost_part2(&areas1), 80);

        let data2 = from_lines(&example2());
        let areas2 = to_areas(&data2);
        assert_eq!(calculate_cost_part2(&areas2), 436);

        let data3 = from_lines(&example3());
        let areas3 = to_areas(&data3);
        assert_eq!(calculate_cost_part2(&areas3), 1206);
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
