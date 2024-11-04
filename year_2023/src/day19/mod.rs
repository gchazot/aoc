use crate::utils;
use std::collections::HashMap;

#[test]
fn test_mine() {
    execute();
}
pub fn execute() {
    let mine = TriageCenter::from_lines(utils::read_lines("src/day19/mine.txt"));
    assert_eq!(376008, mine.process());
}

#[test]
fn test_example() {
    let example = TriageCenter::from_lines(_example());
    assert_eq!(19114, example.process());
}

struct TriageCenter {
    parts: Vec<Part>,
    workflows: Vec<Workflow>,
}

impl TriageCenter {
    fn from_lines(lines: Vec<String>) -> TriageCenter {
        let mut workflow_strs = Vec::new();
        let mut parts_strs = Vec::new();

        let mut separator_found = false;
        lines.iter().for_each(|line| {
            if line.is_empty() {
                separator_found = true;
            } else if separator_found {
                parts_strs.push(line);
            } else {
                workflow_strs.push(line)
            }
        });
        let workflows = workflow_strs.into_iter().map(Workflow::from_line).collect();
        let parts = parts_strs
            .into_iter()
            .map(|line| Part::from_line(line))
            .collect();

        TriageCenter { workflows, parts }
    }

    fn process(&self) -> i32 {
        self.parts
            .iter()
            .map(|part| match self.process_part(part) {
                Decision::Accept => part.ratings.values().sum(),
                _ => 0,
            })
            .sum()
    }

    fn process_part(&self, part: &Part) -> Decision {
        let mut wf = self.get_workflow(&"in".to_string());
        loop {
            let decision = wf.decide(part);
            match decision {
                Decision::Accept | Decision::Reject => {
                    break decision;
                }
                Decision::Redirect(next) => wf = self.get_workflow(&next),
            }
        }
    }

    fn get_workflow(&self, name: &String) -> &Workflow {
        self.workflows.iter().find(|&w| w.name == *name).unwrap()
    }
}

#[test]
fn test_process_part() {
    let example = TriageCenter::from_lines(_example());

    assert!(matches!(
        example.process_part(&_test_part(787, 2655, 1222, 2876)),
        Decision::Accept
    ));
    assert!(matches!(
        example.process_part(&_test_part(1679, 44, 2067, 496)),
        Decision::Reject
    ));
    assert!(matches!(
        example.process_part(&_test_part(2036, 264, 79, 2244)),
        Decision::Accept
    ));
    assert!(matches!(
        example.process_part(&_test_part(2461, 1339, 466, 291)),
        Decision::Reject
    ));
    assert!(matches!(
        example.process_part(&_test_part(2127, 1623, 2188, 1013)),
        Decision::Accept
    ));
}

struct Part {
    ratings: HashMap<Category, i32>,
}

impl Part {
    fn from_line(line: &String) -> Part {
        let ratings = line
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(|rating_str| {
                let (category_str, value_str) = rating_str.split_once("=").unwrap();
                (
                    Category::from_string(category_str),
                    value_str.parse::<i32>().unwrap(),
                )
            })
            .collect::<HashMap<Category, i32>>();

        Part { ratings }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_line(line: &String) -> Workflow {
        let (name, rest) = line.split_once("{").unwrap();
        let rules = rest
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(Rule::from_string)
            .collect();

        Workflow {
            name: name.to_string(),
            rules,
        }
    }

    fn decide(&self, part: &Part) -> Decision {
        for rule in &self.rules {
            match rule.decide(part) {
                None => continue,
                Some(decision) => {
                    return decision.clone();
                }
            };
        }
        unreachable!()
    }
}

#[test]
fn test_workflow_decide() {
    let wf = Workflow::from_line(&"px{a<2006:qkq,m>2090:A,rfg}".to_string());

    assert!(matches!(
        wf.decide(&_test_part(0, 0, 1000, 0)),
        Decision::Redirect(next) if next == "qkq",
    ));
    assert!(matches!(
        wf.decide(&_test_part(0, 3000, 3000, 0)),
        Decision::Accept
    ));
    assert!(matches!(
        wf.decide(&_test_part(0, 1000, 3000, 0)),
        Decision::Redirect(next) if next == "rfg"
    ));
}

struct Rule {
    condition: Option<Condition>,
    decision: Decision,
}

impl Rule {
    fn from_string(s: &str) -> Rule {
        if s.contains(":") {
            let (condition_str, decision_str) = s.split_once(":").unwrap();
            let condition = Some(Condition::from_string(condition_str));
            let decision = Decision::from_string(decision_str);
            Rule {
                condition,
                decision,
            }
        } else {
            Rule {
                condition: None,
                decision: Decision::from_string(s),
            }
        }
    }

    fn decide(&self, part: &Part) -> Option<&Decision> {
        match &self.condition {
            None => Some(&self.decision),
            Some(condition) => {
                if condition.matches(&part) {
                    Some(&self.decision)
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn test_rule_decide() {
    let rule1 = Rule::from_string("x<10:qkq");
    assert!(matches!(rule1.decide(&_test_part(20, 0, 0, 0)), None));
    assert!(matches!(rule1.decide(&_test_part(10, 0, 0, 0)), None));
    assert!(matches!(
        rule1.decide(&_test_part(9, 3, 4, 5)),
        Some(Decision::Redirect(next)) if next == "qkq",
    ));

    let rule2 = Rule::from_string("m>10:A");
    assert!(matches!(rule2.decide(&_test_part(0, 2, 0, 0)), None));
    assert!(matches!(rule2.decide(&_test_part(0, 10, 0, 0)), None));
    assert!(matches!(
        rule2.decide(&_test_part(15, 30, 40, 50)),
        Some(Decision::Accept)
    ));

    let rule3 = Rule::from_string("rfg");
    assert!(matches!(
            rule3.decide(&_test_part(0, 0, 0, 0)),
            Some(Decision::Redirect(next)) if next == "rfg"));
    assert!(matches!(
            rule3.decide(&_test_part(10,10, 10, 10)),
            Some(Decision::Redirect(next)) if next == "rfg"));

    let rule4 = Rule::from_string("R");
    assert!(matches!(
        rule4.decide(&_test_part(0, 0, 0, 0)),
        Some(Decision::Reject)
    ));
    assert!(matches!(
        rule4.decide(&_test_part(10, 10, 10, 10)),
        Some(Decision::Reject)
    ));
}

#[derive(Eq, PartialEq, Debug)]
struct Condition {
    category: Category,
    test: Test,
    threshold: i32,
}

impl Condition {
    fn from_string(s: &str) -> Condition {
        let category = Category::from_string(&s[0..1]);
        let test = Test::from_string(&s[1..2]);
        let threshold = s[2..].parse::<i32>().unwrap();

        Condition {
            category,
            test,
            threshold,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let value = part.ratings[&self.category];
        match self.test {
            Test::MoreThan => value > self.threshold,
            Test::LessThan => value < self.threshold,
        }
    }
}

#[test]
fn test_condition_matches() {
    let c1 = Condition::from_string("a<1234");
    for i in [-10, 0, 1233, 1234, 1235] {
        for j in [-10, 0, 1233, 1234, 1235] {
            let part = _test_part(j, j, i, j);
            assert_eq!(c1.matches(&part), i < 1234);
        }
    }

    let c2 = Condition::from_string("a>1234");
    for i in [-10, 0, 1233, 1234, 1235] {
        for j in [-10, 0, 1233, 1234, 1235] {
            let part = _test_part(j, j, i, j);
            assert_eq!(c2.matches(&part), i > 1234);
        }
    }

    let part = _test_part(2, 3, 4, 5);

    let condition = |ct, val| Condition::from_string(&format!("{ct}{val}").to_string());

    for i in -1..7 {
        assert_eq!(condition("x>", i).matches(&part), 2 > i);
        assert_eq!(condition("m>", i).matches(&part), 3 > i);
        assert_eq!(condition("a>", i).matches(&part), 4 > i);
        assert_eq!(condition("s>", i).matches(&part), 5 > i);

        assert_eq!(condition("x<", i).matches(&part), 2 < i);
        assert_eq!(condition("m<", i).matches(&part), 3 < i);
        assert_eq!(condition("a<", i).matches(&part), 4 < i);
        assert_eq!(condition("s<", i).matches(&part), 5 < i);
    }
}

fn _test_part(x: i32, m: i32, a: i32, s: i32) -> Part {
    Part {
        ratings: HashMap::from([
            (Category::X, x),
            (Category::M, m),
            (Category::A, a),
            (Category::S, s),
        ]),
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_string(s: &str) -> Category {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Test {
    MoreThan,
    LessThan,
}

impl Test {
    fn from_string(s: &str) -> Test {
        match s {
            ">" => Test::MoreThan,
            "<" => Test::LessThan,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
enum Decision {
    Accept,
    Reject,
    Redirect(String),
}

impl Decision {
    fn from_string(s: &str) -> Decision {
        match s {
            "A" => Decision::Accept,
            "R" => Decision::Reject,
            _ => Decision::Redirect(s.to_string()),
        }
    }
}

#[test]
fn test_from_lines() {
    let example = TriageCenter::from_lines(_example());

    assert_eq!(11, example.workflows.len());

    let wf1 = &example.workflows[0];
    assert_eq!("px", wf1.name);
    assert_eq!(3, wf1.rules.len());
    let wf1r1 = &wf1.rules[0];
    assert!(matches!(&wf1r1.decision, Decision::Redirect(to) if to == "qkq"));
    let wf1r1cond = wf1r1.condition.as_ref().unwrap();
    assert_eq!(Category::A, wf1r1cond.category);
    assert_eq!(Test::LessThan, wf1r1cond.test);
    assert_eq!(2006, wf1r1cond.threshold);

    let wf1r2 = &wf1.rules[1];
    assert!(matches!(&wf1r2.decision, Decision::Accept,));
    let wf1r2cond = wf1r2.condition.as_ref().unwrap();
    assert_eq!(Category::M, wf1r2cond.category);
    assert_eq!(Test::MoreThan, wf1r2cond.test);
    assert_eq!(2090, wf1r2cond.threshold);

    let wf1r3 = &wf1.rules[2];
    assert!(matches!(&wf1r3.decision, Decision::Redirect(to) if to == "rfg"));
    assert!(matches!(wf1r3.condition, None));

    assert_eq!(5, example.parts.len());
    assert!(example.parts.iter().all(|part| part.ratings.len() == 4));

    assert_eq!(787, example.parts[0].ratings[&Category::X]);
    assert_eq!(2655, example.parts[0].ratings[&Category::M]);
    assert_eq!(1222, example.parts[0].ratings[&Category::A]);
    assert_eq!(2876, example.parts[0].ratings[&Category::S]);

    assert_eq!(2127, example.parts[4].ratings[&Category::X]);
    assert_eq!(1623, example.parts[4].ratings[&Category::M]);
    assert_eq!(2188, example.parts[4].ratings[&Category::A]);
    assert_eq!(1013, example.parts[4].ratings[&Category::S]);
}

fn _example() -> Vec<String> {
    vec![
        "px{a<2006:qkq,m>2090:A,rfg}".to_string(),
        "pv{a>1716:R,A}".to_string(),
        "lnx{m>1548:A,A}".to_string(),
        "rfg{s<537:gd,x>2440:R,A}".to_string(),
        "qs{s>3448:A,lnx}".to_string(),
        "qkq{x<1416:A,crn}".to_string(),
        "crn{x>2662:A,R}".to_string(),
        "in{s<1351:px,qqz}".to_string(),
        "qqz{s>2770:qs,m<1801:hdj,R}".to_string(),
        "gd{a>3333:R,R}".to_string(),
        "hdj{m>838:A,pv}".to_string(),
        "".to_string(),
        "{x=787,m=2655,a=1222,s=2876}".to_string(),
        "{x=1679,m=44,a=2067,s=496}".to_string(),
        "{x=2036,m=264,a=79,s=2244}".to_string(),
        "{x=2461,m=1339,a=466,s=291}".to_string(),
        "{x=2127,m=1623,a=2188,s=1013}".to_string(),
    ]
}
