use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[test]
fn test_prime_factors() {
    fn check(number: u32, factors: Vec<u32>) {
        let result = prime_factors(&number);
        assert_eq!(
            result.iter().map(|(_n, &exponent)| exponent).sum::<u32>(),
            factors.len() as u32
        );

        for (number, &exponent) in result.iter() {
            let expected = factors.iter().filter(|&n| n == number).count() as u32;
            assert_eq!(exponent, expected);
        }
    }

    check(2, vec![2]);
    check(3, vec![3]);
    check(4, vec![2, 2]);
    check(5, vec![5]);
    check(6, vec![2, 3]);
    check(7, vec![7]);
    check(8, vec![2, 2, 2]);
    check(9, vec![3, 3]);
}

pub fn prime_factors(number: &u32) -> HashMap<u32, u32> {
    let mut result = HashMap::new();

    let mut n = *number;
    for i in primes(&n) {
        while n % i == 0 {
            n = &n / i;

            let count = result.entry(i).or_insert(0);
            *count += 1;
        }
    }
    result
}

pub fn primes(number: &u32) -> impl Iterator<Item = u32> {
    let mut sieve = HashSet::new();
    let &n = number;

    let upper_bound = (n as f64).sqrt().floor() as u32 + 1;
    for x in 2..upper_bound {
        for y in 2..(n / x) + 1 {
            sieve.insert(x * y as u32);
        }
    }

    (2..n + 1).filter(move |&i| !&sieve.contains(&i))
}
