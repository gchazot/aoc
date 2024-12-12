use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

pub fn execute() -> String {
    let data = read_to_string("input/day9.txt").unwrap();

    let disk = from_string(data);

    let compact1 = compact(&disk);
    let part1 = checksum(&compact1);

    let compact2 = compact2(&disk);
    let part2 = checksum(&compact2);

    format!("{} {}", part1, part2)
}

// (type, length)
// None is empty space, u32 is a file ID
#[derive(Clone, Eq, PartialEq, Debug)]
struct Block(Option<usize>, u32);

fn from_string(s: String) -> VecDeque<Block> {
    let mut result = VecDeque::new();

    for (i, c) in s.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if n > 0 {
            if i % 2 == 0 {
                result.push_back(Block(Some(i / 2), n));
            } else {
                result.push_back(Block(None, n));
            }
        }
    }

    result
}

fn compact(input: &VecDeque<Block>) -> VecDeque<Block> {
    assert!(matches!(input.front().unwrap().0, Some(_)));

    let mut source = input.clone();
    let mut result = VecDeque::new();

    let mut front = source.pop_front().unwrap();
    let mut back = source.pop_back().unwrap();

    while !source.is_empty() {
        while front.0.is_some() || front.1 == 0 {
            if front.1 > 0 {
                result.push_back(front);
            }
            front = source.pop_front().unwrap();
        }
        while back.0.is_none() || back.1 == 0 {
            back = source.pop_back().unwrap();
        }
        let to_move = u32::min(front.1, back.1);
        let new = Block(back.0, to_move);
        result.push_back(new);
        front.1 -= to_move;
        back.1 -= to_move;
    }
    if back.1 > 0 {
        let last = result.back_mut().unwrap();
        if back.0 == last.0 {
            last.1 += back.1;
        }
    }

    result
}

fn compact2(input: &VecDeque<Block>) -> VecDeque<Block> {
    let mut result = input.clone();
    let mut seen = HashSet::new();

    let mut j = result.len() - 1;
    while j > 0 {
        let block_right = &result[j];
        if block_right.0.is_none() || !seen.insert(block_right.0.unwrap()) {
            j -= 1;
            continue;
        }
        for i in 0..j {
            let block_left = &result[i];
            if block_left.0.is_none() && block_left.1 >= block_right.1 {
                let remaining = block_left.1 - block_right.1;
                result[i].1 = block_right.1;
                result.swap(i, j);
                if remaining > 0 {
                    result.insert(i + 1, Block(None, remaining));
                    j += 1
                }
                break;
            }
        }
        j -= 1;
    }

    let mut i = 0;
    while i < result.len() - 1 {
        while i + 1 < result.len() && result[i].0 == result[i + 1].0 {
            let removed = result.remove(i + 1);
            result[i].1 += removed.unwrap().1;
        }
        i += 1;
    }

    result
}

fn checksum(input: &VecDeque<Block>) -> usize {
    let mut result = 0;
    let mut i = 0;
    for block in input {
        if let Some(block_id) = block.0 {
            for _ in 0..block.1 {
                result += i * block_id;
                i += 1;
            }
        } else {
            i += block.1 as usize;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "6359213660505 6381624803796");
    }

    #[test]
    fn test_from_string() {
        let example = Vec::from_iter(from_string(_example()));
        assert_eq!(example.len(), 18);
        assert_eq!(example[0], Block(Some(0), 2));
        assert_eq!(example[1], Block(None, 3));
        assert_eq!(example[2], Block(Some(1), 3));
        assert_eq!(example[3], Block(None, 3));
        assert_eq!(example[4], Block(Some(2), 1));
        assert_eq!(example[5], Block(None, 3));
        assert_eq!(example[16], Block(Some(8), 4));
        assert_eq!(example[17], Block(Some(9), 2));
    }

    #[test]
    fn test_compact() {
        let example = from_string(_example());

        let compacted = Vec::from_iter(compact(&example));
        assert_eq!(compacted.len(), 13);
        assert_eq!(compacted[0], Block(Some(0), 2));
        assert_eq!(compacted[1], Block(Some(9), 2));
        assert_eq!(compacted[2], Block(Some(8), 1));
        assert_eq!(compacted[3], Block(Some(1), 3));
        assert_eq!(compacted[11], Block(Some(5), 4));
        assert_eq!(compacted[12], Block(Some(6), 2));
    }

    #[test]
    fn test_checksum() {
        let example = from_string(_example());

        let compacted = compact(&example);
        let checksum1 = checksum(&compacted);

        assert_eq!(1928, checksum1);

        let compacted2 = compact2(&example);
        let checksum2 = checksum(&compacted2);

        assert_eq!(2858, checksum2);
    }
    #[test]
    fn test_compact2() {
        let example = from_string(_example());

        let compacted = Vec::from_iter(compact2(&example));
        assert_eq!(compacted.len(), 16);
        assert_eq!(compacted[0], Block(Some(0), 2));
        assert_eq!(compacted[1], Block(Some(9), 2));
        assert_eq!(compacted[2], Block(Some(2), 1));
        assert_eq!(compacted[3], Block(Some(1), 3));
        assert_eq!(compacted[14], Block(Some(8), 4));
        assert_eq!(compacted[15], Block(None, 2));
    }

    fn _example() -> String {
        "2333133121414131402".to_string()
    }
}
