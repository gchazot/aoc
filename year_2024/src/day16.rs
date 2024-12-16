use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day16.txt");

    let labirynth = Labyrinth::from_lines(data);

    let part1 = labirynth.shortest_route();
    let part2 = labirynth.best_seats();

    format!("{} {}", part1, part2)
}

struct Labyrinth {
    tiles: Vec<Vec<bool>>,
    size: Position,
    start: Position,
    end: Position,
}

impl Labyrinth {
    fn from_lines(lines: Vec<String>) -> Labyrinth {
        let width = lines[0].len();
        let mut start = Position(0, 0);
        let mut end = Position(0, 0);

        let tiles = lines
            .iter()
            .enumerate()
            .map(|(j, line)| {
                assert_eq!(line.len(), width);
                line.chars()
                    .enumerate()
                    .map(|(i, c)| match c {
                        'S' => {
                            start = Position::from_usize(i, j);
                            true
                        }
                        'E' => {
                            end = Position::from_usize(i, j);
                            true
                        }
                        '.' => true,
                        '#' => false,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<bool>>>();

        let size = Position::from_usize(width, tiles.len());

        Labyrinth {
            tiles,
            size,
            start,
            end,
        }
    }

    fn tile(&self, pos: &Position) -> bool {
        self.tiles[pos.1 as usize][pos.0 as usize]
    }

    fn shortest_route(&self) -> usize {
        let scores = self.dijkstra();
        scores
            .iter()
            .filter_map(|((pos, _direction), score)| (*pos == self.end).then_some(*score))
            .min()
            .expect("We should always reach the end")
    }

    fn best_seats(&self) -> usize {
        let scores = self.dijkstra();

        let best_score = scores
            .iter()
            .filter_map(|((pos, _direction), score)| (*pos == self.end).then_some(*score))
            .min()
            .expect("We should always reach the end");

        let best_ends = scores
            .iter()
            .filter_map(|((pos, direction), score)| {
                (*pos == self.end && *score == best_score).then_some((*pos, *direction, *score))
            })
            .collect::<Vec<(Position, Direction, usize)>>();

        let mut best_seats = HashSet::from([self.end]);
        let mut queue = VecDeque::from_iter(best_ends.iter().cloned());

        while let Some((pos, direction, score)) = queue.pop_front() {
            let next = pos - direction.delta();
            if scores
                .get(&(next, direction))
                .is_some_and(|&next_score| next_score + 1 == score)
            {
                queue.push_back((next.clone(), direction, score - 1));
                best_seats.insert(next);
            }

            if scores
                .get(&(pos, direction.rotate_left()))
                .is_some_and(|&next_score| next_score + 1000 == score)
            {
                queue.push_back((pos, direction.rotate_left(), score - 1000));
            }
            if scores
                .get(&(pos, direction.rotate_right()))
                .is_some_and(|&next_score| next_score + 1000 == score)
            {
                queue.push_back((pos, direction.rotate_right(), score - 1000));
            }
        }

        best_seats.len()
    }

    fn dijkstra(&self) -> HashMap<(Position, Direction), usize> {
        use Direction::*;

        let mut scores = HashMap::<(Position, Direction), usize>::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.start, East, 0usize));

        while let Some((current, direction, score)) = queue.pop_front() {
            let existing_score = scores.get(&(current, direction));
            if existing_score.is_some_and(|prev| *prev <= score) {
                continue;
            }
            scores.insert((current, direction), score);

            if current == self.end {
                continue;
            }

            let next = current + direction.delta();
            if self.tile(&next) {
                queue.push_back((next, direction, score + 1));
            }
            queue.push_back((current, direction.rotate_left(), score + 1000));
            queue.push_back((current, direction.rotate_right(), score + 1000));
        }
        scores
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(isize, isize);

impl Position {
    fn from_usize(x: usize, y: usize) -> Position {
        Position(x as isize, y as isize)
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    fn delta(&self) -> Position {
        use Direction::*;
        match self {
            East => Position(1, 0),
            West => Position(-1, 0),
            South => Position(0, 1),
            North => Position(0, -1),
        }
    }

    fn rotate_left(&self) -> Direction {
        use Direction::*;
        match self {
            East => North,
            West => South,
            South => East,
            North => West,
        }
    }
    fn rotate_right(&self) -> Direction {
        use Direction::*;
        match self {
            East => South,
            West => North,
            South => West,
            North => East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "102488 559");
    }

    #[test]
    fn test_from_lines() {
        let lab = Labyrinth::from_lines(_example());
        assert_eq!(lab.size, Position(15, 15));
        assert_eq!(lab.tiles.len(), 15);
        assert_eq!(lab.start, Position(1, 13));
        assert_eq!(lab.end, Position(13, 1));

        assert_eq!(lab.tiles[0][0], false);
        assert_eq!(lab.tiles[1][1], true);
        assert_eq!(lab.tiles[2][2], false);
        assert_eq!(lab.tiles[1][2], true);
        assert_eq!(lab.tiles[2][1], true);
    }

    #[test]
    fn test_shortest_route() {
        let lab = Labyrinth::from_lines(_example());
        assert_eq!(lab.shortest_route(), 7036);
    }

    #[test]
    fn test_best_seats() {
        let lab = Labyrinth::from_lines(_example());
        assert_eq!(lab.best_seats(), 45);
    }

    fn _example() -> Vec<String> {
        aoc_utils::read_lines("input/day16-example.txt")
    }
}
