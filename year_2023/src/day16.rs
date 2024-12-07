use std::collections::HashSet;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() -> String {
    let mine = Contraption::from_lines(aoc_utils::read_lines("input/day16.txt"));

    let part1 = mine.energized_count();

    let (_optimal_laser, optimal_count) = mine.optimize_energizing();
    let part2 = optimal_count;

    format!("{} {}", part1, part2)
}

enum Apparatus {
    None,
    SplitterHorizontal,
    SplitterVertical,
    MirrorTopLeftBottomRight,
    MirrorTopRightBottomLeft,
}

impl Apparatus {
    fn from_char(c: char) -> Apparatus {
        use Apparatus::*;
        match c {
            '.' => None,
            '-' => SplitterHorizontal,
            '|' => SplitterVertical,
            '\\' => MirrorTopLeftBottomRight,
            '/' => MirrorTopRightBottomLeft,
            _ => unreachable!(),
        }
    }
}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
}

type Laser = (usize, usize, Direction);

struct Contraption {
    tiles: Vec<Vec<Apparatus>>,
}

impl Contraption {
    fn from_lines(lines: Vec<String>) -> Contraption {
        let tiles = lines
            .iter()
            .map(|line| line.chars().map(Apparatus::from_char).collect())
            .collect();
        let mut lasers = HashSet::new();
        lasers.insert((0, 0, Direction::Right));
        Contraption { tiles }
    }

    fn optimize_energizing(&self) -> (Laser, usize) {
        use Direction::*;
        let height = self.tiles.len();
        let width = self.tiles[0].len();

        let top: Vec<Laser> = (0..width).map(|i| (i, 0, Down)).collect();
        let bottom: Vec<Laser> = (0..width).map(|i| (i, height - 1, Up)).collect();
        let left: Vec<Laser> = (0..height).map(|i| (0, i, Right)).collect();
        let right: Vec<Laser> = (0..height).map(|i| (width - 1, i, Left)).collect();

        let all = top
            .iter()
            .chain(bottom.iter())
            .chain(left.iter())
            .chain(right.iter());

        all.map(|laser| {
            let lasers = HashSet::from([laser.clone()]);
            let result = self.resolve(lasers);
            (laser.clone(), result)
        })
        .max_by_key(|(_laser, count)| *count)
        .unwrap()
    }

    fn energized_count(&self) -> usize {
        let lasers = HashSet::from([(0, 0, Direction::Right)]);
        self.resolve(lasers)
    }

    fn resolve(&self, lasers: HashSet<Laser>) -> usize {
        let mut seen = HashSet::new();

        let mut current_lasers = lasers.clone();
        while current_lasers.len() > 0 {
            let mut new_lasers = HashSet::new();
            for current_laser in current_lasers {
                if seen.insert(current_laser.clone()) {
                    for new_laser in self.progress(&current_laser) {
                        new_lasers.insert(new_laser);
                    }
                };
            }
            current_lasers = new_lasers;
        }
        seen.iter()
            .map(|laser| (laser.0, laser.1))
            .collect::<HashSet<_>>()
            .len()
    }

    fn progress(&self, laser: &Laser) -> Vec<Laser> {
        use Direction::*;
        let (x, y, direction) = laser;
        let new_lasers = match self.tiles[*y][*x] {
            Apparatus::None => vec![self.step(laser, direction.clone())],
            Apparatus::SplitterHorizontal => match direction {
                Down | Up => vec![self.step(laser, Left), self.step(laser, Right)],
                Right | Left => vec![self.step(laser, direction.clone())],
            },
            Apparatus::SplitterVertical => match direction {
                Down | Up => vec![self.step(laser, direction.clone())],
                Right | Left => vec![self.step(laser, Down), self.step(laser, Up)],
            },
            Apparatus::MirrorTopLeftBottomRight => match direction {
                Down => vec![self.step(laser, Right)],
                Up => vec![self.step(laser, Left)],
                Right => vec![self.step(laser, Down)],
                Left => vec![self.step(laser, Up)],
            },
            Apparatus::MirrorTopRightBottomLeft => match direction {
                Down => vec![self.step(laser, Left)],
                Up => vec![self.step(laser, Right)],
                Right => vec![self.step(laser, Up)],
                Left => vec![self.step(laser, Down)],
            },
        };

        new_lasers
            .iter()
            .filter_map(|laser| laser.clone())
            .collect()
    }

    fn step(&self, laser: &Laser, new_direction: Direction) -> Option<Laser> {
        use Direction::*;
        let max_y = self.tiles.len() - 1;
        let max_x = self.tiles[0].len() - 1;
        let new_pos = match new_direction {
            Down if laser.1 < max_y => (laser.0, laser.1 + 1),
            Up if laser.1 > 0 => (laser.0, laser.1 - 1),
            Right if laser.0 < max_x => (laser.0 + 1, laser.1),
            Left if laser.0 > 0 => (laser.0 - 1, laser.1),
            _ => return None,
        };

        Some((new_pos.0, new_pos.1, new_direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "7111 7831");
    }

    #[test]
    fn test_laser_progress() {
        use Direction::*;
        let contraption = Contraption::from_lines(vec![
            r".\..".to_string(),
            r"..|.".to_string(),
            r"/-/.".to_string(),
            r"\.-.".to_string(),
        ]);

        assert_eq!(contraption.progress(&(0, 0, Right)), vec![(1, 0, Right)]);
        assert_eq!(contraption.progress(&(1, 0, Right)), vec![(1, 1, Down)]);
        assert_eq!(contraption.progress(&(1, 1, Down)), vec![(1, 2, Down)]);
        assert_eq!(
            contraption.progress(&(1, 2, Down)),
            vec![(0, 2, Left), (2, 2, Right)]
        );
        assert_eq!(contraption.progress(&(0, 2, Left)), vec![(0, 3, Down)]);
        assert_eq!(contraption.progress(&(2, 2, Right)), vec![(2, 1, Up)]);
        assert_eq!(contraption.progress(&(0, 3, Down)), vec![(1, 3, Right)]);
        assert_eq!(contraption.progress(&(2, 1, Up)), vec![(2, 0, Up)]);
        assert_eq!(contraption.progress(&(1, 3, Right)), vec![(2, 3, Right)]);
        assert_eq!(contraption.progress(&(2, 0, Up)), vec![]);

        assert_eq!(contraption.progress(&(3, 1, Right)), vec![]);
        assert_eq!(contraption.progress(&(1, 3, Down)), vec![]);
        assert_eq!(contraption.progress(&(0, 1, Left)), vec![]);
        assert_eq!(contraption.progress(&(2, 0, Up)), vec![]);
    }

    #[test]
    fn test_example() {
        let example = Contraption::from_lines(vec![
            r".|...\....".to_string(),
            r"|.-.\.....".to_string(),
            r".....|-...".to_string(),
            r"........|.".to_string(),
            r"..........".to_string(),
            r".........\".to_string(),
            r"..../.\\..".to_string(),
            r".-.-/..|..".to_string(),
            r".|....-|.\".to_string(),
            r"..//.|....".to_string(),
        ]);
        assert_eq!(46, example.energized_count());

        assert_eq!(((3, 0, Direction::Down), 51), example.optimize_energizing());
    }
}
