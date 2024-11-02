use std::fs::read_to_string;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let mine = read_sequence("src/day15/mine.txt");
    assert_eq!(505459, hash_sequence(&mine));

    let mut lab = Lab::new();
    mine.iter().for_each(|instruction| lab.execute(instruction));

    assert_eq!(228508, lab.score())
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

#[derive(Clone)]
struct LabelledLens {
    label: String,
    power: u8,
}

impl LabelledLens {
    fn new(label: &str, power: u8) -> LabelledLens {
        LabelledLens {
            label: label.to_string(),
            power,
        }
    }
}

#[derive(Clone)]
struct LensBox {
    lenses: Vec<LabelledLens>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: vec![] }
    }

    fn add(&mut self, new_lens: LabelledLens) {
        let replaced = self
            .lenses
            .iter_mut()
            .find(|lens| lens.label == new_lens.label)
            .and_then(|lens| {
                lens.power = new_lens.power;
                Some(lens)
            });
        if replaced.is_some() {
            replaced.unwrap().power = new_lens.power;
        } else {
            self.lenses.push(new_lens);
        };
    }

    fn remove(&mut self, label: String) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn score(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| (i as u32 + 1) * lens.power as u32)
            .sum()
    }
}

#[test]
fn test_add_lens() {
    let mut lensbox = LensBox::new();

    lensbox.add(LabelledLens::new("he", 1));
    assert_eq!(1, lensbox.lenses.len());
    assert_eq!("he", lensbox.lenses[0].label);
    assert_eq!(1, lensbox.lenses[0].power);

    lensbox.add(LabelledLens::new("he", 2));
    assert_eq!(1, lensbox.lenses.len());
    assert_eq!("he", lensbox.lenses[0].label);
    assert_eq!(2, lensbox.lenses[0].power);

    lensbox.add(LabelledLens::new("llo", 5));
    assert_eq!(2, lensbox.lenses.len());
    assert_eq!("he", lensbox.lenses[0].label);
    assert_eq!(2, lensbox.lenses[0].power);
    assert_eq!("llo", lensbox.lenses[1].label);
    assert_eq!(5, lensbox.lenses[1].power);

    lensbox.add(LabelledLens::new("he", 8));
    assert_eq!(2, lensbox.lenses.len());
    assert_eq!("he", lensbox.lenses[0].label);
    assert_eq!(8, lensbox.lenses[0].power);
    assert_eq!("llo", lensbox.lenses[1].label);
    assert_eq!(5, lensbox.lenses[1].power);
}

#[test]
fn test_remove_lens() {
    let mut lensbox = LensBox::new();
    lensbox.add(LabelledLens::new("he", 1));
    lensbox.add(LabelledLens::new("llo", 2));
    lensbox.add(LabelledLens::new("wo", 6));
    lensbox.add(LabelledLens::new("rld", 9));
    assert_eq!(4, lensbox.lenses.len());

    lensbox.remove("llo".to_string());
    assert_eq!(3, lensbox.lenses.len());
    assert_eq!("he", lensbox.lenses[0].label);
    assert_eq!("wo", lensbox.lenses[1].label);
    assert_eq!("rld", lensbox.lenses[2].label);
}

struct Lab {
    boxes: Vec<LensBox>,
}

impl Lab {
    fn new() -> Lab {
        Lab {
            boxes: vec![LensBox::new(); 256],
        }
    }

    fn execute(&mut self, instruction: &String) {
        if instruction.ends_with("-") {
            self.remove(instruction.strip_suffix("-").unwrap().to_string());
        } else {
            let mut parts = instruction.split("=");
            let label = parts.next().unwrap();
            let power = parts.next().unwrap().to_string().parse::<u8>().unwrap();
            self.add(LabelledLens::new(label, power));
        }
    }

    fn add(&mut self, new_lens: LabelledLens) {
        let box_index = hash_step(&new_lens.label);
        self.boxes[box_index as usize].add(new_lens);
    }

    fn remove(&mut self, label: String) {
        let box_index = hash_step(&label);
        self.boxes[box_index as usize].remove(label);
    }

    fn score(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, lensbox)| (i as u32 + 1) * lensbox.score())
            .sum()
    }
}

#[test]
fn test_apply_instructions() {
    let mut lab = Lab::new();

    lab.add(LabelledLens::new("rn", 1));
    assert_eq!(1, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.remove("cm".to_string());
    assert_eq!(1, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.add(LabelledLens::new("qp", 3));
    assert_eq!(1, lab.boxes[0].lenses.len());
    assert_eq!(1, lab.boxes[1].lenses.len());
    for i in 2..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.add(LabelledLens::new("cm", 2));
    assert_eq!(2, lab.boxes[0].lenses.len());
    assert_eq!(1, lab.boxes[1].lenses.len());
    for i in 2..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.remove("qp".to_string());
    assert_eq!(2, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }
}

#[test]
fn test_execute() {
    let mut lab = Lab::new();

    lab.execute(&"rn=1".to_string());
    assert_eq!(1, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.execute(&"cm-".to_string());
    assert_eq!(1, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.execute(&"qp=3".to_string());
    assert_eq!(1, lab.boxes[0].lenses.len());
    assert_eq!(1, lab.boxes[1].lenses.len());
    for i in 2..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.execute(&"cm=2".to_string());
    assert_eq!(2, lab.boxes[0].lenses.len());
    assert_eq!(1, lab.boxes[1].lenses.len());
    for i in 2..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }

    lab.execute(&"qp-".to_string());
    assert_eq!(2, lab.boxes[0].lenses.len());
    for i in 1..256 {
        assert_eq!(0, lab.boxes[i].lenses.len());
    }
}

#[test]
fn test_example() {
    let mut lab = Lab::new();

    lab.execute(&"rn=1".to_string());
    lab.execute(&"cm-".to_string());
    lab.execute(&"qp=3".to_string());
    lab.execute(&"cm=2".to_string());
    lab.execute(&"qp-".to_string());
    lab.execute(&"pc=4".to_string());
    lab.execute(&"ot=9".to_string());
    lab.execute(&"ab=5".to_string());
    lab.execute(&"pc-".to_string());
    lab.execute(&"pc=6".to_string());
    lab.execute(&"ot=7".to_string());

    assert_eq!(145, lab.score());
}
