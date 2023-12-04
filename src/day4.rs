use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    println!("# Day 4");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Card {
    id: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>,
    wins: i32,
}

impl Card {
    fn from_str(input: &str) -> Option<Card> {
        let mut iter = input.split(": ");

        let id = iter
            .next()?
            .split_whitespace()
            .skip(1)
            .next()?
            .parse::<i32>()
            .unwrap();

        let mut iter = iter.next()?.split(" | ").map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let mut card = Card {
            id,
            winning: iter.next()?,
            numbers: iter.next()?,
            wins: 0,
        };
        card.wins = card.count_wins();
        Some(card)
    }

    fn count_wins(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|x| self.winning.contains(x))
            .count() as i32
    }

    fn score(&self) -> i32 {
        match self.count_wins() {
            0 => 0,
            1 => 1,
            x => 2_i32.pow(x as u32 - 1),
        }
    }

    fn count_copies(&self, cards: &HashMap<i32, Card>) -> i32 {
        let wins = self.wins;
        let range = self.id + 1..=self.id + wins;

        1 + range
            .map(|x| cards.get(&x).unwrap())
            .map(|c| c.count_copies(cards))
            .sum::<i32>()
    }
}

fn part1(input: &str) -> i32 {
    let lines = input.trim().lines().collect::<Vec<&str>>();

    lines
        .iter()
        .map(|l| Card::from_str(l).unwrap())
        .map(|c| c.score())
        .sum()
}

#[test]
fn test_part1() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    assert_eq!(part1(&input), 13);
}

fn part2(input: &str) -> i32 {
    let lines = input.trim().lines().collect::<Vec<&str>>();

    let mut card_map: HashMap<i32, Card> = HashMap::new();
    let cards = lines.iter().map(|l| Card::from_str(l).unwrap());

    for card in cards.clone() {
        card_map.insert(card.id, card);
    }

    cards.map(|c| c.count_copies(&card_map)).sum::<i32>()
}

#[test]
fn test_part2() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    assert_eq!(part2(&input), 30);
}
