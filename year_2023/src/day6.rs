use std::iter::zip;

pub fn execute() -> String {
    let races = parse_races("day6.txt");
    let part1 = optimize_races(&races);

    let race = Race::from_file("day6.txt");
    let (min, max) = race.optimize();
    let part2 = max - min + 1;

    format!("{} {}", part1, part2)
}

fn optimize_races(races: &Vec<Race>) -> u64 {
    return races
        .iter()
        .map(|race| race.optimize())
        .map(|(min, max)| max - min + 1)
        .product();
}
fn parse_races(filename: &str) -> Vec<Race> {
    let path = format!("input/{}", &filename);
    let lines = aoc_utils::read_lines(&path);

    let times_line = lines[0].clone();
    let dists_line = lines[1].clone();

    let (_times_header, times_str) = times_line.split_once(":").unwrap();
    let (_dists_header, dists_str) = dists_line.split_once(":").unwrap();

    let times = times_str
        .trim()
        .split_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap);
    let dists = dists_str
        .trim()
        .split_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap);

    return Vec::from_iter(zip(times, dists).map(|(time, dist)| Race { time, dist }));
}

struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn optimize(&self) -> (u64, u64) {
        let mut min = 1u64;
        let mut max = self.time;

        for i in 1..self.time / 2 {
            min = i;
            if race(i, self.time) > self.dist as u64 {
                break;
            }
        }
        for i in self.time / 2..self.time {
            if race(i, self.time) <= self.dist as u64 {
                break;
            }
            max = i;
        }

        return (min, max);
    }

    fn from_file(filename: &str) -> Race {
        let path = format!("input/{}", &filename);
        let lines = aoc_utils::read_lines(&path);

        let time_line = lines[0].clone();
        let dist_line = lines[1].clone();

        let (_time_header, time_str) = time_line.split_once(":").unwrap();
        let (_dist_header, dist_str) = dist_line.split_once(":").unwrap();

        let time = time_str.replace(" ", "").parse::<u64>().unwrap();
        let dist = dist_str.replace(" ", "").parse::<u64>().unwrap();

        return Race { time, dist };
    }
}

fn race(charge_time: u64, race_time: u64) -> u64 {
    let race = race_time as u64;
    let charge = charge_time as u64;
    return race * charge - charge * charge;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mine() {
        assert_eq!(execute(), "633080 20048741");
    }

    #[test]
    fn test_optimize_races() {
        let races = parse_races("day6-example.txt");
        assert_eq!(288, optimize_races(&races));

        let race = Race::from_file("day6-example.txt");
        let (min, max) = race.optimize();
        assert_eq!(14, min);
        assert_eq!(71516, max);
        assert_eq!(71503, max - min + 1);
    }

    #[test]
    fn test_parse_races() {
        let races = parse_races("day6-example.txt");
        assert_eq!(3, races.len());

        assert_eq!(7, races[0].time);
        assert_eq!(9, races[0].dist);
        assert_eq!(15, races[1].time);
        assert_eq!(40, races[1].dist);
        assert_eq!(30, races[2].time);
        assert_eq!(200, races[2].dist);
    }

    #[test]
    fn test_parse_race_v2() {
        let race = Race::from_file("day6-example.txt");
        assert_eq!(71530, race.time);
        assert_eq!(940200, race.dist);
    }

    #[test]
    fn test_optimize_race() {
        assert_eq!((2, 5), Race { time: 7, dist: 9 }.optimize());
        assert_eq!((4, 11), Race { time: 15, dist: 40 }.optimize());
        assert_eq!(
            (11, 19),
            Race {
                time: 30,
                dist: 200
            }
            .optimize()
        );
    }
}
