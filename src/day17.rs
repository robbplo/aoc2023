pub fn solve() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    println!("# Day 17");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut universe = Universe::from(input);

    universe.expand();
    let combinations = universe.galaxies.iter().enumerate().flat_map(|(i, a)| {
        universe
            .galaxies
            .iter()
            .enumerate()
            .filter(move |(j, _)| *j > i)
            .map(move |(_, b)| (a, b))
    });
    combinations
        .map(|(a, b)| universe.manhattan_distance(a, b))
        .sum()
}

fn part2(input: &str, expand_by: usize) -> i64 {
    let mut universe = Universe::from(input);
    universe.expand_by(expand_by);
    let combinations = universe.galaxies.iter().enumerate().flat_map(|(i, a)| {
        universe
            .galaxies
            .iter()
            .enumerate()
            .filter(move |(j, _)| j > &i)
            .map(move |(_, b)| (a, b))
    });
    combinations
        .map(|(a, b)| universe.manhattan_distance(a, b))
        .sum()
}

#[test]
fn test() {
    let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(part1(input), 374);
    assert_eq!(part2(input, 10), 1030);
    assert_eq!(part2(input, 100), 8410);
}
