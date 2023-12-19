use crate::utils;
use std::cmp::min;
use std::collections::HashSet;
use std::iter::zip;
use std::ops::Index;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let almanac = Almanac::from_text("mine.txt");
    assert_eq!(313045984, almanac.lowest_location_1());

    for mapping in &almanac.mappings {
        assert!(mapping.is_bijection());
    }

    assert_eq!(20283860, almanac.lowest_location_2());
}

#[test]
fn test_almanac_example() {
    let example = Almanac::from_text("example.txt");
    assert_eq!(7, example.mappings.len());
    assert_eq!(4, example.seeds.len());
    assert_eq!(2, example.mappings[0].entries.len());
    assert_eq!("seed", example.mappings[0].from);
    assert_eq!("soil", example.mappings[0].to);
    assert_eq!(3, example.mappings[1].entries.len());
    assert_eq!("soil", example.mappings[1].from);
    assert_eq!("fertilizer", example.mappings[1].to);

    assert_eq!(82, example.get_location(79));
    assert_eq!(43, example.get_location(14));
    assert_eq!(86, example.get_location(55));
    assert_eq!(35, example.get_location(13));

    assert_eq!(35, example.lowest_location_1());
    assert_eq!(46, example.lowest_location_2());

    for mapping in &example.mappings {
        assert!(mapping.is_bijection());
    }

    let mut sum_origin = 0;
    let mut sum_forward = 0;
    let mut sum_backward = 0;

    for i in 0..100 {
        sum_origin += i;
        sum_forward += example.get_location(i);
        sum_backward += example.get_seed(i);
    }
    assert_eq!(sum_origin, sum_forward);
    assert_eq!(sum_origin, sum_backward);
}

struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn from_text(filename: &str) -> Almanac {
        let path = format!("src/day5/{}", &filename);
        let data = utils::read_lines(&path);
        let mut blocks = data.split(|line| line.trim().is_empty());

        let seeds_block = blocks.next().unwrap();
        debug_assert_eq!(1, seeds_block.len());
        let seeds_line = seeds_block.index(0);
        let seeds_text = seeds_line.split_once(":").unwrap().1.trim();

        let mut seeds = Vec::new();
        for seed_text in seeds_text.split_whitespace() {
            let seed = seed_text.parse::<usize>().unwrap();
            seeds.push(seed);
        }

        let mut mappings = Vec::new();
        for block in blocks {
            let mapping = Mapping::from_text(block);
            mappings.push(mapping);
        }
        return Almanac { seeds, mappings };
    }

    fn lowest_location_1(&self) -> usize {
        return self
            .seeds
            .iter()
            .map(|&seed| self.get_location(seed))
            .reduce(|acc, location| min(acc, location))
            .unwrap();
    }

    fn lowest_location_2(&self) -> usize {
        let mut edges = HashSet::new();

        for mapping in &self.mappings {
            edges.extend(mapping.src_edges());
            edges = HashSet::from_iter(edges.iter().map(|&edge| mapping.forward(edge)));
        }

        let &min_location = edges
            .iter()
            .filter(|&&loc| self.is_location_for_seed(loc))
            .min()
            .unwrap();
        return min_location;
    }

    fn get_location(&self, seed: usize) -> usize {
        let mut value = seed;
        // This makes the assumption that mappings are correctly ordered
        for mapping in &self.mappings {
            value = mapping.forward(value);
        }
        return value;
    }

    fn get_seed(&self, location: usize) -> usize {
        let mut value = location;
        // This makes the assumption that mappings are correctly ordered
        for mapping in self.mappings.iter().rev() {
            value = mapping.backward(value);
        }
        return value;
    }

    fn is_location_for_seed(&self, location: usize) -> bool {
        return self.is_seed(self.get_seed(location));
    }

    fn is_seed(&self, seed: usize) -> bool {
        let mut index = 0;
        while index + 1 < self.seeds.len() {
            let start = self.seeds[index];
            let width = self.seeds[index + 1];
            let end = start + width - 1;

            if start <= seed && seed <= end {
                return true;
            }
            index += 2;
        }
        return false;
    }
}

#[test]
fn test_mapping() {
    let mut text = Vec::new();
    text.push("seed-to-soil map:");
    text.push("50 98 2");
    text.push("52 50 48");

    let mapping = Mapping::from_text(&text);
    assert_eq!("seed", mapping.from);
    assert_eq!("soil", mapping.to);
    assert_eq!(2, mapping.entries.len());

    for i in 0..50 - 1 {
        assert_eq!(i, mapping.forward(i));
    }
    for i in 50..50 + 48 {
        assert_eq!(i - 50 + 52, mapping.forward(i));
    }
    for i in 98..98 + 2 {
        assert_eq!(i - 98 + 50, mapping.forward(i));
    }
    for i in 100..200 {
        assert_eq!(i, mapping.forward(i));
    }
}

struct Mapping {
    from: String,
    to: String,
    entries: Vec<MappingEntry>,
}

impl Mapping {
    fn from_text(text: &[impl AsRef<str>]) -> Mapping {
        let mut lines = text.into_iter();

        let name_line = lines.next().unwrap();
        let names = name_line.as_ref().split_once(" ").unwrap().0;
        let (from_str, to_str) = names.split_once("-to-").unwrap();
        let from = String::from(from_str);
        let to = String::from(to_str);

        let mut entries = Vec::new();
        for line in lines {
            entries.push(MappingEntry::from_text(line.as_ref()));
        }

        return Mapping { from, to, entries };
    }

    fn src_edges(&self) -> HashSet<usize> {
        let mut result = HashSet::new();
        for entry in &self.entries {
            result.insert(entry.src);
            result.insert(entry.src + entry.width - 1);
        }
        return result;
    }

    fn forward(&self, origin: usize) -> usize {
        for entry in &self.entries {
            let mapped = entry.forward(origin);
            if mapped.is_some() {
                return mapped.unwrap();
            }
        }
        return origin;
    }
    fn backward(&self, origin: usize) -> usize {
        for entry in &self.entries {
            let mapped = entry.backward(origin);
            if mapped.is_some() {
                return mapped.unwrap();
            }
        }
        return origin;
    }

    fn is_bijection(&self) -> bool {
        let sources = Vec::from_iter(self.entries.iter().map(|entry| (entry.src, entry.width)));
        let sources_compressed = compress(sources);

        let dests = Vec::from_iter(self.entries.iter().map(|entry| (entry.dst, entry.width)));
        let dests_compressed = compress(dests);

        for pair in zip(sources_compressed.iter(), dests_compressed.iter()) {
            if pair.0 != pair.1 {
                return false;
            }
        }

        return true;
    }
}

#[test]
fn test_compress() {
    type Range = Vec<(usize, usize)>;

    assert_eq!(Range::from([]), compress(Range::from([])));
    assert_eq!(Range::from([(0, 1)]), compress(Range::from([(0, 1)])));
    assert_eq!(
        Range::from([(0, 5)]),
        compress(Range::from([(0, 2), (2, 3)]))
    );
    assert_eq!(
        Range::from([(0, 2), (3, 3)]),
        compress(Range::from([(0, 2), (3, 3)]))
    );
    assert_eq!(
        Range::from([(0, 5)]),
        compress(Range::from([(0, 2), (2, 1), (3, 2)]))
    );
    assert_eq!(
        Range::from([(0, 5)]),
        compress(Range::from([(0, 2), (3, 2), (2, 1)]))
    );
    assert_eq!(
        Range::from([(0, 5)]),
        compress(Range::from([(3, 2), (0, 2), (2, 1)]))
    );
}

fn compress(entries: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if entries.len() < 2 {
        return entries;
    }

    let mut sorted = entries.clone();
    sorted.sort();

    let mut sorted_iter = sorted.iter().peekable();
    let (mut start, mut width) = sorted_iter.next().unwrap();

    let mut result = Vec::new();
    while let Some(entry) = sorted_iter.next() {
        if entry.0 == start + width {
            width += entry.1;
        } else {
            result.push((start, width));
            start = entry.0;
            width = entry.1;
        }
        if sorted_iter.peek().is_none() {
            result.push((start, width));
        }
    }

    return result;
}

#[test]
fn test_mapping_entry() {
    let entry = MappingEntry::from_text("0 15 37");
    assert_eq!(0usize, entry.dst);
    assert_eq!(15usize, entry.src);
    assert_eq!(37usize, entry.width);

    for i in 0..15 - 1 {
        assert_eq!(None, entry.forward(i));
    }
    for i in 15..15 + 37 {
        assert_eq!(Some(i - 15 + 0), entry.forward(i));
    }
    for i in 15 + 37..100 {
        assert_eq!(None, entry.forward(i));
    }

    for i in 0..37 - 1 {
        assert_eq!(Some(i + 15 - 0), entry.backward(i));
    }
    for i in 37..100 {
        assert_eq!(None, entry.backward(i));
    }
}

struct MappingEntry {
    src: usize,
    dst: usize,
    width: usize,
}

impl MappingEntry {
    fn from_text(text: &str) -> MappingEntry {
        let mut parts = text.splitn(3, " ");
        let dst = parts.next().unwrap().parse::<usize>().unwrap();
        let src = parts.next().unwrap().parse::<usize>().unwrap();
        let width = parts.next().unwrap().parse::<usize>().unwrap();
        return MappingEntry { src, dst, width };
    }

    fn forward(&self, origin: usize) -> Option<usize> {
        let end_src = self.src + self.width - 1;
        if self.src <= origin && origin <= end_src {
            return Some(origin - self.src + self.dst);
        }
        return None;
    }

    fn backward(&self, origin: usize) -> Option<usize> {
        let end_dst = self.dst + self.width - 1;
        if self.dst <= origin && origin <= end_dst {
            return Some(origin - self.dst + self.src);
        }

        return None;
    }
}
