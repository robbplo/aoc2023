use crate::grid2d::*;
use core::hash::{Hash, Hasher};
use std::collections::{BinaryHeap, HashMap};

pub fn solve() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    println!("# Day 17");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, Eq)]
struct State {
    position: Point,
    cost: usize,
    prev_steps: [Bearing; 3],
}

impl State {
    fn new(position: Point, cost: usize) -> Self {
        Self {
            position,
            cost,
            prev_steps: [Bearing::North, Bearing::North, Bearing::North],
        }
    }

    fn next(prev: &Self, position: Point, cost: usize, direction: Bearing) -> Self {
        let mut next = Self::new(position, cost);
        next.prev_steps = [prev.prev_steps[1], prev.prev_steps[2], direction];
        next
    }

    fn direction_is_valid(&self, direction: Bearing) -> bool {
        self.prev_steps[0] != direction
            || self.prev_steps[1] != direction
            || self.prev_steps[2] != direction
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.prev_steps.cmp(&other.prev_steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.prev_steps == other.prev_steps
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.prev_steps.hash(state);
    }
}

fn shortest_path(grid: &Grid2D<u8>, start: Point, end: Point) -> usize {
    let mut dist = HashMap::<State, usize>::with_capacity(grid.len());
    let mut queue = BinaryHeap::new();
    queue.push(State::new(start, 0));
    while let Some(state) = queue.pop() {
        let State { position, cost, .. } = state;
        if position == end {
            dbg!(&dist);
            return cost;
        }
        if dist.contains_key(&state) {
            continue;
        }
        dist.insert(state, cost);
        for bearing in &[Bearing::North, Bearing::East, Bearing::South, Bearing::West] {
            if !state.direction_is_valid(*bearing) {
                continue;
            }
            if let Some(next_pos) = bearing.offset_point(position) {
                if let Some(next_cost) = grid.get(next_pos) {
                    let next = State::next(&state, next_pos, cost + *next_cost as usize, *bearing);
                    queue.push(next);
                }
            }
        }
    }
    panic!("No path found");
}

fn part1(input: &str) -> usize {
    let grid = Grid2D::<u8>::from(input);
    shortest_path(&grid, (0, 0), (grid.width() - 1, grid.height() - 1))
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    let smol = "
2413432311323
3215453535623
";
    let grid = Grid2D::<u8>::from(smol);
    //assert_eq!(shortest_path(&grid, (0, 0), (5, 0)), 23);

    let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    assert_eq!(part1(input), 102);
    assert_eq!(part2(input), 0);
}
