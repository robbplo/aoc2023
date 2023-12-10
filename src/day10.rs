use std::{collections::HashSet, vec};

pub fn solve() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("# Day 10");
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
}

const START: char = 'S';

type Position = (isize, isize);

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    start: Position,
}

impl Map {
    fn get(&self, pos: &Position) -> Option<&char> {
        let (x, y) = pos;
        if *x < 0 || *y < 0 {
            return None;
        }
        self.map
            .get(*y as usize)
            .and_then(|line| line.get(*x as usize))
    }

    fn traverse(&self, pos: &Position, direction: &Direction) -> (Position, Direction) {
        let next_pos = match direction {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::West => (pos.0 - 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::East => (pos.0 + 1, pos.1),
        };

        match self.get(&next_pos) {
            Some(curr) => (next_pos, self.char_direction(curr, direction)),
            None => panic!("Invalid position: {:?}", pos),
        }
    }
    fn char_direction(&self, char: &char, dir: &Direction) -> Direction {
        match char {
            'F' => match dir {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!("Invalid movement: {}, {:?}", char, dir),
            },
            '7' => match dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!("Invalid movement: {}, {:?}", char, dir),
            },
            'L' => match dir {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!("Invalid movement: {}, {:?}", char, dir),
            },
            'J' => match dir {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                _ => panic!("Invalid movement: {}, {:?}", char, dir),
            },
            _ => *dir,
        }
    }

    fn start_directions(&self) -> Vec<Direction> {
        let mut result = Vec::new();
        let (x, y) = self.start;
        match self.get(&(x - 1, y)) {
            Some('-') | Some('F') | Some('L') => result.push(Direction::West),
            _ => (),
        }
        match self.get(&(x + 1, y)) {
            Some('-') | Some('7') | Some('J') => result.push(Direction::East),
            _ => (),
        }
        match self.get(&(x, y - 1)) {
            Some('|') | Some('F') | Some('7') => result.push(Direction::North),
            _ => (),
        }
        match self.get(&(x, y + 1)) {
            Some('|') | Some('L') | Some('J') => result.push(Direction::South),
            _ => (),
        }
        result
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let (start_y, start_line) = map
            .iter()
            .enumerate()
            .find(|(_, line)| line.contains(&START))
            .unwrap();
        let start_x = start_line.iter().position(|&c| c == START).unwrap();
        let start = (start_x as isize, start_y as isize);

        Self { map, start }
    }
}

fn part1(input: &str) -> i32 {
    let map = Map::from(input.trim());
    let mut direction = map.start_directions()[0];
    let mut position = map.start;
    let mut steps = 0;

    loop {
        steps += 1;
        let (next_pos, next_dir) = map.traverse(&position, &direction);
        position = next_pos;
        direction = next_dir;
        if map.get(&next_pos) == Some(&START) {
            return steps / 2;
        }
    }
}

fn part2(input: &str) -> i32 {
    0
}

#[test]
fn test() {
    let input = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
    assert_eq!(part1(input), 8);
    //assert_eq!(part2(input), 2);
}
