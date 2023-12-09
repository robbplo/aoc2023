pub fn solve() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("# Day 9");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .map(|nums| build_sequence(&nums))
        .map(|seq| extrapolate_next(&seq))
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .map(|nums| build_sequence(&nums))
        .map(|seq| extrapolate_back(&seq))
        .sum()
}

#[test]
fn test() {
    let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part1(input), 114);
    assert_eq!(part2(input), 2);
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn build_sequence(numbers: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequence: Vec<Vec<i64>> = vec![numbers.to_vec()];
    let mut curr = 0;
    loop {
        let line = next_line(&sequence[curr]);
        sequence.push(line);

        if sequence[curr + 1].iter().all(|n| n == &0) {
            break;
        }
        curr += 1;
    }
    sequence.reverse();
    sequence
}

fn next_line(line: &Vec<i64>) -> Vec<i64> {
    line.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn extrapolate_next(sequence: &Vec<Vec<i64>>) -> i64 {
    sequence.iter().map(|nums| nums.last().unwrap()).sum()
}

fn extrapolate_back(sequence: &Vec<Vec<i64>>) -> i64 {
    sequence
        .iter()
        .map(|nums| nums[0])
        .reduce(|acc, num| num - acc)
        .unwrap()
}
