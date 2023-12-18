use std::cmp::min;
use std::ops::Index;
use crate::utils;

pub fn execute() {
    let almanac = Almanac::from_text("mine.txt");
    assert_eq!(313045984, almanac.lowest_location_1());
    // Too slow for now
    // assert_eq!(313045984, almanac.lowest_location_2());
}

#[test]
fn test_almanac_example () {
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
}

struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn from_text(filename: &str) -> Almanac {
        let path = format!("src/day5/{}", &filename);
        let data = utils::read_lines(&path);
        let mut blocks = data.split(|line|line.trim().is_empty());

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
        return Almanac {seeds, mappings};
    }

    fn lowest_location_1(&self) -> usize {
        return self.seeds.iter()
            .map(|&seed| self.get_location(seed))
            .reduce(|acc, location|min(acc, location))
            .unwrap();
    }

    fn lowest_location_2(&self) -> usize {
        let seeds = self.seed_iter();
        return seeds
            .map(|seed| self.get_location(seed))
            .reduce(|acc, location|min(acc, location))
            .unwrap();
    }

    fn seed_iter(&self) -> SeedIterator {
        return SeedIterator {
            seeds: &self.seeds,
            seed_index: 0,
            progress: 0,
        };
    }

    fn get_location(&self, seed: usize) -> usize {
        let mut value = seed;
        // This makes the assumption that mappings are correctly ordered
        for mapping in &self.mappings {
            value = mapping.forward(value);
        }
        return value;
    }
}

struct SeedIterator<'a> {
    seeds:&'a Vec<usize>,
    seed_index: usize,
    progress: usize,
}

impl<'a> Iterator for SeedIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seed_index + 1 >= self.seeds.len() {
            return None;
        }

        let seed_range = self.seeds[self.seed_index+1];
        if self.progress >= seed_range {
            self.progress = 0;
            self.seed_index += 2;

            if self.seed_index >= self.seeds.len() {
                return None;
            }
        }
        let result = self.seeds[self.seed_index] + self.progress;
        self.progress += 1;
        return Some(result);
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

    for i in 0..50-1 {
        assert_eq!(i, mapping.forward(i));
    }
    for i in 50..50+48 {
        assert_eq!(i-50+52, mapping.forward(i));
    }
    for i in 98..98+2 {
        assert_eq!(i-98+50, mapping.forward(i));
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
        };

        return Mapping {from, to, entries};
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
}

#[test]
fn test_mapping_entry() {
    let entry = MappingEntry::from_text("0 15 37");
    assert_eq!(0usize, entry.dst);
    assert_eq!(15usize, entry.src);
    assert_eq!(37usize, entry.width);

    for i in 0..15-1 {
        assert_eq!(None, entry.forward(i));
    }
    for i in 15..15+37 {
        assert_eq!(Some(i - 15 + 0), entry.forward(i));
    }
    for i in 15+37..100 {
        assert_eq!(None, entry.forward(i));
    }
}

struct MappingEntry {
    src: usize,
    dst: usize,
    width: usize,
}

impl MappingEntry {
    fn from_text(text: &str) -> MappingEntry{
        let mut parts = text.splitn(3, " ");
        let dst= parts.next().unwrap().parse::<usize>().unwrap();
        let src= parts.next().unwrap().parse::<usize>().unwrap();
        let width= parts.next().unwrap().parse::<usize>().unwrap();
        return MappingEntry {src, dst, width};
    }

    fn forward(&self, origin: usize) -> Option<usize> {
        let end_src = self.src + self.width - 1;
        if self.src <= origin && origin <= end_src {
            return Some(origin - self.src + self.dst);
        }
        return None;
    }
}
