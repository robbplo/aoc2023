pub fn solve() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("# Day 13");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Line = Vec<char>;

#[derive(Debug)]
struct Map {
    rows: Vec<Line>,
    cols: Vec<Line>,
}

fn hamming_distance(a: &Line, b: &Line) -> usize {
    a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
}

#[derive(Debug, PartialEq)]
enum Reflection {
    Full,
    Smudge,
}

enum Direction {
    Vertical,
    Horizontal,
}

impl Map {
    fn reflection_at(&self, idx: usize, dir: Direction) -> Option<Reflection> {
        let collection = match dir {
            Direction::Vertical => &self.cols,
            Direction::Horizontal => &self.rows,
        };
        let mut smudge_found = false;

        let zip = (0..=idx).rev().zip((idx + 1)..collection.len());
        for (left, right) in zip {
            match hamming_distance(&collection[left], &collection[right]) {
                0 => {}
                1 => {
                    if smudge_found {
                        return None;
                    }
                    smudge_found = true;
                }
                _ => return None,
            }
        }
        if smudge_found {
            return Some(Reflection::Smudge);
        }
        Some(Reflection::Full)
    }
    fn reflection_score(&self, target: Reflection) -> usize {
        for i in 0..self.cols.len() - 1 {
            match self.reflection_at(i, Direction::Vertical) {
                Some(found) if found == target => return i + 1,
                _ => (),
            }
        }
        for i in 0..self.rows.len() - 1 {
            match self.reflection_at(i, Direction::Horizontal) {
                Some(found) if found == target => return (i + 1) * 100,
                _ => (),
            }
        }
        panic!("No reflection found");
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let matrix: Vec<Line> = value
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let rows: Vec<Line> = matrix.clone();
        let cols: Vec<Line> = (0..matrix[0].len())
            .map(|col| matrix.iter().map(|row| row[col]).collect())
            .collect();
        Self { cols, rows }
    }
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(Map::from)
        .map(|map| map.reflection_score(Reflection::Full))
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(Map::from)
        .map(|map| map.reflection_score(Reflection::Smudge))
        .sum()
}

#[test]
fn test() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    assert_eq!(part1(input), 405);
    assert_eq!(part2(input), 400);

    let input = "
##....##.####
.#...##.###..
.#.#.....#.#.
.##..#.#.#.##
.....#.##.#.#
.....#.##.#.#
.##..#.#.#.##
.#.##....#.#.
.#...##.###..
##....##.####
.############
..####.###.##
..####.###.##
.############
##....##.####
";

    assert_eq!(part1(input), 1200);
    //assert_eq!(part2(input), 400);
}
