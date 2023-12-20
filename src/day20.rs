use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let input = std::fs::read_to_string("input/day20.txt").unwrap();
    println!("# Day 20");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    label: String,
    is_on: bool,
    kind: ModuleKind,
    con_state: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl Module {
    fn receive(&mut self, pulse: &Pulse, from: String) -> Vec<(String, Pulse)> {
        match self.kind {
            ModuleKind::Broadcaster => self.broadcast(pulse),
            ModuleKind::FlipFlop => self.flip_flop(pulse),
            ModuleKind::Conjunction => self.conjunction(pulse, from),
        }
    }
    fn broadcast(&self, pulse: &Pulse) -> Vec<(String, Pulse)> {
        self.outputs
            .iter()
            .map(|label| (label.clone(), pulse.clone()))
            .collect()
    }
    fn flip_flop(&mut self, pulse: &Pulse) -> Vec<(String, Pulse)> {
        let mut outputs = Vec::new();
        match pulse {
            Pulse::Low => {
                let send_pulse = match self.is_on {
                    false => {
                        self.is_on = true;
                        Pulse::High
                    }
                    true => {
                        self.is_on = false;
                        Pulse::Low
                    }
                };
                for label in &self.outputs {
                    outputs.push((label.clone(), send_pulse.clone()));
                }
            }
            Pulse::High => (),
        }
        outputs
    }

    fn conjunction(&mut self, pulse: &Pulse, from: String) -> Vec<(String, Pulse)> {
        let mut outputs = Vec::new();
        let mut send = Pulse::High;
        self.con_state.insert(from, pulse.clone());
        if self.con_state.values().all(|p| *p == Pulse::High) {
            send = Pulse::Low;
        };
        for label in &self.outputs {
            outputs.push((label.clone(), send.clone()));
        }
        outputs
    }
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let mut parts = value.split(" -> ");
        let mut label_chars = parts.next().unwrap().chars().peekable();
        let kind = match label_chars.peek() {
            Some('%') => {
                label_chars.next();
                ModuleKind::FlipFlop
            }
            Some('&') => {
                label_chars.next();
                ModuleKind::Conjunction
            }
            _ => ModuleKind::Broadcaster,
        };
        let outputs: Vec<String> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let con_state = HashMap::new();
        // if kind == ModuleKind::Conjunction {
        //     for label in outputs.iter() {
        //         con_state.insert(label.clone(), Pulse::Low);
        //     }
        // }
        Module {
            label: label_chars.collect::<String>(),
            is_on: false,
            kind,
            con_state,
            outputs,
        }
    }
}

#[derive(Debug)]
struct ModuleMap {
    modules: HashMap<String, Module>,
    queue: VecDeque<(String, String, Pulse)>,
    high_count: usize,
    low_count: usize,
}

impl From<&str> for ModuleMap {
    fn from(value: &str) -> Self {
        let mut modules: HashMap<String, Module> = HashMap::new();
        value
            .trim()
            .lines()
            .map(|line| Module::from(line))
            .for_each(|module| {
                modules.insert(module.label.clone(), module);
            });
        let queue = VecDeque::new();
        ModuleMap {
            modules,
            queue,
            high_count: 0,
            low_count: 0,
        }
    }
}

impl ModuleMap {
    fn push_button(&mut self) {
        let initial = "broadcaster".to_string();
        let broadcaster = self.modules.get_mut(&initial).unwrap();
        let pulse = Pulse::Low;
        self.queue.push_back(("button".to_string(), initial, pulse));
        while let Some((from, to, pulse)) = self.queue.pop_front() {
            println!("{} -{:?}-> {}", from, pulse, to);
            self.inc_pulse(&pulse);
            let module = match self.modules.get_mut(&to) {
                Some(module) => module,
                None => continue,
            };
            let outputs = module.receive(&pulse, from);
            for (label, pulse) in outputs {
                self.queue.push_back((module.label.clone(), label, pulse));
            }
        }
    }

    fn send(&mut self, label: &str, pulse: Pulse) {
        let module = self.modules.get_mut(label).unwrap();
        module.is_on = match pulse {
            Pulse::High => true,
            Pulse::Low => false,
        };
    }

    fn inc_pulse(&mut self, pulse: &Pulse) {
        match *pulse {
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count += 1,
        }
    }
    fn set_init_con_states(&mut self) {
        let con_mods = self.con_labels();
        for con_label in con_mods {
            let con_inputs = self.get_con_inputs(&con_label);
            let con = self.modules.get_mut(&con_label).unwrap();
            for input in con_inputs {
                con.con_state.insert(input.clone(), Pulse::Low);
            }
        }
    }
    fn con_labels(&self) -> Vec<String> {
        let mut labels = Vec::new();
        for module in self.modules.values() {
            if module.kind == ModuleKind::Conjunction {
                labels.push(module.label.clone());
            }
        }
        labels
    }
    fn get_con_inputs(&self, label: &str) -> Vec<String> {
        let mut inputs = Vec::new();
        for module in self.modules.values() {
            if module.outputs.contains(&label.to_string()) {
                inputs.push(module.label.clone());
            }
        }
        inputs
    }
}

fn part1(input: &str) -> usize {
    let mut map = ModuleMap::from(input);
    map.set_init_con_states();

    for _ in 0..1000 {
        map.push_button();
    }

    map.high_count * map.low_count
}

fn part2(input: &str) -> usize {
    let mut map = ModuleMap::from(input);
    map.set_init_con_states();

    for _ in 0..1000 {
        map.push_button();
    }

    map.high_count * map.low_count
}

#[test]
fn test() {
    let input1 = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
    //assert_eq!(part1(input1), 32000000);

    let input2 = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
    assert_eq!(part1(input2), 11687500);
    assert_eq!(part2(input1), 0);
}
