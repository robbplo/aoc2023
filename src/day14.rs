use std::collections::HashMap;

use super::grid2d::Grid2D;
use crate::grid2d::Bearing;

pub fn solve() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("# Day 14");
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
}

impl Grid2D<char> {
    fn hash(&self) -> String {
        self.find_all('O')
            .iter()
            .fold(String::new(), |mut acc, &point| {
                acc.push_str(&format!("{:02}{:02}", point.0, point.1));
                acc
            })
    }
    fn cycle_cached(&mut self, cache: &mut HashMap<String, Self>) -> bool {
        let hash = self.hash();
        if let Some(grid) = cache.get(&hash) {
            *self = grid.clone();
            return true;
        }
        self.roll_cycle();
        cache.insert(hash, self.clone());
        false
    }
    fn roll_cycle(&mut self) {
        self.roll(Bearing::North);
        self.roll(Bearing::West);
        self.roll(Bearing::South);
        self.roll(Bearing::East);
    }
    fn roll(&mut self, bearing: Bearing) {
        let mut rocks = self.find_all('O');

        rocks.iter_mut().for_each(|rock| {
            println!("moving {:?}", rock);
            let mut next = bearing.offset_point(*rock);
            loop {
                println!("next {:?}, value {:?}", next, self.get_opt(next));
                match self.get_opt(next) {
                    None | Some('#') => {
                        break;
                    }
                    Some('.') => {
                        self.set(*rock, '.');
                        *rock = next.unwrap();
                        next = bearing.offset_point(next.unwrap());
                        self.set(next.unwrap(), 'O');
                        continue;
                    }
                    Some('O') => {
                        next = bearing.offset_point(next.unwrap());
                        continue;
                    }
                    _ => {}
                }
            }
        });
    }
    fn north_load(&self) -> usize {
        let rocks = self.find_all('O');
        let height = self.height();
        rocks.iter().map(|rock| height - rock.1).sum()
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid2D::from(input);
    grid.roll(Bearing::North);
    grid.north_load()
}

fn part2(input: &str) -> usize {
    let target = 1e9 as usize;
    let mut current = 0;
    let cycle = 0;
    let mut cache: HashMap<String, Grid2D<char>> = HashMap::new();
    let mut grid = Grid2D::from(input);
    loop {
        current += 1;
        grid.roll_cycle();
        println!("{}", grid);
        if current == 100 {
            break;
        }
    }

    grid.north_load()
}

#[test]
fn test() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    assert_eq!(part1(input), 136);
    //assert_eq!(part2(input), 64);
}

#[test]
fn roll_test() {
    let input = "OO.#O....O";

    let mut grid = Grid2D::from(input);
    grid.roll(Bearing::East);
    assert_eq!(grid.to_string(), ".OO#....OO");
}
