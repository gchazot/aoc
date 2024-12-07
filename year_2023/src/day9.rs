use aoc_utils as utils;
use std::collections::VecDeque;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let readings = Reading::from_file("day9.txt");
    assert_eq!((1772145754, 867), extrapolate_all(readings));
}

#[test]
fn test_extrapolate_all() {
    let readings = Reading::from_file("day9-example.txt");
    assert_eq!((114, 2), extrapolate_all(readings));
}

fn extrapolate_all(mut readings: Vec<Reading>) -> (Value, Value) {
    let mut total_right = 0 as Value;
    let mut total_left = 0 as Value;

    for reading in readings.iter_mut() {
        reading.extrapolate();
        total_right += reading.last();
        total_left += reading.first();
    }

    (total_right, total_left)
}

#[test]
fn test_extrapolate() {
    let mut reading1 = Reading::from_line("0 3 6 9 12 15");
    reading1.extrapolate();

    assert_eq!(8, reading1.values.len());
    assert_eq!(18, reading1.values[7]);
    assert_eq!(-3, reading1.values[0]);

    let mut reading2 = Reading::from_line("10 13 16 21 30 45");
    reading2.extrapolate();

    assert_eq!(8, reading2.values.len());
    assert_eq!(68, reading2.values[7]);
    assert_eq!(5, reading2.values[0]);
}

#[test]
fn test_derive_fully() {
    let reading1 = Reading::from_line("0 3 6 9 12 15");
    assert_eq!(2, reading1.derive_fully().len());

    let reading2 = Reading::from_line("1 3 6 10 15 21");
    assert_eq!(3, reading2.derive_fully().len());

    let reading3 = Reading::from_line("10 13 16 21 30 45");
    assert_eq!(4, reading3.derive_fully().len());
}

#[test]
fn test_is_zeros() {
    let reading1 = Reading::from_line("0 3 6 9 12 15");
    assert_eq!(false, reading1.is_zeros());
    let reading2 = Reading::from_line("0 0 0");
    assert_eq!(true, reading2.is_zeros());
    let reading2 = Reading::from_line("");
    assert_eq!(true, reading2.is_zeros());
}

#[test]
fn test_derive_reading() {
    let reading1 = Reading::from_line("0 3 6 9 12 15");
    let derivative1_1 = reading1.derive();
    assert_eq!(Values::from([3; 5]), derivative1_1.values);
    let derivative1_2 = derivative1_1.derive();
    assert_eq!(Values::from([0; 4]), derivative1_2.values);

    let reading2 = Reading::from_line("1 3 6 10 15 21");
    let derivative2_1 = reading2.derive();
    assert_eq!(Values::from([2, 3, 4, 5, 6]), derivative2_1.values);
    let derivative2_2 = derivative2_1.derive();
    assert_eq!(Values::from([1; 4]), derivative2_2.values);
    let derivative2_3 = derivative2_2.derive();
    assert_eq!(Values::from([0; 3]), derivative2_3.values);
}

#[test]
fn test_parse_readings() {
    let readings = Reading::from_file("day9-example.txt");
    assert_eq!(3, readings.len());
    assert_eq!(Values::from([0, 3, 6, 9, 12, 15]), readings[0].values);
    assert_eq!(Values::from([1, 3, 6, 10, 15, 21]), readings[1].values);
    assert_eq!(Values::from([10, 13, 16, 21, 30, 45]), readings[2].values);
}

#[test]
fn test_parse_reading() {
    let reading = Reading::from_line("0 3 6 9 -12 15");
    assert_eq!(Values::from([0, 3, 6, 9, -12, 15]), reading.values);
}

type Value = i32;
type Values = VecDeque<Value>;

struct Reading {
    values: Values,
}

impl Reading {
    fn from_file(filename: &str) -> Vec<Reading> {
        let path = format!("input/{}", &filename);
        let lines = utils::read_lines(&path);

        lines
            .iter()
            .map(String::as_str)
            .map(Reading::from_line)
            .collect()
    }

    fn from_line(line: &str) -> Reading {
        let values = line
            .trim()
            .split_whitespace()
            .map(str::parse::<Value>)
            .map(Result::unwrap)
            .collect();
        Reading { values }
    }

    fn first(&self) -> Value {
        self.values[0]
    }
    fn last(&self) -> Value {
        self.values[self.values.len() - 1]
    }

    fn extrapolate(&mut self) {
        let mut derivatives = self.derive_fully();

        let mut increment_back = 0 as Value;
        let mut increment_front = 0 as Value;
        for derivative in derivatives.iter_mut().rev() {
            increment_back = derivative.last() + increment_back;
            derivative.values.push_back(increment_back);

            increment_front = derivative.first() - increment_front;
            derivative.values.push_front(increment_front);
        }

        let last_value = self.last();
        self.values.push_back(last_value + increment_back);

        let first_value = self.first();
        self.values.push_front(first_value - increment_front);
    }

    fn derive_fully(&self) -> Vec<Reading> {
        let mut result = Vec::new();
        let mut cur = self;
        while !cur.is_zeros() {
            let new = cur.derive();
            result.push(new);
            cur = result.last().unwrap();
        }
        result
    }

    fn derive(&self) -> Reading {
        let values = (1..self.values.len())
            .map(|i| self.values[i] - self.values[i - 1])
            .collect();
        Reading { values }
    }

    fn is_zeros(&self) -> bool {
        self.values.iter().all(|&v| v == 0)
    }
}
