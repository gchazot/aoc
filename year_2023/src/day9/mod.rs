use crate::utils;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let readings = Reading::from_file("mine.txt");
    assert_eq!(1772145754, extrapolate_all(readings));
}

#[test]
fn test_extrapolate_all() {
    let readings = Reading::from_file("example.txt");
    assert_eq!(114, extrapolate_all(readings));
}

fn extrapolate_all(mut readings: Vec<Reading>) -> Value {
    readings
        .iter_mut()
        .map(|reading| {
            reading.extrapolate();
            reading.values.last().unwrap()
        })
        .sum()
}

#[test]
fn test_extrapolate() {
    let mut reading1 = Reading::from_line("0 3 6 9 12 15");
    reading1.extrapolate();
    assert_eq!(7, reading1.values.len());
    assert_eq!(18, *reading1.values.last().unwrap());
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
    assert_eq!(vec![3; 5], derivative1_1.values);
    let derivative1_2 = derivative1_1.derive();
    assert_eq!(vec![0; 4], derivative1_2.values);

    let reading2 = Reading::from_line("1 3 6 10 15 21");
    let derivative2_1 = reading2.derive();
    assert_eq!(vec![2, 3, 4, 5, 6], derivative2_1.values);
    let derivative2_2 = derivative2_1.derive();
    assert_eq!(vec![1; 4], derivative2_2.values);
    let derivative2_3 = derivative2_2.derive();
    assert_eq!(vec![0; 3], derivative2_3.values);
}

#[test]
fn test_parse_readings() {
    let readings = Reading::from_file("example.txt");
    assert_eq!(3, readings.len());
    assert_eq!(vec![0, 3, 6, 9, 12, 15], readings[0].values);
    assert_eq!(vec![1, 3, 6, 10, 15, 21], readings[1].values);
    assert_eq!(vec![10, 13, 16, 21, 30, 45], readings[2].values);
}

#[test]
fn test_parse_reading() {
    let reading = Reading::from_line("0 3 6 9 -12 15");
    assert_eq!(vec![0, 3, 6, 9, -12, 15], reading.values);
}

type Value = i32;

struct Reading {
    values: Vec<Value>,
}

impl Reading {
    fn from_file(filename: &str) -> Vec<Reading> {
        let path = format!("src/day9/{}", &filename);
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

    fn extrapolate(&mut self) {
        let mut derivatives = self.derive_fully();

        let mut increment = 0 as Value;
        for i in 0..derivatives.len() {
            let j = derivatives.len() - 1 - i;
            let last_value = derivatives[j].values.last().unwrap();
            increment = last_value + increment;
            derivatives[j].values.push(increment);
        }

        let last_value = self.values.last().unwrap();
        self.values.push(last_value + increment);
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
