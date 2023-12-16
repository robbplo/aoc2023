use std::collections::HashMap;

use super::grid2d::*;

pub fn solve() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("# Day 14");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn hash(grid: &Grid2D<char>) -> String {
    grid.find_all('O')
        .iter()
        .fold(String::new(), |mut acc, &(x, y)| {
            acc.push_str(&format!("{:02}{:02}", x, y));
            acc
        })
}
fn roll_cycle(grid: &mut Grid2D<char>) {
    roll(grid, Bearing::North);
    roll(grid, Bearing::West);
    roll(grid, Bearing::South);
    roll(grid, Bearing::East);
}
fn last_free(grid: &Grid2D<char>, start: Point, bearing: Bearing) -> Option<Point> {
    let mut next = bearing.offset_point(start);
    let mut last_free = None;
    loop {
        match grid.get_opt(next) {
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
fn roll(grid: &mut Grid2D<char>, bearing: Bearing) {
    let mut rocks = grid.find_all('O');
    rocks
        .iter_mut()
        .for_each(|rock| match last_free(grid, *rock, bearing) {
            Some(last_free) => {
                grid.set(last_free, 'O');
                grid.set(*rock, '.');
            }
            None => (),
        });
}
fn north_load(grid: &Grid2D<char>) -> usize {
    let rocks = grid.find_all('O');
    let height = grid.height();
    rocks.iter().map(|rock| height - rock.1).sum()
}

fn part1(input: &str) -> usize {
    let mut grid = Grid2D::from(input);
    roll(&mut grid, Bearing::North);
    north_load(&mut grid)
}

fn part2(input: &str) -> usize {
    let mut target = 1e9 as usize;
    let mut cache_hits: HashMap<String, usize> = HashMap::new();
    let grid = &mut Grid2D::from(input);
    while cache_hits.values().all(|x| *x <= 2) {
        target -= 1;
        *cache_hits.entry(hash(grid)).or_insert(0) += 1;
        roll_cycle(grid);
    }
    let cycle = cache_hits.values().filter(|x| **x > 1).count();
    for _ in 0..(target % cycle) {
        roll_cycle(grid);
    }
    north_load(&grid)
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
