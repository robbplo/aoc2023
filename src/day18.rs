pub fn solve() {
    let input = std::fs::read_to_string("input/day18.txt").unwrap();
    println!("# Day 18");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Rule {
    dir: char,
    count: isize,
    color: String,
}

impl Rule {
    fn offset(&self) -> (isize, isize) {
        match self.dir {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let dir = parts.next().unwrap().chars().next().unwrap();
        let count = parts.next().unwrap().parse::<isize>().unwrap();
        let color = parts.next().unwrap().to_string();
        Rule { dir, count, color }
    }
}

fn shoelace(points: &[(isize, isize)]) -> isize {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        sum += x1 * y2 - x2 * y1;
    }
    let (x1, y1) = points.first().unwrap();
    let (x2, y2) = points.last().unwrap();

    sum += x1 * y2 - x2 * y1;

    sum / 2
}

fn interior_points(points: &[(isize, isize)]) -> isize {
    let area = shoelace(&points);
    ((points.len() as isize / 2) - 1 - area).abs()
}

fn print_points(points: &[(isize, isize)]) {
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(input: &str) -> usize {
    let rules = input.trim().lines().map(Rule::from).collect::<Vec<_>>();
    let mut points: Vec<(isize, isize)> = vec![];
    let (mut x, mut y) = (0, 0);
    for rule in rules {
        let (dx, dy) = rule.offset();
        for _ in 0..rule.count {
            x += dx;
            y += dy;
            points.push((x, y));
        }
    }

    interior_points(&points) as usize + points.len()
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    assert_eq!(part1(input), 62);
    assert_eq!(part2(input), 952408144115);
}
