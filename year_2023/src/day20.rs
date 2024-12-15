use std::collections::{HashMap, VecDeque};

pub fn execute() -> String {
    let mut mine = Desert::from_lines(aoc_utils::read_lines("input/day20.txt"));
    let (mut seen_low, mut seen_high) = mine.button_press(&mut HashMap::new());
    for _ in 1..1000 {
        (seen_low, seen_high) = mine.button_press(&mut HashMap::new());
    }
    let part1 = seen_low * seen_high;

    let mut desert = Desert::from_lines(aoc_utils::read_lines("input/day20.txt"));
    let part2 = desert.find_min_button_presses_for_rx();

    format!("{} {}", part1, part2)
}

struct Desert {
    modules: Vec<Box<dyn Module>>,
    pulses: VecDeque<Pulse>,
    pulses_seen_low: u32,
    pulses_seen_high: u32,
}

struct Pulse {
    from: String,
    to: String,
    high: bool,
}

impl Desert {
    fn from_lines(lines: Vec<String>) -> Desert {
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        let mut modules: Vec<Box<dyn Module>> = lines
            .iter()
            .map(|line| -> Box<dyn Module> {
                let (module_str, destinations_str) = line.split_once(" -> ").unwrap();
                let destinations = destinations_str
                    .split(", ")
                    .map(String::from)
                    .collect::<Vec<_>>();
                let type_str = &module_str[0..1];
                let name = match type_str {
                    "%" => &module_str[1..],
                    "&" => &module_str[1..],
                    _ => &module_str,
                }
                .to_string();

                for dest in destinations.iter() {
                    inputs
                        .entry(dest.clone())
                        .or_insert(Vec::new())
                        .push(name.clone());
                }

                match type_str {
                    "%" => Box::new(FlipFlop::new(name.clone(), destinations)),
                    "&" => Box::new(Conjunction::new(name.clone(), destinations)),
                    _ => Box::new(Broadcast::new(name.clone(), destinations)),
                }
            })
            .collect();

        let empty: Vec<String> = vec![];
        for module in modules.iter_mut() {
            let name = module.name();
            module.reset_inputs(inputs.get(name).unwrap_or(&empty).clone());
        }

        Desert {
            modules,
            pulses: VecDeque::new(),
            pulses_seen_low: 0,
            pulses_seen_high: 0,
        }
    }

    fn get_module_mut(&mut self, name: &String) -> Option<&mut Box<dyn Module>> {
        self.modules.iter_mut().find(|module| module.name() == name)
    }

    fn send_pulses(&mut self, pulses: Vec<Pulse>) {
        for pulse in &pulses {
            if pulse.high {
                self.pulses_seen_high += 1
            } else {
                self.pulses_seen_low += 1
            }
        }
        self.pulses.extend(pulses.into_iter());
    }

    fn button_press(&mut self, probes: &mut HashMap<String, Vec<bool>>) -> (u32, u32) {
        self.send_pulses(vec![Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            high: false,
        }]);

        while !self.pulses.is_empty() {
            let pulse = self.pulses.pop_front().unwrap();
            if let Some(module) = self.get_module_mut(&pulse.to) {
                probes
                    .entry(pulse.from.clone())
                    .and_modify(|v| v.push(pulse.high));

                let pulses = module.pulse(pulse.high, pulse.from);
                self.send_pulses(pulses);
            }
        }

        (self.pulses_seen_low, self.pulses_seen_high)
    }

    fn find_min_button_presses_for_rx(&mut self) -> usize {
        let parent_name = self
            .modules
            .iter()
            .find(|&module| module.destinations().contains(&"rx".to_string()))
            .unwrap()
            .name();
        let grandparents = self
            .modules
            .iter()
            .filter_map(|module| {
                module
                    .destinations()
                    .contains(&parent_name)
                    .then_some(module.name().clone())
            })
            .collect::<Vec<_>>();

        let mut first_high_pulse = HashMap::<String, usize>::new();

        let mut pulse_probes: HashMap<String, Vec<bool>> = HashMap::new();
        for gparent in &grandparents {
            pulse_probes.insert(gparent.clone(), vec![]);
        }

        for n in 1.. {
            self.button_press(&mut pulse_probes);

            for (module, pulses) in &pulse_probes {
                if pulses.iter().any(|&pulse| pulse) {
                    first_high_pulse.entry(module.clone()).or_insert(n);
                }
            }
            pulse_probes.values_mut().for_each(|v| v.clear());

            if first_high_pulse.len() == grandparents.len() {
                break;
            }
        }

        let prime_factors = first_high_pulse
            .values()
            .map(|v| aoc_utils::prime_factors(&(*v as u32)))
            .collect::<Vec<_>>();
        let mut common_factors = vec![];
        for factor in prime_factors[0].keys() {
            if let Some(min_common) = prime_factors
                .iter()
                .map(|factors| factors.get(factor).unwrap_or(&0))
                .min()
            {
                common_factors.push((factor, *min_common));
            }
        }
        let divider = common_factors
            .iter()
            .filter_map(|(&factor, repeats)| (*repeats > 0).then_some(factor * repeats))
            .product::<u32>();
        let cycles = first_high_pulse
            .values()
            .map(|cycle| cycle / (divider as usize));

        cycles.product()
    }
}

trait Module {
    fn reset_inputs(&mut self, _inputs: Vec<String>) {}
    fn name(&self) -> &String;
    fn destinations(&self) -> &Vec<String>;
    fn pulse(&mut self, high: bool, from: String) -> Vec<Pulse>;
}
struct FlipFlop {
    name: String,
    is_on: bool,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(name: String, destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            name,
            is_on: false,
            destinations,
        }
    }
}

impl Module for FlipFlop {
    fn reset_inputs(&mut self, _inputs: Vec<String>) {}
    fn name(&self) -> &String {
        &self.name
    }
    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
    fn pulse(&mut self, high: bool, _from: String) -> Vec<Pulse> {
        if high {
            vec![]
        } else {
            self.is_on = !self.is_on;
            self.destinations
                .iter()
                .map(|dest| Pulse {
                    from: self.name.clone(),
                    to: dest.clone(),
                    high: self.is_on,
                })
                .collect()
        }
    }
}

struct Conjunction {
    name: String,
    inputs: HashMap<String, bool>,
    destinations: Vec<String>,
}
impl Conjunction {
    fn new(name: String, destinations: Vec<String>) -> Conjunction {
        Conjunction {
            name,
            inputs: HashMap::new(),
            destinations,
        }
    }
}

impl Module for Conjunction {
    fn reset_inputs(&mut self, inputs: Vec<String>) {
        self.inputs = inputs.iter().map(|input| (input.clone(), false)).collect();
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
    fn pulse(&mut self, high: bool, from: String) -> Vec<Pulse> {
        if !self.inputs.contains_key(&from) {
            panic!("Unknown input {}", from);
        }
        self.inputs.entry(from).and_modify(|h| *h = high);

        let send_high = !self.inputs.values().all(|&high| high);
        self.destinations
            .iter()
            .map(|dest| Pulse {
                from: self.name.clone(),
                to: dest.clone(),
                high: send_high,
            })
            .collect()
    }
}

struct Broadcast {
    name: String,
    destinations: Vec<String>,
}
impl Broadcast {
    fn new(name: String, destinations: Vec<String>) -> Broadcast {
        Broadcast { name, destinations }
    }
}

impl Module for Broadcast {
    fn name(&self) -> &String {
        &self.name
    }
    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
    fn pulse(&mut self, high: bool, _from: String) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|dest| Pulse {
                from: self.name.clone(),
                to: dest.clone(),
                high,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::ops::Deref;

    #[test]
    fn test_mine() {
        assert_eq!(execute(), "861743850 247023644760071");
    }

    #[test]
    fn test_from_lines() {
        let mut example1 = Desert::from_lines(vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ]);

        assert_eq!(5, example1.modules.len());
        assert_eq!(
            HashSet::from([
                "broadcaster".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "inv".to_string()
            ]),
            example1
                .modules
                .iter()
                .map(|m| m.name().clone())
                .collect::<HashSet<_>>()
        );
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string(),],
            *example1
                .get_module(&"broadcaster".to_string())
                .unwrap()
                .deref()
                .destinations()
        );
    }

    impl Desert {
        fn get_module(&mut self, name: &String) -> Option<&Box<dyn Module>> {
            self.modules.iter().find(|module| module.name() == name)
        }
    }

    #[test]
    fn test_button_press() {
        let mut example1 = Desert::from_lines(vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ]);

        let (mut seen_low, mut seen_high) = example1.button_press(&mut HashMap::new());
        assert_eq!(8, seen_low);
        assert_eq!(4, seen_high);

        for _ in 1..1000 {
            (seen_low, seen_high) = example1.button_press(&mut HashMap::new());
        }
        assert_eq!(8000, seen_low);
        assert_eq!(4000, seen_high);

        let mut example2 = Desert::from_lines(vec![
            "broadcaster -> a".to_string(),
            "%a -> inv, con".to_string(),
            "&inv -> b".to_string(),
            "%b -> con".to_string(),
            "&con -> output".to_string(),
        ]);
        let (mut seen_low, mut seen_high) = example2.button_press(&mut HashMap::new());
        for _ in 1..1000 {
            (seen_low, seen_high) = example2.button_press(&mut HashMap::new());
        }
        assert_eq!(4250, seen_low);
        assert_eq!(2750, seen_high);
    }

    #[test]
    fn test_pulse_flipflop() {
        let mut module = FlipFlop::new("foo".to_string(), vec!["bar".to_string()]);
        assert!(!module.is_on);
        assert_eq!(module.destinations.len(), 1);

        let pulses = module.pulse(true, "baz".to_string());
        assert!(!module.is_on);
        assert_eq!(pulses.len(), 0);

        let pulses = module.pulse(false, "baz".to_string());
        assert!(module.is_on);
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].from, "foo".to_string());
        assert_eq!(pulses[0].to, "bar".to_string());
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(true, "baz".to_string());
        assert!(module.is_on);
        assert_eq!(pulses.len(), 0);

        let pulses = module.pulse(false, "baz".to_string());
        assert!(!module.is_on);
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].from, "foo".to_string());
        assert_eq!(pulses[0].to, "bar".to_string());
        assert_eq!(pulses[0].high, false);
    }

    #[test]
    fn test_conjunction_pulse_one_input() {
        let mut module = Conjunction::new("inv".to_string(), vec!["out".to_string()]);
        module.reset_inputs(vec!["inp".to_string()]);

        let pulses = module.pulse(false, "inp".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].from, "inv".to_string());
        assert_eq!(pulses[0].to, "out".to_string());
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(false, "inp".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(true, "inp".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, false);

        let pulses = module.pulse(true, "inp".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, false);

        let pulses = module.pulse(false, "inp".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, true);
    }

    #[test]
    fn test_conjunction_pulse_multiple_input() {
        let mut module = Conjunction::new("conj".to_string(), vec!["out".to_string()]);
        module.reset_inputs(vec![
            "bim".to_string(),
            "bam".to_string(),
            "boom".to_string(),
        ]);

        let pulses = module.pulse(false, "bim".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].from, "conj".to_string());
        assert_eq!(pulses[0].to, "out".to_string());
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(true, "bim".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(true, "bam".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, true);

        let pulses = module.pulse(true, "boom".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, false);

        let pulses = module.pulse(false, "bam".to_string());
        assert_eq!(pulses.len(), 1);
        assert_eq!(pulses[0].high, true);
    }

    #[test]
    #[should_panic]
    fn test_conjunction_pulse_without_inputs() {
        let mut module = Conjunction::new("foo".to_string(), vec!["bar".to_string()]);
        module.reset_inputs(vec![]);
        module.pulse(false, "baz".to_string());
    }

    #[test]
    #[should_panic]
    fn test_conjunction_pulse_with_unknown_input() {
        let mut no_input = Conjunction::new("foo".to_string(), vec!["bar".to_string()]);
        no_input.reset_inputs(vec!["baz".to_string()]);
        no_input.pulse(false, "boo".to_string());
    }
}
