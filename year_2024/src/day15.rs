use std::collections::{HashSet, VecDeque};

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day15.txt");
    let mut warehouse = Warehouse::from_lines(data.clone(), false);
    while warehouse.progress() {}
    let part1 = warehouse.checksum();

    let mut warehouse2 = Warehouse::from_lines(data, true);
    while warehouse2.progress() {}
    let part2 = warehouse2.checksum();

    format!("{} {}", part1, part2)
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Floor,
    Box,
    BoxLeft,
    BoxRight,
}
struct Warehouse {
    size: (usize, usize),
    tiles: Vec<Vec<Tile>>,
    robot: (usize, usize),
    instructions: Vec<(isize, isize)>,
    current: usize,
    is_part2: bool,
}

impl Warehouse {
    fn from_lines(lines: Vec<String>, is_part2: bool) -> Warehouse {
        let input_width = lines[0].len();
        let mut tiles = vec![];

        let mut robot = (0, 0);

        let mut j = 0;
        loop {
            let line = &lines[j];
            if line.is_empty() {
                break;
            }

            assert_eq!(line.len(), input_width);

            let row_iter = line.chars().enumerate().map(|(i, c)| match c {
                '#' => Tile::Wall,
                '.' => Tile::Floor,
                'O' => Tile::Box,
                '@' => {
                    robot.0 = if is_part2 { i * 2 } else { i };
                    robot.1 = j;
                    Tile::Floor
                }
                _ => unreachable!("Should not be here but {:?}", (j, i)),
            });

            let row = if is_part2 {
                row_iter
                    .flat_map(|tile| match tile {
                        Tile::Box => vec![Tile::BoxLeft, Tile::BoxRight],
                        other => vec![other.clone(), other],
                    })
                    .collect::<Vec<_>>()
            } else {
                row_iter.collect::<Vec<_>>()
            };

            tiles.push(row);

            j += 1;
        }

        let width = if is_part2 {
            input_width * 2
        } else {
            input_width
        };
        let size = (width, tiles.len());

        let mut instructions = vec![];
        while j < lines.len() {
            let line = &lines[j];

            let instruction_block = line.chars().map(|c| match c {
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                _ => unreachable!(),
            });
            instructions.extend(instruction_block);

            j += 1;
        }

        Warehouse {
            size,
            tiles,
            robot,
            instructions,
            current: 0,
            is_part2,
        }
    }

    fn push(
        &self,
        from: (usize, usize),
        direction: (isize, isize),
    ) -> Option<HashSet<(usize, usize)>> {
        let mut result = HashSet::new();

        if !self.is_part2 {
            let mut next_floor_tile = 0;

            let mut current = from;
            while current.0 > 0
                && current.1 > 0
                && current.0 < self.size.0
                && current.1 < self.size.1
            {
                next_floor_tile += 1;
                current.0 = (current.0 as isize + direction.0) as usize;
                current.1 = (current.1 as isize + direction.1) as usize;

                match self.tiles[current.1][current.0] {
                    Tile::Box => {
                        continue;
                    }
                    Tile::Floor => {
                        break;
                    }
                    Tile::Wall => {
                        return None;
                    }
                    Tile::BoxLeft | Tile::BoxRight => {
                        panic!("This code does not apply to wide boxes");
                    }
                }
            }

            if next_floor_tile > 1 {
                for i in 1..next_floor_tile {
                    result.insert((
                        (from.0 as isize + i * direction.0) as usize,
                        (from.1 as isize + i * direction.1) as usize,
                    ));
                }
            }
        } else {
            let mut to_check = VecDeque::from([from]);
            while let Some(check) = to_check.pop_front() {
                let next = (
                    (check.0 as isize + direction.0) as usize,
                    (check.1 as isize + direction.1) as usize,
                );
                let is_horizontal = direction.1 == 0;
                match self.tiles[next.1][next.0] {
                    Tile::Box => {
                        panic!("This code does not apply to part 1");
                    }
                    Tile::Floor => {
                        continue;
                    }
                    Tile::Wall => {
                        return None;
                    }
                    Tile::BoxLeft | Tile::BoxRight if is_horizontal => {
                        result.insert(next);
                        to_check.push_back(next)
                    }
                    Tile::BoxRight if !is_horizontal => {
                        // RHS of a box => deal with it when dealing with LHS
                        let check_left = (check.0 - 1, check.1);
                        if !to_check.contains(&check_left) {
                            to_check.push_front(check_left);
                        }
                    }
                    Tile::BoxLeft if !is_horizontal => {
                        let next_right = (next.0 + 1, next.1);
                        result.insert(next);
                        result.insert(next_right);
                        to_check.push_back(next);
                        to_check.push_back(next_right);
                    }
                    _ => unreachable!("I think all cases are actually covered"),
                }
            }
        }

        Some(result)
    }

    fn step(&mut self, from: (usize, usize), direction: (isize, isize)) -> (usize, usize) {
        let mut new_position = from;

        let to_move = self.push(from, direction);

        if let Some(to_move) = to_move {
            let previous = to_move
                .iter()
                .map(|(i, j)| {
                    let prev_tile = self.tiles[*j][*i].clone();
                    self.tiles[*j][*i] = Tile::Floor;
                    (*i, *j, prev_tile)
                })
                .collect::<Vec<_>>();

            for (i, j, tile) in previous.into_iter() {
                self.tiles[(j as isize + direction.1) as usize]
                    [(i as isize + direction.0) as usize] = tile;
            }

            new_position = (
                (from.0 as isize + direction.0) as usize,
                (from.1 as isize + direction.1) as usize,
            );
        }

        new_position
    }

    fn progress(&mut self) -> bool {
        if self.current < self.instructions.len() {
            self.robot = self.step(self.robot, self.instructions[self.current]);
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn checksum(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(j, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, tile)| {
                        matches!(tile, Tile::Box | Tile::BoxLeft).then_some(i + 100 * j)
                    })
                    .collect::<Vec<_>>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1446158 1446175");
    }

    #[test]
    fn test_from_lines() {
        let warehouse = Warehouse::from_lines(_example(), false);
        assert_eq!(warehouse.size, (10, 10));
        assert_eq!(warehouse.instructions.len(), 700);
        assert_eq!(warehouse.robot, (4, 4));

        let warehouse2 = Warehouse::from_lines(_example(), true);
        assert_eq!(warehouse2.size, (20, 10));
        assert_eq!(warehouse2.instructions.len(), 700);
        assert_eq!(warehouse2.robot, (8, 4));
    }

    #[test]
    fn test_push() {
        let warehouse = Warehouse::from_lines(_example(), false);
        assert_eq!(warehouse.push((1, 1), (1, 0)), Some(HashSet::from([])));
        assert_eq!(warehouse.push((1, 1), (0, 1)), Some(HashSet::from([])));
        assert_eq!(warehouse.push((1, 1), (-1, 0)), None);
        assert_eq!(warehouse.push((1, 1), (0, -1)), None);
        assert_eq!(warehouse.push((3, 2), (0, -1)), None);
        assert_eq!(
            warehouse.push((3, 2), (0, 1)),
            Some(HashSet::from([(3, 3), (3, 4)]))
        );
        assert_eq!(
            warehouse.push((4, 7), (-1, 0)),
            Some(HashSet::from([(3, 7), (2, 7)]))
        );
        assert_eq!(
            warehouse.push((4, 7), (1, 0)),
            Some(HashSet::from([(5, 7)]))
        );
    }

    #[test]
    fn test_step() {
        let mut warehouse = Warehouse::from_lines(_example(), false);
        let state_0 = warehouse.tiles.clone();

        assert_eq!(warehouse.step((1, 1), (1, 0)), (2, 1));
        assert_eq!(warehouse.step((1, 1), (0, 1)), (1, 2));
        assert_eq!(warehouse.step((1, 1), (-1, 0)), (1, 1));
        assert_eq!(warehouse.step((1, 1), (0, -1)), (1, 1));
        assert_eq!(warehouse.step((3, 2), (0, -1)), (3, 2));
        assert_eq!(warehouse.tiles, state_0);

        assert!(matches!(warehouse.tiles[3][3], Tile::Box));
        assert!(matches!(warehouse.tiles[4][3], Tile::Box));
        assert!(matches!(warehouse.tiles[5][3], Tile::Floor));
        assert_eq!(warehouse.step((3, 2), (0, 1)), (3, 3));
        assert_ne!(warehouse.tiles, state_0);
        assert!(matches!(warehouse.tiles[3][3], Tile::Floor));
        assert!(matches!(warehouse.tiles[4][3], Tile::Box));
        assert!(matches!(warehouse.tiles[5][3], Tile::Box));

        assert!(matches!(warehouse.tiles[7][3], Tile::Box));
        assert!(matches!(warehouse.tiles[7][2], Tile::Box));
        assert!(matches!(warehouse.tiles[7][1], Tile::Floor));
        let state_1 = warehouse.tiles.clone();
        assert_eq!(warehouse.step((4, 7), (-1, 0)), (3, 7));
        assert_ne!(warehouse.tiles, state_1);
        assert!(matches!(warehouse.tiles[7][3], Tile::Floor));
        assert!(matches!(warehouse.tiles[7][2], Tile::Box));
        assert!(matches!(warehouse.tiles[7][1], Tile::Box));

        assert!(matches!(warehouse.tiles[7][5], Tile::Box));
        assert!(matches!(warehouse.tiles[7][6], Tile::Floor));
        assert!(matches!(warehouse.tiles[7][7], Tile::Box));
        let state_2 = warehouse.tiles.clone();
        assert_eq!(warehouse.step((4, 7), (1, 0)), (5, 7));
        assert_ne!(warehouse.tiles, state_2);
        assert!(matches!(warehouse.tiles[7][5], Tile::Floor));
        assert!(matches!(warehouse.tiles[7][6], Tile::Box));
        assert!(matches!(warehouse.tiles[7][7], Tile::Box));
    }

    #[test]
    fn test_progress() {
        let mut warehouse = Warehouse::from_lines(_example(), false);
        while warehouse.progress() {}
        assert_eq!(warehouse.checksum(), 10092);

        let mut warehouse2 = Warehouse::from_lines(_example(), true);
        while warehouse2.progress() {}
        assert_eq!(warehouse2.checksum(), 9021);
    }

    impl Tile {
        fn to_char(&self) -> char {
            match self {
                Tile::Floor => '.',
                Tile::Box => 'O',
                Tile::Wall => '#',
                Tile::BoxLeft => '[',
                Tile::BoxRight => ']',
            }
        }
    }
    impl Display for Warehouse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut rows = self
                .tiles
                .iter()
                .map(|row| row.iter().map(Tile::to_char).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            rows[self.robot.1][self.robot.0] = '@';

            write!(
                f,
                "{}",
                rows.iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }

    fn _example() -> Vec<String> {
        aoc_utils::read_lines("input/day15-example.txt")
    }
}
