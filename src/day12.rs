use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEXA: Regex = Regex::new("#+").unwrap();
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("# Day 12");
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
}

struct Row {
    springs: String,
    groups: Vec<usize>,
}

impl Row {
    fn possible_permutations(&self) -> usize {
        let mut permutations: usize = 0;
        let string = &self.springs;
        self.make_permutations(string, &mut permutations);

        permutations
    }

    fn make_permutations(&self, string: &str, acc: &mut usize) {
        if let Some(false) = self.partial_validate(string) {
            return;
        }
        let index = string.find('?');
        if index.is_none() {
            if self.validate_permutation(string) {
                *acc += 1;
            }
            return;
        }
        self.make_permutations(&string.replacen('?', "#", 1), acc);
        self.make_permutations(&string.replacen('?', ".", 1), acc);
    }

    fn validate_permutation(&self, string: &str) -> bool {
        let captures = REGEXA
            .find_iter(&string)
            .map(|m| m.as_str().len())
            .collect::<Vec<_>>();
        captures == self.groups
    }

    fn partial_validate(&self, string: &str) -> Option<bool> {
        let trim = string.trim_matches('.');
        if trim.starts_with('#') {
            let first = REGEXA.find_iter(trim).map(|m| m.as_str().len()).next();

            if first > self.groups.first().copied() {
                return Some(false);
            }
        }

        return None;
    }

    fn multiply(&mut self) {
        self.springs.push('?');
        self.springs = self.springs.repeat(5);
        self.groups = self.groups.repeat(5);
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        let springs = split.next().unwrap().to_string();
        let groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Self { springs, groups }
    }
}

fn part1(input: &str) -> usize {
    let rows: Vec<Row> = input.trim().lines().map(Row::from).collect();
    rows.iter().map(Row::possible_permutations).sum()
}

fn part2(input: &str) -> usize {
    let mut rows: Vec<Row> = input.trim().lines().map(Row::from).collect();
    rows.iter_mut().for_each(Row::multiply);
    rows.iter().map(Row::possible_permutations).sum()
}

#[test]
fn test() {
    let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    assert_eq!(part1(input), 21);
    //assert_eq!(part2(input), 525152);
}
