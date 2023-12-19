use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();
    println!("# Day 19");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Operator {
    GreaterThan,
    LessThan,
    None,
}

#[derive(Debug, Clone)]
enum Operation {
    Accepted,
    Rejected,
    ProcessWith(String),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "A" => Operation::Accepted,
            "R" => Operation::Rejected,
            _ => Operation::ProcessWith(value.to_string()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process_part(&self, part: &Part) -> Operation {
        for rule in self.rules[..].iter() {
            if rule.operator == Operator::None {
                return rule.operation.clone();
            }
            let value = match rule.key {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => 0,
            };
            match rule.operator {
                Operator::GreaterThan => {
                    if value > rule.value {
                        return rule.operation.clone();
                    }
                }
                Operator::LessThan => {
                    if value < rule.value {
                        return rule.operation.clone();
                    }
                }
                _ => {}
            }
        }
        panic!("No rule found for part: {:?}", part);
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let mut parts = value.split('{');
        let name = parts.next().unwrap().to_string();
        let rules = parts
            .next()
            .unwrap()
            .trim_end_matches('}')
            .split(',')
            .map(Rule::from)
            .collect::<Vec<_>>();
        Workflow { name, rules }
    }
}

#[derive(Debug)]
struct Rule {
    operator: Operator,
    value: usize,
    key: char,
    operation: Operation,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        // 3 options

        // "a>1716:R"
        // "m<1801:hdj"
        // "A"
        // "hdj"

        // "[key][operator][value]:[operation]"
        // "[operation]

        // "px{a<2006:qkq,m>2090:A,rfg}"
        let mut chars = value.chars();
        if !chars.clone().any(|c| c == '>' || c == '<') {
            return Rule {
                operator: Operator::None,
                value: 0,
                key: '\0',
                operation: Operation::from(value),
            };
        }

        let key = chars.next().unwrap();
        let operator = match chars.next().unwrap() {
            '>' => Operator::GreaterThan,
            '<' => Operator::LessThan,
            _ => Operator::None,
        };
        let mut parts = value.split(['>', '<', ':']).skip(1);
        let value = parts.next().unwrap().parse::<usize>().unwrap();
        let operation = Operation::from(parts.next().unwrap());
        Rule {
            operator,
            value,
            key,
            operation,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        //{x=787,m=2655,a=1222,s=2876}
        let mut parts = value
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|s| s[2..].to_string().parse::<usize>().unwrap());

        let x = parts.next().unwrap();
        let m = parts.next().unwrap();
        let a = parts.next().unwrap();
        let s = parts.next().unwrap();
        Part { x, m, a, s }
    }
}

fn part1(input: &str) -> usize {
    let mut map: HashMap<&str, &Workflow> = HashMap::new();
    let mut strparts = input.trim().split("\n\n");
    let workflows = strparts
        .next()
        .unwrap()
        .lines()
        .map(Workflow::from)
        .collect::<Vec<_>>();
    let parts = strparts
        .next()
        .unwrap()
        .lines()
        .map(Part::from)
        .collect::<Vec<_>>();
    for workflow in workflows.iter() {
        map.insert(&workflow.name, &workflow);
    }
    let initial = map.get("in").unwrap();
    let mut sum = 0;

    for part in parts.iter() {
        let mut current = initial;
        loop {
            let operation = current.process_part(part);
            match operation {
                Operation::Accepted => {
                    sum += part.value();
                    break;
                }
                Operation::Rejected => {
                    break;
                }
                Operation::ProcessWith(name) => {
                    current = map.get(&name[..]).unwrap();
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    let input = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
    assert_eq!(part1(input), 19114);
    assert_eq!(part2(input), 0);
}
