use std::collections::{HashMap, HashSet, VecDeque};

pub fn execute() -> String {
    let mut mine = BrickYard::from_lines(aoc_utils::read_lines("input/day22.txt"));
    mine.drop();
    let disintegratable = mine.disintegratable_bricks();

    let chain_reactions = mine.chain_reactions();

    let part1 = disintegratable.len();
    let part2 = chain_reactions.values().sum::<usize>();

    format!("{} {}", part1, part2)
}

type Dimension = u16;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Coordinates {
    x: Dimension,
    y: Dimension,
    z: Dimension,
}

impl Coordinates {
    fn from_line(s: String) -> Coordinates {
        let coords: Vec<_> = s
            .split(",")
            .map(|c| str::parse::<Dimension>(c).unwrap())
            .collect();
        Coordinates {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

type BrickID = u32;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Brick {
    id: BrickID,
    start: Coordinates,
    end: Coordinates,
    blocks: Vec<Coordinates>,
}

impl Brick {
    fn new(id: BrickID, start: Coordinates, end: Coordinates) -> Brick {
        let mut blocks = Vec::new();
        for x in start.x..end.x + 1 {
            for y in start.y..end.y + 1 {
                for z in start.z..end.z + 1 {
                    blocks.push(Coordinates { x, y, z })
                }
            }
        }
        Brick {
            id,
            start,
            end,
            blocks,
        }
    }
    fn from_line(id: BrickID, s: &String) -> Brick {
        let (start_str, end_str) = s.split_once("~").unwrap();
        let start = Coordinates::from_line(start_str.to_string());
        let end = Coordinates::from_line(end_str.to_string());
        Brick::new(id, start, end)
    }
}

struct BrickYard {
    bricks: Vec<Brick>,
    blocks: HashSet<Coordinates>,
}
impl BrickYard {
    fn from_lines(lines: Vec<String>) -> BrickYard {
        let mut bricks = lines
            .iter()
            .enumerate()
            .map(|(id, line)| Brick::from_line(id as BrickID, line))
            .collect::<Vec<_>>();
        bricks.sort_by_key(|brick| brick.start.z);

        let mut blocks = HashSet::new();
        for brick in bricks.iter() {
            blocks.extend(brick.blocks.iter().cloned());
        }

        BrickYard { bricks, blocks }
    }

    fn drop(&mut self) {
        for brick in self.bricks.iter_mut() {
            let mut drop_height = 0;
            while brick.start.z > 1 + drop_height
                && brick
                    .blocks
                    .iter()
                    .map(|block| Coordinates {
                        x: block.x,
                        y: block.y,
                        z: brick.start.z - drop_height - 1,
                    })
                    .all(|c| !self.blocks.contains(&c))
            {
                drop_height += 1;
            }

            if drop_height > 0 {
                for block in brick.blocks.iter() {
                    self.blocks.remove(block);
                }
                brick.start.z -= drop_height;
                brick.end.z -= drop_height;
                for block in brick.blocks.iter_mut() {
                    block.z -= drop_height;
                    self.blocks.insert(block.clone());
                }
            }
        }
    }

    fn disintegratable_bricks(&self) -> HashSet<BrickID> {
        let mut cannot_be_disintegrated = HashSet::new();
        for brick in self.bricks.iter() {
            let mut supported_by = HashSet::new();
            for other_brick in self.bricks.iter() {
                if brick.id != other_brick.id {
                    for base in brick.blocks.iter().map(|block| Coordinates {
                        x: block.x,
                        y: block.y,
                        z: brick.start.z - 1,
                    }) {
                        if other_brick.blocks.contains(&base) {
                            supported_by.insert(other_brick.id);
                            break;
                        }
                    }
                }
            }
            if supported_by.len() == 1 {
                for &support in supported_by.iter() {
                    cannot_be_disintegrated.insert(support.clone());
                }
            }
        }

        let all_ids = self
            .bricks
            .iter()
            .map(|brick| brick.id)
            .collect::<HashSet<BrickID>>();
        let can_be_disintegrated = all_ids
            .difference(&cannot_be_disintegrated)
            .cloned()
            .collect();

        can_be_disintegrated
    }

    fn chain_reactions(&self) -> HashMap<BrickID, usize> {
        let mut supports = HashMap::<BrickID, Vec<BrickID>>::new();
        let mut supported_by = HashMap::<BrickID, HashSet<BrickID>>::new();

        for brick in self.bricks.iter() {
            supported_by.entry(brick.id).or_insert(HashSet::new());
            for other_brick in self.bricks.iter() {
                supports.entry(other_brick.id).or_insert(vec![]);
                if brick.id != other_brick.id {
                    for base in brick.blocks.iter().map(|block| Coordinates {
                        x: block.x,
                        y: block.y,
                        z: brick.start.z - 1,
                    }) {
                        if other_brick.blocks.contains(&base) {
                            supports.entry(other_brick.id).and_modify(|v| {
                                v.push(brick.id);
                            });
                            supported_by
                                .get_mut(&brick.id)
                                .unwrap()
                                .insert(other_brick.id);
                            break;
                        }
                    }
                }
            }
        }

        let mut reactions = HashMap::new();
        for brick in self.bricks.iter() {
            let mut still_supports = supports.clone();
            let mut still_supported_by = supported_by.clone();

            let mut to_remove = VecDeque::from([brick.id]);
            while to_remove.len() > 0 {
                let id = to_remove.pop_front().unwrap();
                let all_supported = still_supports.remove(&id);
                if all_supported.is_some() {
                    for supported in all_supported.unwrap() {
                        still_supported_by.get_mut(&supported).unwrap().remove(&id);
                        if still_supported_by.get(&supported).unwrap().is_empty() {
                            to_remove.push_back(supported);
                        }
                    }
                }
            }

            let mut still_there = Vec::new();
            for (id, supported) in still_supports {
                still_there.push(id);
                for id in supported {
                    still_there.push(id);
                }
            }

            let mut still_there: HashSet<BrickID> = HashSet::from_iter(still_there.into_iter());
            still_there.remove(&brick.id);

            let unsupported = self.bricks.len() - 1 - still_there.len();

            reactions.insert(brick.id, unsupported);
        }

        reactions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "389 70609");
    }

    fn _example() -> Vec<String> {
        vec![
            "1,0,1~1,2,1".to_string(),
            "0,0,2~2,0,2".to_string(),
            "0,2,3~2,2,3".to_string(),
            "0,0,4~0,2,4".to_string(),
            "2,0,5~2,2,5".to_string(),
            "0,1,6~2,1,6".to_string(),
            "1,1,8~1,1,9".to_string(),
        ]
    }

    #[test]
    fn test_brick_from_line() {
        let example1 = Brick::from_line(0, &String::from("1,0,1~1,2,1"));
        assert_eq!(0, example1.id);
        assert_eq!(1, example1.start.x);
        assert_eq!(0, example1.start.y);
        assert_eq!(1, example1.start.z);
        assert_eq!(1, example1.end.x);
        assert_eq!(2, example1.end.y);
        assert_eq!(1, example1.end.z);

        assert_eq!(3, example1.blocks.len());
        assert!(example1.blocks.contains(&example1.start));
        assert!(example1.blocks.contains(&example1.end));
        assert!(example1.blocks.contains(&Coordinates { x: 1, y: 1, z: 1 }));

        let example2 = Brick::from_line(1, &String::from("0,0,2~2,0,2"));
        assert_eq!(1, example2.id);
        assert_eq!(0, example2.start.x);
        assert_eq!(0, example2.start.y);
        assert_eq!(2, example2.start.z);
        assert_eq!(2, example2.end.x);
        assert_eq!(0, example2.end.y);
        assert_eq!(2, example2.end.z);

        assert_eq!(3, example2.blocks.len());
        assert!(example2.blocks.contains(&example2.start));
        assert!(example2.blocks.contains(&example2.end));
        assert!(example2.blocks.contains(&Coordinates { x: 1, y: 0, z: 2 }));

        let example2 = Brick::from_line(2, &String::from("1,1,8~1,1,9"));
        assert_eq!(2, example2.id);
        assert_eq!(1, example2.start.x);
        assert_eq!(1, example2.start.y);
        assert_eq!(8, example2.start.z);
        assert_eq!(1, example2.end.x);
        assert_eq!(1, example2.end.y);
        assert_eq!(9, example2.end.z);

        assert_eq!(2, example2.blocks.len());
        assert!(example2.blocks.contains(&example2.start));
        assert!(example2.blocks.contains(&example2.end));
    }

    #[test]
    fn test_from_lines() {
        let example = BrickYard::from_lines(_example());
        assert_eq!(7, example.bricks.len());
        assert_eq!(20, example.blocks.len());
        assert_eq!(Coordinates { x: 1, y: 0, z: 1 }, example.bricks[0].start);

        let unordered = BrickYard::from_lines(vec![
            "0,0,2~2,0,2".to_string(),
            "0,2,3~2,2,3".to_string(),
            "1,0,1~1,2,1".to_string(),
        ]);
        assert_eq!(2, unordered.bricks[0].id);
        assert_eq!(0, unordered.bricks[1].id);
        assert_eq!(1, unordered.bricks[2].id);
    }

    #[test]
    fn test_drop() {
        let mut example = BrickYard::from_lines(_example());
        example.drop();

        assert!(example
            .bricks
            .iter()
            .enumerate()
            .all(|(index, brick)| index as BrickID == brick.id));

        assert_eq!(Coordinates { x: 1, y: 0, z: 1 }, example.bricks[0].start);
        assert_eq!(Coordinates { x: 1, y: 2, z: 1 }, example.bricks[0].end);

        assert_eq!(Coordinates { x: 0, y: 0, z: 2 }, example.bricks[1].start);
        assert_eq!(Coordinates { x: 2, y: 0, z: 2 }, example.bricks[1].end);

        assert_eq!(Coordinates { x: 0, y: 2, z: 2 }, example.bricks[2].start);
        assert_eq!(Coordinates { x: 2, y: 2, z: 2 }, example.bricks[2].end);

        assert_eq!(Coordinates { x: 0, y: 0, z: 3 }, example.bricks[3].start);
        assert_eq!(Coordinates { x: 0, y: 2, z: 3 }, example.bricks[3].end);

        assert_eq!(Coordinates { x: 2, y: 0, z: 3 }, example.bricks[4].start);
        assert_eq!(Coordinates { x: 2, y: 2, z: 3 }, example.bricks[4].end);

        assert_eq!(Coordinates { x: 0, y: 1, z: 4 }, example.bricks[5].start);
        assert_eq!(Coordinates { x: 2, y: 1, z: 4 }, example.bricks[5].end);

        assert_eq!(Coordinates { x: 1, y: 1, z: 5 }, example.bricks[6].start);
        assert_eq!(Coordinates { x: 1, y: 1, z: 6 }, example.bricks[6].end);
    }

    #[test]
    fn test_destroy_one() {
        let mut example = BrickYard::from_lines(_example());
        example.drop();
        let destroyables = example.disintegratable_bricks();

        assert_eq!(5, destroyables.len());
    }

    #[test]
    fn test_chain_reactions() {
        let mut example = BrickYard::from_lines(_example());
        example.drop();

        let chain_reactions = example.chain_reactions();

        assert_eq!(7, chain_reactions.len());
        assert_eq!(7, chain_reactions.values().sum::<usize>());

        assert_eq!(6, chain_reactions[&0]);
        assert_eq!(0, chain_reactions[&1]);
        assert_eq!(0, chain_reactions[&2]);
        assert_eq!(0, chain_reactions[&3]);
        assert_eq!(0, chain_reactions[&4]);
        assert_eq!(1, chain_reactions[&5]);
        assert_eq!(0, chain_reactions[&6]);
    }
}
