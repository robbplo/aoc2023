use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();
    println!("# Day 15");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn hash(input: &str) -> usize {
    let mut h = 0;
    for c in input.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut map: HashMap<usize, VecDeque<(String, usize)>> = HashMap::new();
    input.trim().split(',').for_each(|seq| {
        let op_idx = seq.find(['=', '-']).unwrap();
        let mut split = seq.split(['=', '-']);
        let label = split.next().unwrap();
        let focal_length = split.next().unwrap().parse::<usize>().unwrap_or(0);
        let entry = map.entry(hash(label)).or_insert_with(VecDeque::new);
        match seq.chars().nth(op_idx).unwrap() {
            '=' => {
                let existing = entry.iter().enumerate().find(|(i, val)| val.0 == label);
                if let Some((i, val)) = existing {
                    entry.remove(i);
                    entry.insert(i, (label.to_string(), focal_length));
                } else {
                    entry.push_back((label.to_string(), focal_length));
                }
            }
            '-' => {
                for (i, val) in entry.iter().enumerate() {
                    if val.0 == label {
                        entry.remove(i);
                        break;
                    }
                }
            }
            _ => panic!("Invalid input"),
        }
    });

    map.iter()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, val)| (i + 1) * (j + 1) * val.1)
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
    assert_eq!(part2(input), 145);
}
