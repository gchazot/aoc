pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day22.txt");
    let secrets = from_lines(data);
    let part1 = secrets
        .iter()
        .map(|secret| repeat(*secret, 2000) as u64)
        .sum::<u64>();
    let part2 = 456;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "12979353889 456");
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
        vec![String::from("123"), String::from("456")]
    }
}
