use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("# Day 8");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Address = (char, char, char);

#[derive(Debug)]

struct Node {
    label: Address,
    left: Address,
    right: Address,
}

impl Node {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars().filter(|c| c.is_alphanumeric());
        let label = (
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        );
        let left = (
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        );
        let right = (
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        );
        Self { label, left, right }
    }

    fn navigate(&self, direction: &char) -> &Address {
        match direction {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Invalid direction"),
        }
    }

    fn is_start(&self) -> bool {
        match self.label {
            (_, _, 'A') => true,
            _ => false,
        }
    }

    fn is_end(&self) -> bool {
        match self.label {
            (_, _, 'Z') => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<Address, Node>,
    starts: Vec<Address>,
}

impl Network {
    fn from_str(s: &str) -> Self {
        let mut nodes = HashMap::new();
        let mut starts = Vec::new();
        for line in s.lines() {
            let node = Node::from_str(line);
            if node.is_start() {
                starts.push(node.label);
            }
            nodes.insert(node.label, node);
        }
        Self { nodes, starts }
    }

    fn get(&self, address: &Address) -> &Node {
        self.nodes.get(address).unwrap()
    }
}

fn part1(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");
    let mut instructions = parts.next().unwrap().chars().cycle();
    let network = Network::from_str(parts.next().unwrap());
    let mut current = network.get(&('A', 'A', 'A'));
    let mut steps = 0;
    loop {
        let direction = instructions.next().unwrap();
        current = network.get(current.navigate(&direction));
        steps += 1;
        if current.label == ('Z', 'Z', 'Z') {
            break;
        }
    }

    steps
}

#[test]
fn test_part1() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(input), 6);
}

fn part2(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");
    let mut instructions = parts.next().unwrap().chars().cycle().peekable();
    let network = Network::from_str(parts.next().unwrap());
    let starts = network.starts.to_owned();
    let mut current = starts.clone();
    let mut steps: i64 = 0;
    let mut counts = Vec::new();

    for i in 0..starts.len() {
        loop {
            let node = network.get(&current[i]);

            if node.is_end() {
                counts.push(steps);
                steps = 0;
                break;
            }

            let direction = instructions.next().unwrap();
            current[i] = *node.navigate(&direction);
            steps += 1;
        }
    }

    counts.iter().cloned().reduce(|a, b| lcm(a, b)).unwrap()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a.abs();
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[test]
fn test_part2() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX) ";
    assert_eq!(part2(input), 6);
}

