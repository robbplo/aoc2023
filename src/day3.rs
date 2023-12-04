use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    println!("# Day 3");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let board = input.trim().lines().collect::<Vec<&str>>();
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let mut total = 0;

    for (row_idx, line) in board.iter().enumerate() {
        for mat in pattern.find_iter(line) {
            if has_surrounding_symbol(
                board.clone(),
                mat.start().saturating_sub(1),
                row_idx.saturating_sub(1),
                mat.end(),
                row_idx + 1,
            ) {
                total += mat.as_str().parse::<i32>().unwrap();
            }
        }
    }

    total
}

fn has_surrounding_symbol(
    board: Vec<&str>,
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
) -> bool {
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            if y < board.len()
                && x < board[y].len()
                && board[y].chars().nth(x).unwrap() != '.'
                && !board[y].chars().nth(x).unwrap().is_digit(10)
            {
                return true;
            }
        }
    }
    return false;
}

#[test]
fn test_part1() {
    let input = "
12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";

    assert_eq!(part1(&input), 413);
}

fn part2(input: &str) -> i32 {
    let board = input.trim().lines().collect::<Vec<&str>>();
    let pattern = regex::Regex::new(r"(\d+)").unwrap();
    let mut total = 0;
    let mut gears = HashMap::new();

    for (row_idx, line) in board.iter().enumerate() {
        for mat in pattern.find_iter(line) {
            has_surrounding_symbol_part_2(
                board.clone(),
                mat.start().saturating_sub(1),
                row_idx.saturating_sub(1),
                mat.end(),
                row_idx + 1,
                &mut gears,
                mat.as_str(),
            );
        }
    }

    gears.iter().for_each(|(_, v)| {
        if v.len() == 2 {
            total += v.iter().map(|x| x.parse::<i32>().unwrap()).product::<i32>();
        }
    });

    total
}

fn has_surrounding_symbol_part_2<'a>(
    board: Vec<&str>,
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    gears: &mut HashMap<(usize, usize), Vec<&'a str>>,
    mat: &'a str,
) -> bool {
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            if y < board.len()
                && x < board[y].len()
                && board[y].chars().nth(x).unwrap() != '.'
                && !board[y].chars().nth(x).unwrap().is_digit(10)
            {
                if board[y].chars().nth(x).unwrap() == '*' {
                    // add mat to the vector in x,y
                    gears.entry((x, y)).or_insert_with(Vec::new).push(mat);
                }
                return true;
            }
        }
    }
    return false;
}

#[test]
fn test_part2() {
    let input = "
12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";

    assert_eq!(part2(&input), 6756);
}
