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
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|nums| build_sequence(&nums))
        .map(|seq| extrapolate_next(&seq))
        .sum()
}

#[test]
fn test_part1() {
    let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part1(input), 114);
}

fn build_sequence(numbers: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequence: Vec<Vec<i64>> = vec![numbers.to_vec()];
    let mut curr = 0;
    loop {
        let next = sequence[curr]
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();

        sequence.push(next.clone());
        if next.iter().all(|n| n == &0) {
            break;
        }
        curr += 1;
    }

    sequence
}

fn extrapolate_next(sequence: &Vec<Vec<i64>>) -> i64 {
    sequence.iter().map(|nums| nums.last().unwrap()).sum()
}

fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|nums| build_sequence(&nums))
        .map(|seq| extrapolate_back(&seq))
        .sum()
}

fn extrapolate_back(sequence: &Vec<Vec<i64>>) -> i64 {
    let mut extrapolated = vec![0; sequence.len()];
    let first_numbers = sequence
        .iter()
        .map(|nums| nums[0])
        .rev()
        .collect::<Vec<i64>>();

    for i in 0..sequence.len() - 1 {
        extrapolated[i + 1] = first_numbers[i + 1] - extrapolated[i]
    }

    *extrapolated.last().unwrap()
}

#[test]
fn test_part2() {
    let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part2(input), 2);
}
