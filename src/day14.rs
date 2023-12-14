use std::collections::HashMap;

use super::grid2d::*;

pub fn solve() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("# Day 14");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

impl Grid2D<char> {
    fn hash(&self) -> String {
        self.find_all('O')
            .iter()
            .fold(String::new(), |mut acc, &(x, y)| {
                acc.push_str(&format!("{:02}{:02}", x, y));
                acc
            })
    }
    fn roll_cycle(&mut self) {
        self.roll(Bearing::North);
        self.roll(Bearing::West);
        self.roll(Bearing::South);
        self.roll(Bearing::East);
    }
    fn last_free(&self, start: Point, bearing: Bearing) -> Option<Point> {
        let mut next = bearing.offset_point(start);
        let mut last_free = None;
        loop {
            match self.get_opt(next) {
                Some('.') => {
                    last_free = next;
                    next = bearing.offset_point(next.unwrap());
                }
                Some('O') => next = bearing.offset_point(next.unwrap()),
                Some('#') => break,
                _ => break,
            }
        }
        last_free
    }
    fn roll(&mut self, bearing: Bearing) {
        let mut rocks = self.find_all('O');

        rocks
            .iter_mut()
            .for_each(|rock| match self.last_free(*rock, bearing) {
                Some(last_free) => {
                    self.set(last_free, 'O');
                    self.set(*rock, '.');
                }
                None => (),
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
    let mut target = 1e9 as usize;
    let mut cache_hits: HashMap<String, usize> = HashMap::new();
    let mut grid = Grid2D::from(input);
    while cache_hits.values().all(|x| *x <= 2) {
        target -= 1;
        *cache_hits.entry(grid.hash()).or_insert(0) += 1;
        grid.roll_cycle();
    }
    let cycle = cache_hits.values().filter(|x| **x > 1).count();
    for _ in 0..(target % cycle) {
        grid.roll_cycle();
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
    assert_eq!(part2(input), 64);
}
