use std::cmp::max;
use std::cmp::min;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day9.txt");

    let tiles = from_lines(data);
    let (a, b) = find_largest_rectangle(&tiles, &vec![]);
    let part1 = area(a, b);

    let forbidden = get_forbidden_rectangles(&tiles);
    let (a, b) = find_largest_rectangle(&tiles, &forbidden);
    let part2 = area(a, b);

    format!("{} {}", part1, part2)
}

fn from_lines(lines: Vec<String>) -> Vec<(i64, i64)> {
    lines
        .iter()
        .map(|line| {
            let coords: Vec<_> = line
                .split(',')
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect()
}

fn get_forbidden_rectangles(tiles: &Vec<(i64, i64)>) -> Vec<((i64, i64), (i64, i64))> {
    assert_unique_coordinates(tiles);
    assert_already_ordered(tiles);

    let top_left = tiles.iter().min().unwrap();
    let top_left_index = tiles.iter().position(|tile| *tile == *top_left).unwrap();

    let tiles = [&tiles[top_left_index..], &tiles[..top_left_index]].concat();
    assert_unique_coordinates(&tiles);
    assert_already_ordered(&tiles);

    let forbid_right = (tiles[0].0 == tiles[1].0 && tiles[0].1 < tiles[1].1)
        || (tiles[0].1 == tiles[1].1 && tiles[0].0 > tiles[1].0);

    let mut forbidden = Vec::new();

    let num_tiles = tiles.len();
    for i in 0..num_tiles {
        let next_i = if i == num_tiles - 1 { 0 } else { i + 1 };
        let a = tiles[i];
        let b = tiles[next_i];

        assert_ne!(
            a.0 != b.0,
            a.1 != b.1,
            "Invalid tile pair: {:?}, neither vertical or horizontal",
            (a, b)
        );

        if a.0 == b.0 {
            let x = a.0;
            if a.1 < b.1 {
                if forbid_right {
                    forbidden.push(((x - 1, a.1 + 1), (x - 1, b.1 - 1)));
                } else {
                    forbidden.push(((x + 1, a.1 + 1), (x + 1, b.1 - 1)));
                }
            } else {
                if forbid_right {
                    forbidden.push(((x + 1, b.1 + 1), (x + 1, a.1 - 1)));
                } else {
                    forbidden.push(((x - 1, b.1 + 1), (x - 1, a.1 - 1)));
                }
            }
        } else if a.1 == b.1 {
            let y = a.1;
            if a.0 < b.0 {
                if forbid_right {
                    forbidden.push(((a.0 + 1, y + 1), (b.0 - 1, y + 1)));
                } else {
                    forbidden.push(((a.0 + 1, y - 1), (b.0 - 1, y - 1)));
                }
            } else {
                if forbid_right {
                    forbidden.push(((b.0 + 1, y - 1), (a.0 - 1, y - 1)));
                } else {
                    forbidden.push(((b.0 + 1, y + 1), (a.0 - 1, y + 1)));
                }
            }
        } else {
            panic!("Invalid tile pair: {:?}", (a, b))
        };
    }

    forbidden
}

fn assert_already_ordered(tiles: &Vec<(i64, i64)>) {
    let num_tiles = tiles.len();
    let vertical = tiles[0].0 == tiles[1].0;
    for i in 0..num_tiles {
        let next_i = if i == num_tiles - 1 { 0 } else { i + 1 };
        if (i % 2 == 0) == vertical {
            assert_eq!(tiles[i].0, tiles[next_i].0);
        } else {
            assert_eq!(tiles[i].1, tiles[next_i].1);
        }
    }
}

fn assert_unique_coordinates(tiles: &Vec<(i64, i64)>) {
    let num_tiles = tiles.len();
    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = tiles.clone().into_iter().unzip();
    lefts.sort_unstable();
    rights.sort_unstable();
    lefts.dedup();
    rights.dedup();
    assert_eq!(lefts.len(), num_tiles / 2);
    assert_eq!(lefts.len(), rights.len());
}

fn find_largest_rectangle(
    tiles: &Vec<(i64, i64)>,
    forbidden: &Vec<((i64, i64), (i64, i64))>,
) -> ((i64, i64), (i64, i64)) {
    let mut best: Option<(i64, ((i64, i64), (i64, i64)))> = None;
    for t1 in tiles.iter() {
        for t2 in tiles.iter() {
            if t1 != t2 {
                if is_compatible((*t1, *t2), forbidden) {
                    let area = area(*t1, *t2);
                    if best.is_none() || area > best.unwrap().0 {
                        best = Some((area, (*t1, *t2)));
                    }
                }
            }
        }
    }

    let (a, b) = best.unwrap().1;

    if a.0 < b.0 || (a.0 == b.0 && a.1 < b.1) {
        (a, b)
    } else {
        (b, a)
    }
}

fn is_compatible(
    rect: ((i64, i64), (i64, i64)),
    forbidden: &Vec<((i64, i64), (i64, i64))>,
) -> bool {
    let norm_rect = (
        (min(rect.0.0, rect.1.0), min(rect.0.1, rect.1.1)),
        (max(rect.0.0, rect.1.0), max(rect.0.1, rect.1.1)),
    );

    !forbidden.iter().any(|f| overlaps(norm_rect, *f))
}

fn overlaps(a: ((i64, i64), (i64, i64)), b: ((i64, i64), (i64, i64))) -> bool {
    let ((ax1, ay1), (ax2, ay2)) = a;
    let ((bx1, by1), (bx2, by2)) = b;
    (ax1 <= bx2 && bx1 <= ax2) && (ay1 <= by2 && by1 <= ay2)
}

fn area((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "4739623064 1654141440");
    }

    #[test]
    fn test_find_largest_rectangle() {
        let tiles = from_lines(example());

        let (a, b) = find_largest_rectangle(&tiles, &vec![]);
        assert_eq!(a, (2, 5));
        assert_eq!(b, (11, 1));

        let forbidden = get_forbidden_rectangles(&tiles);
        let (a, b) = find_largest_rectangle(&tiles, &forbidden);
        assert_eq!(area(a, b), 24);
    }

    #[test]
    fn test_area() {
        assert_eq!(area((1, 1), (3, 3)), 9);
        assert_eq!(area((2, 5), (9, 7)), 24);
        assert_eq!(area((7, 1), (11, 7)), 35);
        assert_eq!(area((7, 3), (2, 3)), 6);
    }
    #[test]
    fn test_from_lines() {
        let tiles = from_lines(example());

        assert_eq!(tiles.len(), 8);
        assert_eq!(tiles[0], (7, 1));
        assert_eq!(tiles[7], (7, 3));
    }
    fn example() -> Vec<String> {
        vec![
            String::from("7,1"),
            String::from("11,1"),
            String::from("11,7"),
            String::from("9,7"),
            String::from("9,5"),
            String::from("2,5"),
            String::from("2,3"),
            String::from("7,3"),
        ]
    }
}
