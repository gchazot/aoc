use crate::utils;

pub fn execute() {
    let input = utils::read_lines("src/day1/mine.txt");
    assert_eq!(Some(54927), sum_lines(&input, false));
    assert_eq!(Some(54581), sum_lines(&input, true));
}

struct Digit<'a> {
    text: &'a str,
    number: &'a str,
    value: u32,
}

const DIGITS: [Digit; 10] = [
    Digit{ value: 1, number: "1", text: "one"  },
    Digit{ value: 2, number: "2", text: "two"  },
    Digit{ value: 3, number: "3", text: "three"  },
    Digit{ value: 4, number: "4", text: "four"  },
    Digit{ value: 5, number: "5", text: "five"  },
    Digit{ value: 6, number: "6", text: "six"  },
    Digit{ value: 7, number: "7", text: "seven"  },
    Digit{ value: 8, number: "8", text: "eight"  },
    Digit{ value: 9, number: "9", text: "nine"  },
    Digit{ value: 0, number: "0", text: "zero"  },
];

#[test]
fn test_sum_lines() {
    let example1 = utils::read_lines("src/day1/example1.txt");
    assert_eq!(Some(142), sum_lines(&example1, false));

    let example2 = utils::read_lines("src/day1/example2.txt");
    assert_eq!(Some(281), sum_lines(&example2, true));
}

fn sum_lines<T: AsRef<str>>(input: &[T], with_text: bool) -> Option<u32> {
    let mut total: Option<u32> = None;
    for line in input {
        let num: Option<u32> = calculate_line(line.as_ref(), with_text);

        if num.is_some() {
            if total.is_none() {
                 total = num
            }
            else {
                total = Some(total.unwrap() + num.unwrap());
            }
        }
    }
    return total;
}

fn calculate_line(line: &str, with_text: bool) -> Option<u32> {
    const BASE: u32 = 10;
    let mut a: Option<u32> = None;
    let mut b: Option<u32> = None;

    for i in 0..line.len() {
        let slice = &line[i..];
        for digit in DIGITS {
            if slice.starts_with(digit.number) || (with_text && slice.starts_with(digit.text)) {
                if a.is_none() {
                    a = Some(digit.value);
                }
                b = Some(digit.value);
            }
        }
    }

    if a.is_some() && b.is_some() {
        let result = BASE * a.unwrap() + b.unwrap();
        return Some(result);
    } else {
        return None;
    }
}
