pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day20.txt");
    let race = Race::from_lines(&data);
    let path = race.find_path();

    let cheats_part1 = race.find_cheats(&path, 2);
    let part1 = count_cheats(&cheats_part1, 100, usize::MAX);

    let cheats_part2 = race.find_cheats(&path, 20);
    let part2 = count_cheats(&cheats_part2, 100, usize::MAX);

    format!("{} {}", part1, part2)
}

struct Race {
    track: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Race {
    fn from_lines(lines: &Vec<String>) -> Race {
        let width = lines[0].len();

        let mut start = (0, 0);
        let mut end = (0, 0);

        let track = lines
            .iter()
            .enumerate()
            .map(|(j, line)| {
                assert_eq!(line.len(), width);

                line.chars()
                    .enumerate()
                    .map(|(i, c)| match c {
                        '.' => true,
                        '#' => false,
                        'S' => {
                            start = (i, j);
                            true
                        }
                        'E' => {
                            end = (i, j);
                            true
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Race { track, start, end }
    }

    fn find_path(&self) -> Vec<(usize, usize)> {
        let mut result = vec![self.start];

        let mut prev = (0, 0);
        let mut current = self.start;
        while current != self.end {
            for option in [
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1),
            ] {
                if option != prev && self.track[option.1][option.0] {
                    result.push(option);
                    prev = current;
                    current = option;
                    break;
                }
            }
        }

        result
    }

    fn find_cheats(
        &self,
        path: &Vec<(usize, usize)>,
        max_duration: usize,
    ) -> Vec<((usize, usize), (usize, usize), usize)> {
        let mut result = vec![];

        for (i, pos1) in path.iter().enumerate() {
            for (j, pos2) in path.iter().enumerate().skip(i + 1) {
                let normal_duration = j - i;
                let cheat_duration = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1);
                if cheat_duration > max_duration || cheat_duration >= normal_duration {
                    continue;
                }
                result.push((*pos1, *pos2, normal_duration - cheat_duration));
            }
        }

        result
    }
}

fn count_cheats(
    cheats: &Vec<((usize, usize), (usize, usize), usize)>,
    min_gain: usize,
    max_gain: usize,
) -> usize {
    cheats
        .iter()
        .filter(|(_start, _end, gain)| min_gain <= *gain && *gain <= max_gain)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1411 1010263");
    }

    #[test]
    fn test_from_lines() {
        let race = Race::from_lines(&example());

        assert_eq!(race.start, (1, 3));
        assert_eq!(race.end, (5, 7));

        assert!(!race.track[0][0]);
        assert!(!race.track[0][1]);
        assert!(!race.track[1][0]);
        assert!(race.track[1][1]);
        assert!(race.track[1][2]);
        assert!(race.track[2][1]);
        assert!(race.track[11][13]);
    }

    #[test]
    fn test_find_path() {
        let race = Race::from_lines(&example());
        let path = race.find_path();
        assert_eq!(path.len(), 85);
        assert_eq!(path[0], race.start);
        assert_eq!(path[6], (3, 3));
        assert_eq!(path[14], (7, 3));
        assert_eq!(path[84], race.end);
    }

    #[test]
    fn test_find_cheats() {
        let race = Race::from_lines(&example());

        let path = race.find_path();

        let cheats_part1 = race.find_cheats(&path, 2);
        assert_eq!(cheats_part1.len(), 44);

        let cheats_part2 = race.find_cheats(&path, 20);
        assert_eq!(count_cheats(&cheats_part2, 50, 50), 32);
        assert_eq!(count_cheats(&cheats_part2, 52, 52), 31);
        assert_eq!(count_cheats(&cheats_part2, 54, 54), 29);
        assert_eq!(count_cheats(&cheats_part2, 74, 74), 4);
        assert_eq!(count_cheats(&cheats_part2, 76, 76), 3);
    }

    fn example() -> Vec<String> {
        aoc_utils::read_lines("input/day20-example.txt")
    }
}
