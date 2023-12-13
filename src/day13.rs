pub fn solve() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("# Day 13");
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
}

type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Map {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Map {
    fn is_vertical_reflection(&self, idx: usize) -> bool {
        let left = idx;
        let right = idx + 1;
        let zip = (0..=left).rev().zip(right..self.cols.len());
        for (left, right) in zip {
            if self.cols[left] != self.cols[right] {
                return false;
            }
        }
        return true;
    }
    fn is_horizontal_reflection(&self, idx: usize) -> bool {
        let left = idx;
        let right = idx + 1;
        let zip = (0..=left).rev().zip(right..self.rows.len());
        for (left, right) in zip {
            if self.rows[left] != self.rows[right] {
                return false;
            }
        }
        return true;
    }
    fn reflection_score(&self) -> usize {
        for i in 0..self.cols.len() - 1 {
            if self.is_vertical_reflection(i) {
                return i + 1;
            }
        }
        for i in 0..self.rows.len() - 1 {
            if self.is_horizontal_reflection(i) {
                return (i + 1) * 100;
            }
        }
        panic!("No reflection found");
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let matrix: Matrix<char> = value
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let rows: Vec<String> = matrix
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect();
        let cols: Vec<String> = (0..matrix[0].len())
            .map(|col| matrix.iter().map(|row| row[col]).collect::<String>())
            .collect();
        Self { cols, rows }
    }
}

fn part1(input: &str) -> usize {
    let maps = input
        .trim()
        .split("\n\n")
        .map(Map::from)
        .collect::<Vec<_>>();

    maps.iter().map(|map| map.reflection_score()).sum()
}

fn part2(input: &str) -> usize {
    0
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

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#
";

    assert_eq!(part1(input), 709);
    //assert_eq!(part2(input), 525152);

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

    //assert_eq!(part2(input), 0);
}
