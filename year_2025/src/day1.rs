pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day1.txt");

    let part1 = count_zeros(&data, false);
    let part2 = count_zeros(&data, true);

    format!("{} {}", part1, part2)
}

fn count_zeros(data: &Vec<String>, count_all: bool) -> i32 {
    let mut current: i32 = 50;
    let mut count = 0;

    for line in data {
        let dir = line.chars().nth(0).unwrap();
        let steps = line[1..].parse::<i32>().unwrap();

        if count_all {
            count += steps / 100;
        }

        let increment = match dir {
            'L' => -(steps % 100),
            'R' => steps % 100,
            _ => panic!("Invalid direction: {}", dir),
        };

        let mut next = current + increment;

        if next > 99 {
            next -= 100;
            if count_all && next != 0 {
                count += 1
            }
        }
        if next < 0 {
            next += 100;
            if count_all && current != 0 {
                count += 1
            }
        }

        if next == 0 && increment != 0 {
            count += 1;
        }

        current = next;

        assert!(current < 100 && current >= 0);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = example();
        assert_eq!(count_zeros(&data, false), 3);
        assert_eq!(count_zeros(&data, true), 6);
    }
    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1139 6684");
    }

    fn example() -> Vec<String> {
        vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ]
    }
}
