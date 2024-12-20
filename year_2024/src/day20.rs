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

    fn find_path(&self) -> Vec<Vec<usize>> {
        let mut result = vec![vec![usize::MAX; self.track[0].len()]; self.track.len()];
        result[self.start.1][self.start.0] = 0;

        let mut prev = (0, 0);
        let mut current = self.start;
        let mut distance = 1;
        while current != self.end {
            for option in [
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1),
            ] {
                if option != prev && self.track[option.1][option.0] {
                    result[option.1][option.0] = distance;
                    prev = current;
                    current = option;
                    break;
                }
            }
            distance += 1;
        }

        result
    }

    fn find_cheats(
        &self,
        path: &Vec<Vec<usize>>,
        max_duration: usize,
    ) -> Vec<((usize, usize), (usize, usize), usize)> {
        let mut result = vec![];

        let height = path.len();
        let width = path[0].len();

        for i in 0..width {
            for j in 0..height {
                let a = (i, j);
                let distance_a = path[j][i];

                if distance_a == usize::MAX {
                    continue;
                }

                for cheat_duration in 2..max_duration + 1 {
                    let mut points_to_check = vec![];
                    for offset in 0..cheat_duration {
                        let inv_offset = cheat_duration - offset;

                        if i + inv_offset < width && j + offset < height {
                            points_to_check.push((i + inv_offset, j + offset));
                        }
                        if i + offset < width && j >= inv_offset {
                            points_to_check.push((i + offset, j - inv_offset));
                        }
                        if i >= inv_offset && j >= offset {
                            points_to_check.push((i - inv_offset, j - offset));
                        }
                        if i >= offset && j + inv_offset < height {
                            points_to_check.push((i - offset, j + inv_offset));
                        }
                    }

                    for b in points_to_check {
                        let distance_b = path[b.1][b.0];
                        if distance_b == usize::MAX {
                            continue;
                        }

                        if distance_a < distance_b {
                            let normal_duration = distance_b - distance_a;

                            if cheat_duration < normal_duration {
                                result.push((a, b, normal_duration - cheat_duration));
                            }
                        }
                    }
                }
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

        assert_eq!(path[race.start.1][race.start.0], 0);
        assert_eq!(path[3][3], 6);
        assert_eq!(path[3][7], 14);
        assert_eq!(path[7][4], 83);
        assert_eq!(path[race.end.1][race.end.0], 84);
    }

    #[test]
    fn test_find_cheats() {
        let race = Race::from_lines(&example());
        let path = race.find_path();
        let cheats_part1 = race.find_cheats(&path, 2);

        assert_eq!(cheats_part1.len(), 44);
        assert_eq!(count_cheats(&cheats_part1, 2, 2), 14);
        assert_eq!(count_cheats(&cheats_part1, 4, 4), 14);
        assert_eq!(count_cheats(&cheats_part1, 6, 6), 2);
        assert_eq!(count_cheats(&cheats_part1, 8, 8), 4);
        assert_eq!(count_cheats(&cheats_part1, 10, 10), 2);
        assert_eq!(count_cheats(&cheats_part1, 12, 12), 3);
        assert_eq!(count_cheats(&cheats_part1, 20, 20), 1);
        assert_eq!(count_cheats(&cheats_part1, 36, 36), 1);
        assert_eq!(count_cheats(&cheats_part1, 38, 38), 1);
        assert_eq!(count_cheats(&cheats_part1, 40, 40), 1);
        assert_eq!(count_cheats(&cheats_part1, 64, 64), 1);

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
