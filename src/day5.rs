#[derive(Debug)]
struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    fn from_str(str: &str) -> Self {
        let rules = str
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

    fn source_to_destination(&self, source: i64) -> i64 {
        for rule in &self.rules {
            let result = rule.source_to_destination(source);
            if result != source {
                return result;
            }
        }
        source
    }

    fn destination_to_source(&self, destination: i64) -> i64 {
        for rule in &self.rules {
            let result = rule.destination_to_source(destination);
            if result != destination {
                return result;
            }
        }
        destination
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

    fn destination_to_source(&self, destination: i64) -> i64 {
        let range = self.destination_start..=self.destination_start + self.range_length;
        if range.contains(&destination) {
            return self.source_start + (destination - self.destination_start);
        }
        destination
    }
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("# Day 5");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut iter = input.trim().split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mappings = iter.map(|s| Mapping::from_str(s)).collect::<Vec<Mapping>>();

    let results = seeds.iter().map(|seed| {
        mappings
            .iter()
            .fold(*seed, |acc, mapping| mapping.source_to_destination(acc))
    });

    results.min().unwrap()
}

#[test]
fn test_part1() {
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
}

fn part2(input: &str) -> i64 {
    let mut iter = input.trim().split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let seed_ranges = seeds
        .chunks(2)
        .map(|x| x[0]..=x[0] + x[1])
        .collect::<Vec<std::ops::RangeInclusive<i64>>>();

    let mut mappings = iter.map(|s| Mapping::from_str(s)).collect::<Vec<Mapping>>();

    mappings.reverse();

    for i in 0..std::i64::MAX {
        let mut result = i;
        for mapping in mappings.iter() {
            result = mapping.destination_to_source(result);
        }

        if seed_ranges.iter().any(|range| range.contains(&result)) {
            mappings.reverse();

            return mappings
                .iter()
                .fold(result, |acc, mapping| mapping.source_to_destination(acc));
        }
    }

    0
}

#[test]
fn test_part2() {
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

    assert_eq!(part2(&input), 46);
}
