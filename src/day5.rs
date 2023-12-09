use std::ops::RangeInclusive;

pub fn solve() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("# Day 5");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    Mapping::from(input).find_min_location()
}

fn part2(input: &str) -> i64 {
    Mapping::from(input).find_min_location_with_ranges()
}

#[derive(Debug)]
struct Mapping {
    seeds: Vec<i64>,
    steps: Vec<Step>,
}

impl Mapping {
    fn find_min_location(&self) -> i64 {
        self.seeds
            .iter()
            .map(|seed| {
                self.steps
                    .iter()
                    .fold(*seed, |acc, step| step.source_to_destination(acc))
            })
            .min()
            .unwrap()
    }

    fn find_min_location_with_ranges(&self) -> i64 {
        let ranges = self.seed_ranges();
        let mut min_location = i64::MAX;

        for range in ranges {
            let mut seed = *range.start();
            while seed <= *range.end() {
                let (location, skip) = self.convert_and_skip(seed);
                min_location = min_location.min(location);
                seed += skip.max(1);
            }
        }
        min_location
    }

    fn convert_and_skip(&self, seed: i64) -> (i64, i64) {
        self.steps.iter().fold((seed, i64::MAX), |acc, step| {
            step.convert_and_skip(acc.0, acc.1)
        })
    }

    fn seed_ranges(&self) -> Vec<RangeInclusive<i64>> {
        let seeds: &[i64] = &self.seeds;
        let mut chunks: Vec<&[i64]> = seeds.chunks(2).collect();
        chunks.sort_by(|a, b| a[0].cmp(&b[0]));
        chunks.iter().map(|x| x[0]..=x[0] + x[1]).collect()
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let mut iter = value.trim().split("\n\n");
        let seeds = iter
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let steps = iter.map(|s| Step::from(s)).collect::<Vec<Step>>();
        Mapping { seeds, steps }
    }
}

#[derive(Debug)]
struct Step {
    rules: Vec<Rule>,
}

impl Step {
    fn source_to_destination(&self, source: i64) -> i64 {
        for rule in &self.rules {
            let result = rule.source_to_destination(source);
            if result != source {
                return result;
            }
        }
        source
    }

    fn convert_and_skip(&self, source: i64, current_skip: i64) -> (i64, i64) {
        for rule in &self.rules {
            let (result, skip) = rule.convert_and_skip(source);
            if result != source {
                return (result, skip.min(current_skip));
            }
        }
        (source, current_skip)
    }
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let rules = value
            .split(" map:")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split("\n")
            .map(|s| Rule::from_str(s))
            .collect::<Vec<Rule>>();

        Self { rules }
    }
}

#[derive(Debug)]
struct Rule {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

impl Rule {
    fn from_str(str: &str) -> Self {
        let mut parts = str.split_whitespace();
        let destination_start = parts.next().unwrap().parse::<i64>().unwrap();
        let source_start = parts.next().unwrap().parse::<i64>().unwrap();
        let range_length = parts.next().unwrap().parse::<i64>().unwrap();

        Self {
            destination_start,
            source_start,
            range_length,
        }
    }

    fn source_to_destination(&self, source: i64) -> i64 {
        let range = self.source_start..=self.source_start + self.range_length;
        if range.contains(&source) {
            return self.destination_start + (source - self.source_start);
        }
        source
    }

    fn convert_and_skip(&self, source: i64) -> (i64, i64) {
        let range = self.source_start..=self.source_start + self.range_length;
        if range.contains(&source) {
            return (
                self.destination_start + (source - self.source_start),
                self.range_length - (source - self.source_start),
            );
        }
        (source, 0)
    }
}

#[test]
fn test() {
    let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 46);
}
