pub fn solve() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    let lines = input.trim().lines();
    println!("# Day 2");
    println!("Part 1: {}", part1(&lines.clone().collect()));
    println!("Part 2: {}", part2(&lines.collect()));
}

fn part1(lines: &Vec<&str>) -> i32 {
    let max = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    lines
        .iter()
        .map(|line| line_to_game(line))
        .map(|game| if game_valid(&game, &max) { game.id } else { 0 })
        .sum()
}

#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .lines()
        .collect();

    assert_eq!(part1(&input), 8);
}

fn part2(lines: &Vec<&str>) -> i32 {
    lines
        .iter()
        .map(|line| line_to_game(line))
        .map(|game| game_minimal_set(&game))
        .map(|set| set_power(&set))
        .sum()
}

#[test]
fn test_part2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .lines()
        .collect();

    assert_eq!(part2(&input), 2286);
}

#[derive(Clone, Debug)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

fn game_valid(game: &Game, max: &CubeSet) -> bool {
    game.sets
        .iter()
        .all(|set| set.red <= max.red && set.green <= max.green && set.blue <= max.blue)
}

fn game_minimal_set(game: &Game) -> CubeSet {
    let mut min = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in game.sets.iter() {
        min.red = set.red.max(min.red);
        min.green = set.green.max(min.green);
        min.blue = set.blue.max(min.blue);
    }
    min
}

fn set_power(set: &CubeSet) -> i32 {
    set.red.max(1) * set.green.max(1) * set.blue.max(1)
}

fn line_to_game(line: &str) -> Game {
    let mut parts = line.split(": ");
    let id = parts
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let sets: Vec<_> = parts
        .next()
        .unwrap()
        .split("; ")
        .map(|set| line_to_set(set))
        .collect();

    Game { id, sets }
}

fn line_to_set(line: &str) -> CubeSet {
    let mut parts = line.split(", ");
    let mut set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    while let Some(part) = parts.next() {
        let mut count_color = part.split(" ");
        let count = count_color.next().unwrap().parse::<i32>().unwrap();
        let color = count_color.next().unwrap();
        match color {
            "red" => set.red = count,
            "green" => set.green = count,
            "blue" => set.blue = count,
            _ => panic!("Unknown color: {}", color),
        }
    }
    set
}
