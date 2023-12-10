use std::{collections::HashSet, fmt::Display};

pub fn solve() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("# Day 10");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const START: char = 'S';

type Position = (isize, isize);

#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn replace_non_loop_chars(&mut self) {
        let loop_positions: HashSet<Position> = self.get_loop_positions().into_iter().collect();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if !loop_positions.contains(&(x as isize, y as isize)) {
                    self.map[y][x] = '.'
                }
            }
        }
    }

    fn get_loop_positions(&self) -> Vec<Position> {
        let mut direction = self.start_directions()[0];
        let mut position = self.start;
        let mut result = vec![];

        loop {
            let (next_pos, next_dir) = self.traverse(&position, &direction);
            result.push(next_pos);
            position = next_pos;
            direction = next_dir;
            if self.get(&next_pos) == Some(&START) {
                break;
            }
        }
        result
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

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.iter() {
            let mut buf = String::new();
            for char in line {
                buf.push(*char)
            }
            writeln!(f, "{}", buf)?;
        }
        Ok(())
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .trim()
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
    let map = Map::from(input);
    map.get_loop_positions().len() as i32 / 2
}

fn part2(input: &str) -> i32 {
    let mut map = Map::from(input);
    map.replace_non_loop_chars();
    let mut count = 0;
    let include_start = map.start_directions().contains(&Direction::North);
    for row in map.map {
        let mut in_loop = false;
        for char in row {
            match char {
                '|' | 'J' | 'L' => in_loop = !in_loop,
                'S' => {
                    if include_start {
                        in_loop = !in_loop
                    }
                }
                '.' => {
                    if in_loop {
                        count += 1
                    }
                }
                _ => (),
            }
        }
    }

    count
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

    let input2 = "
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
";

    let input3 = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
    assert_eq!(part2(input2), 4);
    assert_eq!(part2(input3), 10);
}
