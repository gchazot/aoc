use std::fs::read_to_string;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let mine = read_sequence("src/day15/mine.txt");
    assert_eq!(505459, hash_sequence(&mine));
}

fn hash_sequence(sequence: &Vec<String>) -> u32 {
    sequence.iter().map(|step| hash_step(&step) as u32).sum()
}
fn hash_step(step: &String) -> u8 {
    let mut result = 0;
    for c in step.chars() {
        let c_val = c as u8;
        let add: u16 = result as u16 + c_val as u16;
        let mult = 17u16 * add;
        result = mult as u8;
        // println!("{} {} {} {} {}", c, c_val, add, mult, result);
    }
    result
}

fn read_sequence(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn test_hash_step() {
    assert_eq!(52, hash_step(&"HASH".to_string()));

    assert_eq!(30, hash_step(&"rn=1".to_string()));
    assert_eq!(253, hash_step(&"cm-".to_string()));
    assert_eq!(97, hash_step(&"qp=3".to_string()));
    assert_eq!(47, hash_step(&"cm=2".to_string()));
    assert_eq!(14, hash_step(&"qp-".to_string()));
    assert_eq!(180, hash_step(&"pc=4".to_string()));
    assert_eq!(9, hash_step(&"ot=9".to_string()));
    assert_eq!(197, hash_step(&"ab=5".to_string()));
    assert_eq!(48, hash_step(&"pc-".to_string()));
    assert_eq!(214, hash_step(&"pc=6".to_string()));
    assert_eq!(231, hash_step(&"ot=7".to_string()));
}

#[test]
fn test_hash_sequence() {
    let example = vec![
        "rn=1".to_string(),
        "cm-".to_string(),
        "qp=3".to_string(),
        "cm=2".to_string(),
        "qp-".to_string(),
        "pc=4".to_string(),
        "ot=9".to_string(),
        "ab=5".to_string(),
        "pc-".to_string(),
        "pc=6".to_string(),
        "ot=7".to_string(),
    ];
    assert_eq!(1320, hash_sequence(&example));
}
