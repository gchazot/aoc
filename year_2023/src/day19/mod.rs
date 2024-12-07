use aoc_utils as utils;
use std::collections::HashMap;
use std::fmt::Debug;

#[test]
fn test_mine() {
    execute();
}
pub fn execute() {
    let mine = TriageCenter::from_lines(utils::read_lines("src/day19/mine.txt"));
    assert_eq!(376008, mine.process());

    assert_eq!(124078207789312, mine.count_combinations());
}

#[test]
fn test_example() {
    let example = TriageCenter::from_lines(_example());
    assert_eq!(19114, example.process());

    assert_eq!(167409079868000, example.count_combinations());
}

struct TriageCenter {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
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
        self.workflows.get(name).unwrap()
    }

    fn count_combinations(&self) -> i64 {
        use Category::*;

        let all_routes = self._explore_one(self.get_workflow(&"in".to_string()));

        all_routes
            .iter()
            .map(|route| {
                let mut mins = HashMap::from([(X, 0), (M, 0), (A, 0), (S, 0)]);
                let mut maxs = HashMap::from([(X, 4000), (M, 4000), (A, 4000), (S, 4000)]);

                fn setmin(mins: &mut HashMap<Category, i64>, cat: &Category, val: i64) {
                    if mins[&cat] < val {
                        mins.insert(cat.clone(), val);
                    }
                }
                fn setmax(maxs: &mut HashMap<Category, i64>, cat: &Category, val: i64) {
                    if maxs[&cat] > val {
                        maxs.insert(cat.clone(), val);
                    }
                }

                for branch in route {
                    match branch {
                        Branch::True(condition) => match condition.test {
                            Test::MoreThan => {
                                setmin(&mut mins, &condition.category, condition.threshold as i64)
                            }
                            Test::LessThan => setmax(
                                &mut maxs,
                                &condition.category,
                                condition.threshold as i64 - 1,
                            ),
                        },
                        Branch::False(condition) => match condition.test {
                            Test::MoreThan => {
                                setmax(&mut maxs, &condition.category, condition.threshold as i64)
                            }
                            Test::LessThan => setmin(
                                &mut mins,
                                &condition.category,
                                condition.threshold as i64 - 1,
                            ),
                        },
                    }
                }

                vec![X, M, A, S]
                    .iter()
                    .map(|cat| maxs[&cat] - mins[&cat])
                    .product::<i64>()
            })
            .sum()
    }

    fn _explore_one(&self, workflow: &Workflow) -> Vec<Vec<Branch>> {
        use Branch::*;

        let mut result = vec![];

        let mut current = vec![];

        for rule in workflow.rules.iter() {
            match &rule.decision {
                Decision::Accept => {
                    let mut child_result = current.clone();
                    if rule.condition.is_some() {
                        child_result.push(True(rule.condition.clone().unwrap()));
                    }
                    result.push(child_result);
                }
                Decision::Reject => {}
                Decision::Redirect(next) => {
                    let mut children = self._explore_one(self.get_workflow(&next));
                    for child in children.iter_mut() {
                        let mut child_result = current.clone();
                        if rule.condition.is_some() {
                            child_result.push(True(rule.condition.clone().unwrap()))
                        }
                        child_result.append(child);
                        result.push(child_result);
                    }
                }
            }
            if rule.condition.is_none() {
                break;
            } else {
                current.push(False(rule.condition.clone().unwrap()));
            }
        }

        result
    }
}

#[derive(Clone, Debug)]
enum Branch {
    True(Condition),
    False(Condition),
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
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_line(line: &String) -> (String, Workflow) {
        let (name, rest) = line.split_once("{").unwrap();
        let rules = rest
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(Rule::from_string)
            .collect();

        (name.to_string(), Workflow { rules })
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
    let wf = Workflow::from_line(&"px{a<2006:qkq,m>2090:A,rfg}".to_string()).1;

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

#[derive(Eq, PartialEq, Clone)]
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

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.category.to_string(),
            self.test.to_string(),
            self.threshold
        )
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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
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

    fn to_string(&self) -> String {
        match self {
            Category::X => "x".to_string(),
            Category::M => "m".to_string(),
            Category::A => "a".to_string(),
            Category::S => "s".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
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

    fn to_string(&self) -> String {
        match self {
            Test::MoreThan => ">".to_string(),
            Test::LessThan => "<".to_string(),
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

    let wf1 = &example.get_workflow(&"px".to_string());
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
