pub fn execute() -> String {
    let data = aoc_utils::read_line("input/day2.txt");

    let part1 = get_all_invalids(&data, false).iter().sum::<usize>();
    let part2 = get_all_invalids(&data, true).iter().sum::<usize>();

    format!("{} {}", part1, part2)
}
type ProductID = usize;

fn get_all_invalids(data: &String, any_splits: bool) -> Vec<ProductID> {
    let ranges = parse_ranges(&data);
    ranges
        .iter()
        .flat_map(|range| get_invalid_ids_in_range(range, any_splits))
        .collect()
}

fn get_invalid_ids_in_range(range: &(ProductID, ProductID), any_splits: bool) -> Vec<ProductID> {
    let mut invalids = vec![];

    for n in range.0..range.1 + 1 {
        if !is_valid_id(n, any_splits) {
            invalids.push(n);
        }
    }

    invalids
}

fn is_valid_id(n: ProductID, any_splits: bool) -> bool {
    let digits = f32::log10(n as f32) as usize + 1;
    assert!(digits > 0);

    let max_splits = if any_splits { digits } else { 2 };

    for splits in 2..max_splits + 1 {
        if digits % splits == 0 {
            if is_invalid_id(n, splits) {
                return false;
            }
        }
    }
    return true;
}

fn is_invalid_id(n: ProductID, splits: usize) -> bool {
    let text = n.to_string().chars().collect::<Vec<char>>();
    let digits = text.len();

    for s in 1..splits {
        for i in 0..digits / splits {
            if text[i] != text[i + s * digits / splits] {
                return false;
            }
        }
    }
    true
}

fn parse_ranges(data: &String) -> Vec<(ProductID, ProductID)> {
    data.split(',')
        .map(|range| {
            let mut split = range.split('-');
            (
                split.next().unwrap().parse::<ProductID>().unwrap(),
                split.next().unwrap().parse::<ProductID>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = example();

        let all_invalids = get_all_invalids(&data, false);
        assert_eq!(all_invalids.iter().sum::<usize>(), 1227775554);

        let all_invalids_really = get_all_invalids(&data, true);
        assert_eq!(all_invalids_really.iter().sum::<usize>(), 4174379265);
    }
    #[test]
    fn test_mine() {
        assert_eq!(execute(), "41294979841 66500947346");
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges(&example());
        assert_eq!(ranges.len(), 11);
        for range in ranges {
            assert!(range.0 < range.1);
        }
    }

    fn example() -> String {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
    }
}
