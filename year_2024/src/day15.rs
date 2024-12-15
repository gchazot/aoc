pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day15.txt");
    let mut warehouse = Warehouse::from_lines(data);
    while warehouse.progress() {}

    let part1 = warehouse.checksum();
    let part2 = 456;

    format!("{} {}", part1, part2)
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Floor,
    Box,
}
struct Warehouse {
    size: usize,
    tiles: Vec<Vec<Tile>>,
    robot: (usize, usize),
    instructions: Vec<(isize, isize)>,
    current: usize,
}

impl Warehouse {
    fn from_lines(lines: Vec<String>) -> Warehouse {
        let size = lines[0].len();
        let mut tiles = vec![];

        let mut robot = (0, 0);

        let mut j = 0;
        loop {
            let line = &lines[j];
            if line.is_empty() {
                break;
            }

            assert_eq!(line.len(), size);

            let row = line
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Floor,
                    '@' => {
                        robot.0 = i;
                        robot.1 = j;
                        Tile::Floor
                    }
                    _ => unreachable!("Should not be here but {:?}", (j, i)),
                })
                .collect::<Vec<_>>();

            tiles.push(row);

            j += 1;
        }

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
        }
    }

    fn next_floor(&self, from: (usize, usize), direction: (isize, isize)) -> isize {
        let mut current = from;
        let mut result = 0;
        while current.0 > 0 && current.1 > 0 && current.0 < self.size && current.1 < self.size {
            result += 1;
            current.0 = (current.0 as isize + direction.0) as usize;
            current.1 = (current.1 as isize + direction.1) as usize;

            match self.tiles[current.1][current.0] {
                Tile::Box => {
                    continue;
                }
                Tile::Floor => {
                    return result;
                }
                Tile::Wall => {
                    return 0;
                }
            }
        }

        return 0;
    }

    fn step(&mut self, from: (usize, usize), direction: (isize, isize)) -> (usize, usize) {
        let distance = self.next_floor(from, direction);
        if distance == 0 {
            return from;
        }

        let result = (
            (from.0 as isize + direction.0) as usize,
            (from.1 as isize + direction.1) as usize,
        );
        if distance > 1 {
            self.tiles[(from.1 as isize + distance * direction.1) as usize]
                [(from.0 as isize + distance * direction.0) as usize] = Tile::Box;
            self.tiles[result.1][result.0] = Tile::Floor;
        }
        result
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
                    .filter_map(|(i, tile)| matches!(tile, Tile::Box).then_some(i + 100 * j))
                    .collect::<Vec<_>>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;
    use std::ops::IndexMut;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "1446158 456");
    }

    #[test]
    fn test_from_lines() {
        let warehouse = Warehouse::from_lines(_example());
        assert_eq!(warehouse.size, 10);
        assert_eq!(warehouse.instructions.len(), 700);
        assert_eq!(warehouse.robot, (4, 4));
    }

    #[test]
    fn test_next_floor() {
        let warehouse = Warehouse::from_lines(_example());
        assert_eq!(warehouse.next_floor((1, 1), (1, 0)), 1);
        assert_eq!(warehouse.next_floor((1, 1), (0, 1)), 1);
        assert_eq!(warehouse.next_floor((1, 1), (-1, 0)), 0);
        assert_eq!(warehouse.next_floor((1, 1), (0, -1)), 0);
        assert_eq!(warehouse.next_floor((3, 2), (0, -1)), 0);
        assert_eq!(warehouse.next_floor((3, 2), (0, 1)), 3);
        assert_eq!(warehouse.next_floor((4, 7), (-1, 0)), 3);
        assert_eq!(warehouse.next_floor((4, 7), (1, 0)), 2);
    }

    #[test]
    fn test_step() {
        let mut warehouse = Warehouse::from_lines(_example());
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
        let mut warehouse = Warehouse::from_lines(_example());
        while warehouse.progress() {}
        assert_eq!(warehouse.checksum(), 10092);
    }

    impl Tile {
        fn to_char(&self) -> char {
            match self {
                Tile::Floor => '.',
                Tile::Box => 'O',
                Tile::Wall => '#',
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
