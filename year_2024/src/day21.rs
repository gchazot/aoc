use std::collections::HashMap;
use RemoteKey::*;

pub fn execute() -> String {
    let data = aoc_utils::read_lines("input/day21.txt");

    let part1: usize = data.iter().map(|code| score(code, 2)).sum();
    let part2: usize = data.iter().map(|code| score(code, 25)).sum();

    format!("{} {}", part1, part2)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RemoteKey {
    Left,  // 0
    Down,  // 1
    Right, // 2
    Up,    // 3
    A,     // 4
}

impl RemoteKey {
    fn index(&self) -> usize {
        match self {
            Left => 0,
            Down => 1,
            Right => 2,
            Up => 3,
            A => 4,
        }
    }
    fn invert(&self) -> Self {
        match self {
            Left => Right,
            Down => Up,
            Right => Left,
            Up => Down,
            A => panic!("Cannot invert"),
        }
    }
}

type NumpadKey = u8;
const NUMPAD_A: NumpadKey = 10;

fn code_from_line(line: &String) -> Vec<NumpadKey> {
    line.chars()
        .map(|c| c.to_digit(11).unwrap() as NumpadKey)
        .collect()
}

fn numpad_transitions(a: NumpadKey, b: NumpadKey) -> Vec<Vec<RemoteKey>> {
    if a <= b {
        let options = NUMPAD_PATHS[a as usize][(b - a) as usize];
        options.iter().map(|&option| option.to_vec()).collect()
    } else {
        let options = NUMPAD_PATHS[b as usize][(a - b) as usize];
        options
            .iter()
            .map(|&option| option.iter().map(|key| key.invert()).rev().collect())
            .collect()
    }
}

fn numpad_sequences(code: &Vec<NumpadKey>) -> Vec<Vec<RemoteKey>> {
    let transitions_options = code.iter().enumerate().map(|(i, next)| {
        let key = if i > 0 { code[i - 1] } else { NUMPAD_A };
        numpad_transitions(key, *next)
    });

    let mut result = vec![vec![]];
    for transition_options in transitions_options.into_iter() {
        result = result
            .iter()
            .flat_map(|sequence| {
                transition_options.iter().map(|subsequence| {
                    let mut new_sequence = sequence.clone();
                    new_sequence.extend(subsequence);
                    new_sequence.push(A);
                    new_sequence
                })
            })
            .collect();
    }
    result
}

fn score(line: &String, depth: u8) -> usize {
    let code = code_from_line(line);

    let sequences = numpad_sequences(&code);
    let shortest = sequences
        .iter()
        .map(|sequence| shortest_sequence(sequence, depth))
        .min()
        .unwrap();

    let code_num: usize = line[..line.len() - 1].parse().unwrap();

    code_num * shortest
}

fn shortest_sequence(sequence: &Vec<RemoteKey>, depth: u8) -> usize {
    let mut seen_transition_lengths = HashMap::new();

    shortest_sequence_cached(sequence, depth, &mut seen_transition_lengths)
}

fn shortest_sequence_cached(
    sequence: &[RemoteKey],
    depth: u8,
    seen_transition_length: &mut HashMap<(RemoteKey, RemoteKey, u8), usize>,
) -> usize {
    if depth == 0 {
        return sequence.len();
    }

    let mut total = 0;
    let mut key = A;
    for next in sequence {
        let seen_key = (key.clone(), next.clone(), depth);
        if !seen_transition_length.contains_key(&seen_key) {
            let from_index = key.index();
            let to_index = next.index();
            let shortest = REMOTE_PATHS[from_index][to_index]
                .iter()
                .map(|&transition| {
                    let mut transition = transition.to_vec();
                    transition.push(A);
                    shortest_sequence_cached(
                        transition.as_slice(),
                        depth - 1,
                        seen_transition_length,
                    )
                })
                .min()
                .unwrap();
            seen_transition_length.insert(seen_key, shortest);
        }
        total += seen_transition_length.get(&seen_key).unwrap();
        key = *next;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Display, Formatter};

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "163086 198466286401228");
    }

    #[test]
    fn test_score() {
        assert_eq!(
            example().iter().map(|code| score(code, 2)).sum::<usize>(),
            126384
        );

        assert_eq!(
            example().iter().map(|code| score(code, 25)).sum::<usize>(),
            154115708116294
        );
    }

    #[test]
    fn test_numpad_sequences() {
        assert_eq!(numpad_sequences(&vec![NUMPAD_A]), vec![vec![A]]);
        assert_eq!(numpad_sequences(&vec![3]), vec![vec![Up, A]]);
        assert_eq!(numpad_sequences(&vec![0]), vec![vec![Left, A]]);

        assert_eq!(numpad_sequences(&vec![3, 6]), vec![vec![Up, A, Up, A]]);
        assert_eq!(numpad_sequences(&vec![3, 2]), vec![vec![Up, A, Left, A]]);

        assert!(is_permutation(
            numpad_sequences(&vec![3, 5]),
            vec![vec![Up, A, Left, Up, A], vec![Up, A, Up, Left, A]],
        ));

        assert!(is_permutation(
            numpad_sequences(&vec![5, 3]),
            vec![
                vec![Up, Up, Left, A, Down, Right, A],
                vec![Up, Up, Left, A, Right, Down, A],
                vec![Up, Left, Up, A, Down, Right, A],
                vec![Up, Left, Up, A, Right, Down, A],
                vec![Left, Up, Up, A, Down, Right, A],
                vec![Left, Up, Up, A, Right, Down, A],
            ]
        ));
    }

    #[test]
    fn test_numpad_transitions() {
        for i in 0..NUMPAD_A + 1 {
            assert_eq!(numpad_transitions(i, i), vec![vec![]], "{}", i);
        }
        assert_eq!(numpad_transitions(0, NUMPAD_A), vec![vec![Right]]);
        assert_eq!(numpad_transitions(NUMPAD_A, 0), vec![vec![Left]]);
        assert_eq!(numpad_transitions(0, 2), vec![vec![Up]]);
        assert_eq!(numpad_transitions(2, 0), vec![vec![Down]]);
        assert_eq!(numpad_transitions(0, 1), vec![vec![Up, Left]]);
        assert_eq!(numpad_transitions(1, 0), vec![vec![Right, Down]]);

        assert_eq!(
            numpad_transitions(0, 3),
            vec![vec![Up, Right], vec![Right, Up]]
        );
        assert_eq!(
            numpad_transitions(3, 0),
            vec![vec![Left, Down], vec![Down, Left]]
        );
    }

    #[test]
    fn test_codes_from_lines() {
        let lines = &example();
        let codes = lines.iter().map(code_from_line).collect::<Vec<_>>();

        assert_eq!(codes.len(), 5);

        assert_eq!(codes[0], vec![0, 2, 9, NUMPAD_A]);
        assert_eq!(codes[1], vec![9, 8, 0, NUMPAD_A]);
        assert_eq!(codes[2], vec![1, 7, 9, NUMPAD_A]);
        assert_eq!(codes[3], vec![4, 5, 6, NUMPAD_A]);
        assert_eq!(codes[4], vec![3, 7, 9, NUMPAD_A]);
    }

    fn example() -> Vec<String> {
        vec![
            String::from("029A"),
            String::from("980A"),
            String::from("179A"),
            String::from("456A"),
            String::from("379A"),
        ]
    }

    fn is_permutation<T: IntoIterator + PartialEq>(a: Vec<T>, b: Vec<T>) -> bool {
        a.iter().len() == b.iter().len() && a.iter().all(|x| b.iter().any(|y| *x == *y))
    }

    impl RemoteKey {
        fn to_char(&self) -> char {
            match self {
                Left => '<',
                Down => 'v',
                Right => '>',
                Up => '^',
                A => 'A',
            }
        }
    }
    impl Display for RemoteKey {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_char())
        }
    }
}

static REMOTE_PATHS: &'static [&[&[&[RemoteKey]]]] = &[
    &[
        // Left
        &[
            // Left-Left
            &[],
        ],
        &[
            // Left-Down
            &[Right],
        ],
        &[
            // Left-Right
            &[Right, Right],
        ],
        &[
            // Left-Up
            &[Right, Up],
        ],
        &[
            // Left-A
            &[Right, Right, Up],
            &[Right, Up, Right],
        ],
    ],
    &[
        // Down
        &[
            // Down-Left
            &[Left],
        ],
        &[
            // Down-Down
            &[],
        ],
        &[
            // Down-Right
            &[Right],
        ],
        &[
            // Down-Up
            &[Up],
        ],
        &[
            // Down-A
            &[Right, Up],
            &[Up, Right],
        ],
    ],
    &[
        // Right
        &[
            // Right-Left
            &[Left, Left],
        ],
        &[
            // Right-Down
            &[Left],
        ],
        &[
            // Right-Right
            &[],
        ],
        &[
            // Right-Up
            &[Left, Up],
            &[Up, Left],
        ],
        &[
            // Right-A
            &[Up],
        ],
    ],
    &[
        // Up
        &[
            // Up-Left
            &[Down, Left],
        ],
        &[
            // Up-Down
            &[Down],
        ],
        &[
            // Up-Right
            &[Down, Right],
            &[Right, Down],
        ],
        &[
            // Up-Up
            &[],
        ],
        &[
            // Up-A
            &[Right],
        ],
    ],
    &[
        // A
        &[
            // A-Left
            &[Down, Left, Left],
            &[Left, Down, Left],
        ],
        &[
            // A-Down
            &[Down, Left],
            &[Left, Down],
        ],
        &[
            // A-Right
            &[Down],
        ],
        &[
            // A-Up
            &[Left],
        ],
        &[
            // A-A
            &[],
        ],
    ],
];

static NUMPAD_PATHS: &'static [&[&[&[RemoteKey]]]] = &[
    &[
        // 0
        &[
            // 0-0
            &[],
        ],
        &[
            // 0-1
            &[Up, Left],
        ],
        &[
            // 0-2
            &[Up],
        ],
        &[
            // 0-3
            &[Up, Right],
            &[Right, Up],
        ],
        &[
            // 0-4
            &[Up, Up, Left],
            &[Up, Left, Up],
        ],
        &[
            // 0-5
            &[Up, Up],
        ],
        &[
            // 0-6
            &[Up, Up, Right],
            &[Up, Right, Up],
            &[Right, Up, Up],
        ],
        &[
            // 0-7
            &[Up, Up, Up, Left],
            &[Up, Up, Left, Up],
            &[Up, Left, Up, Up],
        ],
        &[
            // 0-8
            &[Up, Up, Up],
        ],
        &[
            // 0-9
            &[Up, Up, Up, Right],
            &[Up, Up, Right, Up],
            &[Up, Right, Up, Up],
            &[Right, Up, Up, Up],
        ],
        &[
            // 0-A
            &[Right],
        ],
    ],
    &[
        // 1
        &[
            // 1-1
            &[],
        ],
        &[
            // 1-2
            &[Right],
        ],
        &[
            // 1-3
            &[Right, Right],
        ],
        &[
            // 1-4
            &[Up],
        ],
        &[
            // 1-5
            &[Up, Right],
            &[Right, Up],
        ],
        &[
            // 1-6
            &[Up, Right, Right],
            &[Right, Up, Right],
            &[Right, Right, Up],
        ],
        &[
            // 1-7
            &[Up, Up],
        ],
        &[
            // 1-8
            &[Up, Up, Right],
            &[Up, Right, Up],
            &[Right, Up, Up],
        ],
        &[
            // 1-9
            &[Up, Up, Right, Right],
            &[Up, Right, Up, Right],
            &[Up, Right, Right, Up],
            &[Right, Up, Up, Right],
            &[Right, Up, Right, Up],
            &[Right, Right, Up, Up],
        ],
        &[
            // 1-A
            &[Right, Right, Down],
            &[Right, Down, Right],
        ],
    ],
    &[
        // 2
        &[
            // 2-2
            &[],
        ],
        &[
            // 2-3
            &[Right],
        ],
        &[
            // 2-4
            &[Up, Left],
            &[Left, Up],
        ],
        &[
            // 2-5
            &[Up],
        ],
        &[
            // 2-6
            &[Up, Right],
            &[Right, Up],
        ],
        &[
            // 2-7
            &[Up, Up, Left],
            &[Up, Left, Up],
            &[Left, Up, Up],
        ],
        &[
            // 2-8
            &[Up, Up],
        ],
        &[
            // 2-9
            &[Up, Up, Right],
            &[Up, Right, Up],
            &[Right, Up, Up],
        ],
        &[
            // 2-A
            &[Right, Down],
            &[Down, Right],
        ],
    ],
    &[
        // 3
        &[
            // 3-3
            &[],
        ],
        &[
            // 3-4
            &[Up, Left, Left],
            &[Left, Up, Left],
            &[Left, Left, Up],
        ],
        &[
            // 3-5
            &[Up, Left],
            &[Left, Up],
        ],
        &[
            // 3-6
            &[Up],
        ],
        &[
            // 3-7
            &[Up, Up, Left, Left],
            &[Up, Left, Up, Left],
            &[Left, Up, Up, Left],
            &[Left, Up, Up, Left],
            &[Up, Left, Left, Up],
            &[Left, Up, Left, Up],
            &[Left, Left, Up, Up],
        ],
        &[
            // 3-8
            &[Up, Up, Left],
            &[Up, Left, Up],
            &[Left, Up, Up],
        ],
        &[
            // 3-9
            &[Up, Up],
        ],
        &[
            // 3-A
            &[Down],
        ],
    ],
    &[
        // 4
        &[
            // 4-4
            &[],
        ],
        &[
            // 4-5
            &[Right],
        ],
        &[
            // 4-6
            &[Right, Right],
        ],
        &[
            // 4-7
            &[Up],
        ],
        &[
            // 4-8
            &[Up, Right],
            &[Right, Up],
        ],
        &[
            // 4-9
            &[Up, Right, Right],
            &[Right, Up, Right],
            &[Right, Right, Up],
        ],
        &[
            // 4-A
            &[Right, Right, Down, Down],
            &[Right, Down, Right, Down],
            &[Right, Down, Down, Right],
            &[Down, Right, Right, Down],
            &[Down, Right, Down, Right],
        ],
    ],
    &[
        // 5
        &[
            // 5-5
            &[],
        ],
        &[
            // 5-6
            &[Right],
        ],
        &[
            // 5-7
            &[Up, Left],
            &[Left, Up],
        ],
        &[
            // 5-8
            &[Up],
        ],
        &[
            // 5-9
            &[Up, Right],
            &[Right, Up],
        ],
        &[
            // 5-A
            &[Right, Down, Down],
            &[Down, Right, Down],
            &[Down, Down, Right],
        ],
    ],
    &[
        // 6
        &[
            // 6-6
            &[],
        ],
        &[
            // 6-7
            &[Up, Left, Left],
            &[Left, Up, Left],
            &[Left, Left, Up],
        ],
        &[
            // 6-8
            &[Up, Left],
            &[Left, Up],
        ],
        &[
            // 6-9
            &[Up],
        ],
        &[
            // 6-A
            &[Down, Down],
        ],
    ],
    &[
        // 7
        &[
            // 7-7
            &[],
        ],
        &[
            // 7-8
            &[Right],
        ],
        &[
            // 7-9
            &[Right, Right],
        ],
        &[
            // 7-A
            &[Right, Right, Down, Down, Down],
            &[Right, Down, Right, Down, Down],
            &[Right, Down, Down, Right, Down],
            &[Right, Down, Down, Down, Right],
            &[Down, Right, Right, Down, Down],
            &[Down, Right, Down, Right, Down],
            &[Down, Right, Down, Down, Right],
            &[Down, Down, Right, Right, Down],
            &[Down, Down, Right, Down, Right],
        ],
    ],
    &[
        // 8
        &[
            // 8-8
            &[],
        ],
        &[
            // 8-9
            &[Right],
        ],
        &[
            // 8-A
            &[Right, Down, Down, Down],
            &[Down, Right, Down, Down],
            &[Down, Down, Right, Down],
            &[Down, Down, Down, Right],
        ],
    ],
    &[
        // 9
        &[
            // 9-9
            &[],
        ],
        &[
            // 9-A
            &[Down, Down, Down],
        ],
    ],
    &[
        // A
        &[
            // A-A
            &[],
        ],
    ],
];
