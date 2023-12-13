pub fn solve() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let lines = input.trim().lines();
    println!("# Day 1");
    println!("Part 1: {}", part1(&lines.clone().collect()));
    println!("Part 2: {}", part2(&lines.collect()));
}

fn part1(lines: &Vec<&str>) -> i32 {
    lines.iter().map(|line| linevalue(line)).sum()
}

#[test]
fn test_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let lines = input.lines().collect();
    assert_eq!(part1(&lines), 142);
}

fn linevalue(line: &str) -> i32 {
    let mut chars = line.chars().filter(|c| c.is_digit(10)).peekable();
    let mut number = String::from("");
    number.push(*chars.peek().unwrap());
    number.push(chars.next_back().unwrap());
    number.parse::<i32>().unwrap()
}

#[test]
fn test_linevalue() {
    assert_eq!(linevalue("1abc2"), 12);
    assert_eq!(linevalue("pqr3stu8vwx"), 38);
    assert_eq!(linevalue("a1b2c3d4e5f"), 15);
    assert_eq!(linevalue("treb7uchet"), 77);
}

fn part2(lines: &Vec<&str>) -> i32 {
    lines.iter().map(|line| linevalue2(line)).sum()
}

#[test]
fn test_part2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let lines = input.lines().collect();
    assert_eq!(part2(&lines), 281);
}

fn linevalue2(line: &str) -> i32 {
    linevalue(&convert_line(line))
}

#[test]
fn test_linevalue2() {
    assert_eq!(linevalue2("two1nine"), 29);
    assert_eq!(linevalue2("eightwothree"), 83);
    assert_eq!(linevalue2("abcone2threexyz"), 13);
    assert_eq!(linevalue2("xtwone3four"), 24);
    assert_eq!(linevalue2("4nineeightseven2"), 42);
    assert_eq!(linevalue2("zoneight234"), 14);
    assert_eq!(linevalue2("7pqrstsixteen"), 76);
}

fn convert_line(line: &str) -> String {
    let mut bytes = line.as_bytes();
    let mut result = Vec::new();
    loop {
        match bytes {
            [b'o', b'n', b'e', ..] => result.push(b'1'),
            [b't', b'w', b'o', ..] => result.push(b'2'),
            [b't', b'h', b'r', b'e', b'e', ..] => result.push(b'3'),
            [b'f', b'o', b'u', b'r', ..] => result.push(b'4'),
            [b'f', b'i', b'v', b'e', ..] => result.push(b'5'),
            [b's', b'i', b'x', ..] => result.push(b'6'),
            [b's', b'e', b'v', b'e', b'n', ..] => result.push(b'7'),
            [b'e', b'i', b'g', b'h', b't', ..] => result.push(b'8'),
            [b'n', b'i', b'n', b'e', ..] => result.push(b'9'),
            [] => break,
            _ => result.push(bytes[0]),
        }
        bytes = &bytes[1..];
    }

    String::from_utf8(result).unwrap()
}
