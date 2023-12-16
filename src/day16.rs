use std::collections::{HashSet, VecDeque};

use super::grid2d::*;

pub fn solve() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("# Day 16");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn traverse(grid: &Grid2D<char>, start: Point, start_dir: Bearing) -> usize {
    let mut visited: HashSet<(Point, Bearing)> = HashSet::new();
    let mut points: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, Bearing)> = VecDeque::from(vec![(start, start_dir)]);
    while let Some((pos, dir)) = queue.pop_front() {
        if visited.contains(&(pos, dir)) {
            continue;
        }
        if let Some(c) = grid.get(pos) {
            visited.insert((pos, dir));
            points.insert(pos);
            for d in char_paths(*c, dir) {
                if let Some(p) = d.offset_point(pos) {
                    queue.push_back((p, d))
                }
            }
        }
    }

    points.len()
}

fn char_paths(c: char, dir: Bearing) -> Vec<Bearing> {
    match (c, dir) {
        ('.', _) => vec![dir],
        ('-', Bearing::East | Bearing::West) => vec![dir],
        ('|', Bearing::North | Bearing::South) => vec![dir],
        ('-', Bearing::North | Bearing::South) => vec![Bearing::East, Bearing::West],
        ('|', Bearing::East | Bearing::West) => vec![Bearing::North, Bearing::South],
        ('\\', Bearing::East) => vec![Bearing::South],
        ('\\', Bearing::South) => vec![Bearing::East],
        ('\\', Bearing::West) => vec![Bearing::North],
        ('\\', Bearing::North) => vec![Bearing::West],
        ('/', Bearing::East) => vec![Bearing::North],
        ('/', Bearing::South) => vec![Bearing::West],
        ('/', Bearing::West) => vec![Bearing::South],
        ('/', Bearing::North) => vec![Bearing::East],
        _ => panic!("Unexpected char: {}", c),
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid2D::from(input);
    traverse(&grid, (0, 0), Bearing::East)
}

fn part2(input: &str) -> usize {
    let grid = Grid2D::from(input);
    let mut max_points = 0;
    for x in 0..grid.width() {
        let points = traverse(&grid, (x, 0), Bearing::South);
        if points > max_points {
            max_points = points;
        }
        let points = traverse(&grid, (x, grid.height() - 1), Bearing::North);
        if points > max_points {
            max_points = points;
        }
    }
    for y in 0..grid.height() {
        let points = traverse(&grid, (0, y), Bearing::East);
        if points > max_points {
            max_points = points;
        }
        let points = traverse(&grid, (grid.width() - 1, y), Bearing::West);
        if points > max_points {
            max_points = points;
        }
    }

    max_points
}

#[test]
fn test() {
    let input = "
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";

    assert_eq!(part1(input), 46);
    assert_eq!(part2(input), 51);
}
