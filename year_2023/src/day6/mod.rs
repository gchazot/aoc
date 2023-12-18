use std::iter::zip;
use crate::utils;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let races = parse_races("mine.txt");
    assert_eq!(633080, optimize_races(&races));
}

#[test]
fn test_optimize_races() {
    let races = parse_races("example.txt");
    assert_eq!(288, optimize_races(&races));
}

fn optimize_races(races: &Vec<Race>) -> u32 {
    return races.iter()
        .map(|race|race.optimize())
        .map(|(min, max)|max-min+1)
        .product();
}

#[test]
fn test_parse_races() {
    let races = parse_races("example.txt");
    assert_eq!(3, races.len());

    assert_eq!(7, races[0].time);
    assert_eq!(9, races[0].dist);
    assert_eq!(15, races[1].time);
    assert_eq!(40, races[1].dist);
    assert_eq!(30, races[2].time);
    assert_eq!(200, races[2].dist);
}

fn parse_races(filename: &str) -> Vec<Race> {
    let path = format!("src/day6/{}", &filename);
    let lines = utils::read_lines(&path);

    let times_line = lines[0].clone();
    let dists_line = lines[1].clone();

    let (_times_header, times_str) = times_line.split_once(":").unwrap();
    let (_dists_header, dists_str) = dists_line.split_once(":").unwrap();

    let times = times_str
        .trim()
        .split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap);
    let dists = dists_str
        .trim()
        .split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap);

    return Vec::from_iter(
        zip(times, dists)
            .map(|(time, dist)| Race{time, dist})
    )
}

struct Race {
    time: u32,
    dist: u32,
}

#[test]
fn test_optimize_race() {
    assert_eq!((2, 5), Race{time:7, dist:9}.optimize());
    assert_eq!((4, 11), Race{time:15, dist:40}.optimize());
    assert_eq!((11, 19), Race{time:30, dist:200}.optimize());
}

impl Race {
    fn optimize(&self) -> (u32, u32) {
        let mut min = 1u32;
        let mut max = self.time;

        for i in 1..self.time/2 {
            min = i;
            if self.time*i - i*i > self.dist {
                break;
            }
        }
        for i in self.time/2..self.time {
            if self.time*i - i*i <= self.dist {
                break;
            }
            max = i;
        }

        return (min, max);
    }
}
