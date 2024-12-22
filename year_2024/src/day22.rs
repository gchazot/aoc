use std::collections::{HashMap, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day22.txt");
    let secrets = from_lines(data);
    let part1 = part1(&secrets);
    let part2 = part2(&secrets);

    format!("{} {}", part1, part2)
}

fn from_lines(lines: Vec<String>) -> Vec<u32> {
    lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn next(secret: u32) -> u32 {
    const PRUNE: u32 = !(!0 << 24);

    let mut result = secret;

    result ^= result << 6; // *64
    result &= PRUNE;

    result ^= result >> 5; // /32
    result &= PRUNE;

    result ^= result << 11; // * 2048
    result &= PRUNE;

    result
}

fn repeat(secret: u32, times: u32) -> u32 {
    let mut result = secret;
    for _ in 0..times {
        result = next(result);
    }
    result
}

fn part1(secrets: &Vec<u32>) -> u64 {
    secrets
        .iter()
        .map(|secret| repeat(*secret, 2000) as u64)
        .sum::<u64>()
}

fn part2(secrets: &Vec<u32>) -> i16 {
    let mut total_prices = HashMap::new();

    for secret in secrets {
        let secret_prices = sequences_and_prices(*secret);
        for (sequence, price) in secret_prices.into_iter() {
            total_prices
                .entry(sequence)
                .and_modify(|total| *total += price)
                .or_insert(price);
        }
    }

    *total_prices.values().max().unwrap()
}

fn sequences_and_prices(secret: u32) -> HashMap<Vec<i8>, i16> {
    let mut result = HashMap::new();

    let mut prices_and_deltas = vec![];
    let mut current = secret;
    let mut current_price = (current % 10) as i8;
    for _ in 0..2000 {
        let new = next(current);
        let new_price = (new % 10) as i8;

        prices_and_deltas.push((new_price, new_price - current_price));

        current = new;
        current_price = new_price;
    }

    let mut queue = VecDeque::new();
    for (price, delta) in prices_and_deltas {
        queue.push_back(delta);
        if queue.len() < 4 {
            continue;
        } else if queue.len() > 4 {
            queue.pop_front();
        }

        let sequence = Vec::from_iter(queue.iter().cloned());

        result.entry(sequence).or_insert(price as i16);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "12979353889 1449");
    }

    #[test]
    fn test_part1() {
        let secrets = from_lines(example());
        assert_eq!(part1(&secrets), 37327623);
    }

    #[test]
    fn test_repeat() {
        let times = 2000;
        assert_eq!(repeat(1, times), 8685429);
        assert_eq!(repeat(10, times), 4700978);
        assert_eq!(repeat(100, times), 15273692);
        assert_eq!(repeat(2024, times), 8667524);
    }

    #[test]
    fn test_next() {
        assert_eq!(next(123), 15887950);
        assert_eq!(next(15887950), 16495136);
        assert_eq!(next(16495136), 527345);
        assert_eq!(next(527345), 704524);
        assert_eq!(next(704524), 1553684);
        assert_eq!(next(1553684), 12683156);
        assert_eq!(next(12683156), 11100544);
        assert_eq!(next(11100544), 12249484);
        assert_eq!(next(12249484), 7753432);
        assert_eq!(next(7753432), 5908254);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("1"),
            String::from("10"),
            String::from("100"),
            String::from("2024"),
        ]
    }
}
