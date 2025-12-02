pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day3.txt");
    let part1 = get_max_power(&data, 2);
    let part2 = get_max_power(&data, 12);

    format!("{} {}", part1, part2)
}

fn get_max_power(data: &Vec<String>, num_batteries: usize) -> u64 {
    parse_data(data)
        .iter()
        .map(|bank| get_best_joltage(bank, num_batteries))
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

fn parse_data(data: &Vec<String>) -> Vec<Vec<u8>> {
    data.iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn get_best_joltage(bank: &Vec<u8>, num_batteries: usize) -> u64 {
    let mut bat_powers = vec![];
    let mut range_start = 0;
    for n in 0..num_batteries {
        let range_end = bank.len() - (num_batteries - 1 - n);
        let (v, i) = get_max(&bank[range_start..range_end]);
        bat_powers.push(v);
        range_start += i + 1;
    }

    bat_powers
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| *v as u64 * 10u64.pow(i as u32))
        .sum()
}

fn get_max(values: &[u8]) -> (u8, usize) {
    let result = values
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, v)| **v)
        .unwrap();
    (*result.1, result.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "17144 170371185255900");
    }

    #[test]
    fn test_example() {
        assert_eq!(get_max_power(&example(), 2), 357);
        assert_eq!(get_max_power(&example(), 12), 3121910778619);
    }

    #[test]
    fn test_parse_data() {
        let banks = parse_data(&example());
        assert_eq!(banks.len(), 4);
        for bank in banks {
            assert_eq!(bank.len(), 15);
        }
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(
            get_best_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            98
        );
        assert_eq!(
            get_best_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
            89
        );
        assert_eq!(
            get_best_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
            78
        );
        assert_eq!(
            get_best_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
            92
        );
    }

    fn example() -> Vec<String> {
        vec![
            String::from("987654321111111"),
            String::from("811111111111119"),
            String::from("234234234234278"),
            String::from("818181911112111"),
        ]
    }
}
