use std::collections::HashMap;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day8.txt");

    let mut boxes1 = JunctionBox::from_lines(&data);
    add_n_connections(&mut boxes1, 1000);
    let part1 = count_circuits(&boxes1);

    let mut boxes2 = JunctionBox::from_lines(&data);
    let (a, b) = find_closing_connection(&mut boxes2);
    let part2 = a.x * b.x;

    format!("{} {}", part1, part2)
}

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circuit: Option<u32>,
}

impl JunctionBox {
    fn from_lines(lines: &Vec<String>) -> Vec<Self> {
        lines.into_iter().map(Self::from_line).collect()
    }

    fn from_line(line: &String) -> Self {
        let coords: Vec<_> = line
            .split(',')
            .map(|coord| coord.parse().unwrap())
            .collect();
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            circuit: None,
        }
    }
}

fn count_circuits(junction_boxes: &Vec<JunctionBox>) -> usize {
    let n_circuits = 3;

    let mut counts = HashMap::new();
    for abox in junction_boxes {
        if abox.circuit.is_some() {
            counts
                .entry(abox.circuit.unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1usize);
        }
    }

    let mut largest = vec![0; n_circuits];

    for count in counts.values() {
        if *count > largest[0] {
            largest[0] = *count;
            largest.sort_unstable();
        }
    }

    largest.iter().product()
}

fn add_n_connections(junction_boxes: &mut Vec<JunctionBox>, n: usize) {
    let connections = shortest_distances(junction_boxes);

    let mut net_id = 0;
    let mut remaining = n;
    for (i, j) in connections {
        if junction_boxes[i].circuit.is_none() && junction_boxes[j].circuit.is_none() {
            net_id += 1;
            junction_boxes[i].circuit = Some(net_id);
            junction_boxes[j].circuit = Some(net_id);
        } else if junction_boxes[i].circuit.is_none() {
            junction_boxes[i].circuit = junction_boxes[j].circuit.clone();
        } else if junction_boxes[j].circuit.is_none() {
            junction_boxes[j].circuit = junction_boxes[i].circuit.clone();
        } else if junction_boxes[i].circuit != junction_boxes[j].circuit {
            let merged_circuit = junction_boxes[j].circuit;
            let remaining_circuit = junction_boxes[i].circuit;
            for abox in junction_boxes.iter_mut() {
                if abox.circuit == merged_circuit {
                    abox.circuit = remaining_circuit;
                }
            }
        }

        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
}

fn find_closing_connection(junction_boxes: &mut Vec<JunctionBox>) -> (JunctionBox, JunctionBox) {
    let connections = shortest_distances(junction_boxes);

    let mut net_id = 0;
    let mut unconnected = junction_boxes.len();
    for (i, j) in connections {
        if junction_boxes[i].circuit.is_none() && junction_boxes[j].circuit.is_none() {
            net_id += 1;
            junction_boxes[i].circuit = Some(net_id);
            junction_boxes[j].circuit = Some(net_id);
            unconnected -= 2;
        } else if junction_boxes[i].circuit.is_none() {
            junction_boxes[i].circuit = junction_boxes[j].circuit.clone();
            unconnected -= 1;
        } else if junction_boxes[j].circuit.is_none() {
            junction_boxes[j].circuit = junction_boxes[i].circuit.clone();
            unconnected -= 1;
        } else if junction_boxes[i].circuit != junction_boxes[j].circuit {
            let merged_circuit = junction_boxes[j].circuit;
            let remaining_circuit = junction_boxes[i].circuit;
            for abox in junction_boxes.iter_mut() {
                if abox.circuit == merged_circuit {
                    abox.circuit = remaining_circuit;
                }
            }
        } else {
            continue;
        }

        if unconnected == 0 {
            let net_id = junction_boxes.first().unwrap().circuit.unwrap();

            if junction_boxes
                .iter()
                .all(|abox| abox.circuit == Some(net_id))
            {
                return (junction_boxes[i], junction_boxes[j]);
            }
        }
    }

    panic!("No connection found");
}

fn shortest_distances(junction_boxes: &Vec<JunctionBox>) -> Vec<(usize, usize)> {
    let mut all_dists = vec![vec![-1; junction_boxes.len()]; junction_boxes.len()];
    for i in 0..junction_boxes.len() - 1 {
        let a = &junction_boxes[i];
        for j in i + 1..junction_boxes.len() {
            let b = &junction_boxes[j];
            let distance = (a.x - b.x).abs() * (a.x - b.x).abs()
                + (a.y - b.y).abs() * (a.y - b.y).abs()
                + (a.z - b.z).abs() * (a.z - b.z).abs();

            all_dists[i][j] = distance;
            all_dists[j][i] = distance;
        }
    }

    let mut all_closest = all_dists
        .iter()
        .enumerate()
        .flat_map(|(i, dists)| {
            dists
                .iter()
                .enumerate()
                .filter(|&(_j, dist)| *dist > 0)
                .map(move |(j, dist)| (*dist, (i, j)))
        })
        .collect::<Vec<_>>();
    all_closest.sort_unstable();
    all_closest.dedup_by(|a, b| a.1.0 == b.1.1 && a.1.1 == b.1.0);

    all_closest
        .iter()
        .map(|(_dist, pair)| {
            if pair.0 > pair.1 {
                (pair.1, pair.0)
            } else {
                *pair
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "117000 8368033065");
    }

    #[test]
    fn test_count_circuits() {
        let mut boxes = JunctionBox::from_lines(&example());
        add_n_connections(&mut boxes, 10);

        assert_eq!(count_circuits(&boxes), 40);
    }
    #[test]
    fn test_add_n_connections() {
        let mut boxes = JunctionBox::from_lines(&example());
        add_n_connections(&mut boxes, 10);

        assert_eq!(boxes[0].circuit, Some(1));
        assert_eq!(boxes[19].circuit, Some(1));

        assert_eq!(boxes[0].circuit, Some(1));
        assert_eq!(boxes[19].circuit, Some(1));
    }

    #[test]
    fn test_find_closing_connection() {
        let mut boxes = JunctionBox::from_lines(&example());
        let (a, b) = find_closing_connection(&mut boxes);
        assert_eq!(a.x, 216);
        assert_eq!(b.x, 117);
        assert_eq!(a.x * b.x, 25272);
    }

    #[test]
    fn test_shortest_distances() {
        let boxes = JunctionBox::from_lines(&example());

        let pairs = shortest_distances(&boxes);

        assert_eq!(boxes[pairs[0].0].x, 162);
        assert_eq!(boxes[pairs[0].0].y, 817);
        assert_eq!(boxes[pairs[0].0].z, 812);
        assert_eq!(boxes[pairs[0].1].x, 425);
        assert_eq!(boxes[pairs[0].1].y, 690);
        assert_eq!(boxes[pairs[0].1].z, 689);

        assert_eq!(boxes[pairs[1].0].x, 162);
        assert_eq!(boxes[pairs[1].0].y, 817);
        assert_eq!(boxes[pairs[1].0].z, 812);
        assert_eq!(boxes[pairs[1].1].x, 431);
        assert_eq!(boxes[pairs[1].1].y, 825);
        assert_eq!(boxes[pairs[1].1].z, 988);
    }

    #[test]
    fn test_junctionbox_from_lines() {
        let boxes = JunctionBox::from_lines(&example());

        assert_eq!(boxes.len(), 20);
        assert!(boxes.iter().all(|abox| abox.circuit.is_none()));

        assert_eq!(boxes[0].x, 162);
        assert_eq!(boxes[0].y, 817);
        assert_eq!(boxes[0].z, 812);
        assert_eq!(boxes[1].x, 57);
        assert_eq!(boxes[1].y, 618);
        assert_eq!(boxes[1].z, 57);
        assert_eq!(boxes[19].x, 425);
        assert_eq!(boxes[19].y, 690);
        assert_eq!(boxes[19].z, 689);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("162,817,812"),
            String::from("57,618,57"),
            String::from("906,360,560"),
            String::from("592,479,940"),
            String::from("352,342,300"),
            String::from("466,668,158"),
            String::from("542,29,236"),
            String::from("431,825,988"),
            String::from("739,650,466"),
            String::from("52,470,668"),
            String::from("216,146,977"),
            String::from("819,987,18"),
            String::from("117,168,530"),
            String::from("805,96,715"),
            String::from("346,949,466"),
            String::from("970,615,88"),
            String::from("941,993,340"),
            String::from("862,61,35"),
            String::from("984,92,344"),
            String::from("425,690,689"),
        ]
    }
}
