pub fn solve() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    println!("# Day 6");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut lines = input.trim().lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let races = times.iter().zip(distances.iter()).collect::<Vec<_>>();

    races.iter().map(|r| possible_wins(r)).product()
}

fn part2(input: &str) -> i64 {
    let mut lines = input.trim().lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, s| acc + s)
        .parse::<i64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, s| acc + s)
        .parse::<i64>()
        .unwrap();
    let race = &(&time, &distance);

    possible_wins(race)
}

fn possible_wins(race: &(&i64, &i64)) -> i64 {
    let (time, distance) = race;
    let mut wins = 0;

    for i in 1..**time {
        let speed = i;
        let actual_distance = speed * (**time - i);
        if actual_distance > **distance {
            wins += 1;
        }
    }
    wins
}
#[test]
fn test_possible_wins() {
    assert_eq!(possible_wins(&(&7, &9)), 4);
}

#[test]
fn test_part1() {
    let input = "
Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part1(input), 288);
}
