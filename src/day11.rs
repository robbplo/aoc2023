pub fn solve() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("# Day 11");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 1_000_000));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Galaxy {
    label: usize,
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(label: usize, x: usize, y: usize) -> Self {
        Self { label, x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Universe {
    galaxies: Vec<Galaxy>,
    height: usize,
    width: usize,
}

impl Universe {
    fn expand_by(&mut self, n: usize) {
        let factor = (1).max(n - 1);
        let occupied_cols: Vec<usize> = self.galaxies.iter().map(|g| g.x).collect();
        let empty_cols = (0..self.width).filter(|x| !occupied_cols.contains(x));
        let occupied_rows: Vec<usize> = self.galaxies.iter().map(|g| g.y).collect();
        let empty_rows = (0..self.height).filter(|y| !occupied_rows.contains(y));
        for galaxy in self.galaxies.iter_mut() {
            let shift_x = empty_cols.clone().filter(|x| *x < galaxy.x).count() * factor;
            let shift_y = empty_rows.clone().filter(|y| *y < galaxy.y).count() * factor;

            galaxy.x += shift_x;
            galaxy.y += shift_y;
        }
    }

    fn expand(&mut self) {
        self.expand_by(1);
    }

    fn manhattan_distance(&self, a: &Galaxy, b: &Galaxy) -> i64 {
        (a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let lines = value.trim().lines();
        let mut galaxies = Vec::new();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();
        let mut galaxy_label = 1;

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy::new(galaxy_label, x, y));
                    galaxy_label += 1;
                }
            }
        }

        Universe {
            galaxies,
            height,
            width,
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut universe = Universe::from(input);

    universe.expand();
    let combinations = universe.galaxies.iter().enumerate().flat_map(|(i, a)| {
        universe
            .galaxies
            .iter()
            .enumerate()
            .filter(move |(j, _)| *j > i)
            .map(move |(_, b)| (a, b))
    });
    combinations
        .map(|(a, b)| universe.manhattan_distance(a, b))
        .sum()
}

fn part2(input: &str, expand_by: usize) -> i64 {
    let mut universe = Universe::from(input);
    universe.expand_by(expand_by);
    let combinations = universe.galaxies.iter().enumerate().flat_map(|(i, a)| {
        universe
            .galaxies
            .iter()
            .enumerate()
            .filter(move |(j, _)| j > &i)
            .map(move |(_, b)| (a, b))
    });
    combinations
        .map(|(a, b)| universe.manhattan_distance(a, b))
        .sum()
}

#[test]
fn test() {
    let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(part1(input), 374);
    assert_eq!(part2(input, 10), 1030);
    assert_eq!(part2(input, 100), 8410);
}
