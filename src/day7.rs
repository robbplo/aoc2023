use std::{cmp::Ordering, collections::HashMap};

pub fn solve() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("# Day 7");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bet: i32,
}

impl Hand {
    fn from_str(input: &str) -> Hand {
        let mut iter = input.split_whitespace();
        let cards = iter.next().unwrap().chars().collect::<Vec<_>>();
        let bet = iter.next().unwrap().parse::<i32>().unwrap();
        Hand { cards, bet }
    }

    fn score(&self) -> Score {
        let jokers = self.cards.iter().filter(|c| **c == 'J').count() as i32;
        let freq =
            self.cards
                .iter()
                .filter(|c| **c != 'J')
                .fold(HashMap::new(), |mut map, card| {
                    map.entry(card).and_modify(|f| *f += 1).or_insert(1);
                    map
                });
        let mut freq_values: Vec<&i32> = freq.values().collect();
        freq_values.sort();
        let mut highest = *freq_values.pop().unwrap_or(&0);
        highest += jokers;
        freq_values.push(&highest);
        freq_values.reverse();

        match freq_values.as_slice() {
            [5, ..] => Score::FiveOfAKind,
            [4, ..] => Score::FourOfAKind,
            [3, 2, ..] => Score::FullHouse,
            [2, 2, ..] => Score::TwoPair,
            [3, ..] => Score::ThreeOfAKind,
            [2, ..] => Score::Pair,
            _ => Score::HighCard,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = self.score();
        let other_score = other.score();
        if self_score == other_score {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let self_value = card_value(self_card);
                let other_value = card_value(other_card);
                if self_value != other_value {
                    return self_value.cmp(&other_value);
                }
            }
        }
        self_score.cmp(&other_score)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_value(card: &char) -> i32 {
    match card {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as i32,
    }
}

fn part1(input: &str) -> i64 {
    let lines = input.trim().lines();
    let mut hands: Vec<Hand> = lines.map(|l| Hand::from_str(l)).collect();
    hands.sort();
    let mut total = 0;
    let mut multiplier = 1;
    for hand in hands {
        total += hand.bet as i64 * multiplier;
        multiplier += 1;
    }
    total
}

#[test]
fn test_part1() {
    let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    //assert_eq!(part1(input), 6440);
}

fn part2(input: &str) -> i64 {
    let lines = input.trim().lines();
    let mut hands: Vec<Hand> = lines.map(|l| Hand::from_str(l)).collect();
    hands.sort();
    let mut total = 0;
    let mut multiplier = 1;
    for hand in hands {
        println!("{:?} {:?} {:?}", hand.cards, hand.bet, hand.score());
        total += hand.bet as i64 * multiplier;
        multiplier += 1;
    }
    total
}

#[test]
fn test_part2() {
    let input = "
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41
";
    assert_eq!(part2(input), 6839);
}

