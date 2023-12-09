use std::{cmp::Ordering, collections::HashMap, fmt::Debug, sync::Mutex, thread::current};

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::aoc::Solution;

#[derive(Clone, Copy)]
pub enum Part {
    PartOne,
    PartTwo,
}

lazy_static! {
    static ref CURRENT_PART: Mutex<Part> = Mutex::new(Part::PartOne);
}

pub struct Seven;

/// Five of a kind, where all five cards have the same label: AAAAA
/// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
/// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
/// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
/// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
/// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
/// High card, where all cards' labels are distinct: 23456
#[derive(Debug, Clone, Copy, Eq, Ord)]
pub enum HandKind {
    FiveOfAKind,  // len = 1
    FourOfAKind,  // len = 2
    FullHouse,    // len = 2
    ThreeOfAKind, // len = 3
    TwoPair,      // len = 3
    OnePair,      // len = 4
    HighCard,     // len = 5
}

impl Into<u32> for HandKind {
    fn into(self) -> u32 {
        match self {
            HandKind::FiveOfAKind => 7,
            HandKind::FourOfAKind => 6,
            HandKind::FullHouse => 5,
            HandKind::ThreeOfAKind => 4,
            HandKind::TwoPair => 3,
            HandKind::OnePair => 2,
            HandKind::HighCard => 1,
        }
    }
}

impl From<u32> for HandKind {
    fn from(value: u32) -> Self {
        match value {
            7 => HandKind::FiveOfAKind,
            6 => HandKind::FourOfAKind,
            5 => HandKind::FullHouse,
            4 => HandKind::ThreeOfAKind,
            3 => HandKind::TwoPair,
            2 => HandKind::OnePair,
            1 => HandKind::HighCard,
            _ => panic!("fuck you"),
        }
    }
}

impl PartialEq for HandKind {
    fn eq(&self, other: &Self) -> bool {
        let a: u32 = (*self).into();
        let b: u32 = (*other).into();

        a == b
    }
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a: u32 = (*self).into();
        let b: u32 = (*other).into();

        a.partial_cmp(&b)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Card(char);

impl From<char> for Card {
    fn from(chr: char) -> Self {
        Card(chr)
    }
}

impl From<u32> for Card {
    fn from(val: u32) -> Self {
        let chr = match val {
            12 => 'A',
            11 => 'K',
            10 => 'Q',
            9 => 'T',
            8 => '9',
            7 => '8',
            6 => '7',
            5 => '6',
            4 => '5',
            3 => '4',
            2 => '3',
            1 => '2',
            0 => 'J',
            _ => panic!("unexpected"),
        };

        Card(chr)
    }
}

impl From<Card> for u32 {
    fn from(card: Card) -> Self {
        match *CURRENT_PART.lock().unwrap() {
            Part::PartOne => match card.0 {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                '9' => 7,
                '8' => 6,
                '7' => 5,
                '6' => 4,
                '5' => 3,
                '4' => 2,
                '3' => 1,
                '2' => 0,
                _ => panic!("unexpected"),
            },

            Part::PartTwo => match card.0 {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                'J' => 0,
                _ => panic!("unexpected"),
            },
        }
    }
}

#[derive(Clone, Eq, Ord)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: String = self.cards.iter().map(|card| card.0).collect();

        f.debug_struct("Hand")
            .field("cards", &cards)
            .field("bid", &self.bid)
            .finish()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // self.cards.iter().map(|c| c.0).collect::<String>()
        // == other.cards.iter().map(|c| c.0).collect::<String>()

        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a: u32 = self.kind(true).into();
        let b: u32 = other.kind(true).into();

        match a.partial_cmp(&b) {
            Some(Ordering::Equal) => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let c1: u32 = (*c1).into();
                    let c2: u32 = (*c2).into();

                    match c1.partial_cmp(&c2) {
                        Some(Ordering::Equal) => continue,
                        ord => return ord,
                    }
                }

                return Some(Ordering::Equal);
            }
            ord => return ord,
        }
    }
}

impl Hand {
    fn kind(&self, v2_behavior: bool) -> HandKind {
        let simple_compute_kind = |hand: &Hand| -> HandKind {
            let mut count: HashMap<Card, u32> = HashMap::new();

            for card in hand.cards.iter() {
                *count.entry(*card).or_default() += 1;
            }

            match count.len() {
                1 => HandKind::FiveOfAKind,

                2 => match *count.values().sorted().max().unwrap() {
                    4 => HandKind::FourOfAKind,
                    3 => HandKind::FullHouse,
                    _ => panic!("unexpected"),
                },

                3 => match *count.values().sorted().max().unwrap() {
                    3 => HandKind::ThreeOfAKind,
                    2 => HandKind::TwoPair,
                    _ => panic!("unexpected"),
                },

                4 => HandKind::OnePair,
                5 => HandKind::HighCard,
                _ => panic!("unexpected"),
            }
        };

        match (v2_behavior, *CURRENT_PART.lock().unwrap()) {
            (_, Part::PartOne) | (false, Part::PartTwo) => simple_compute_kind(self),
            (true, Part::PartTwo) => {
                // try all the combinations and return the max
                // let hand = self.clone();

                let j_positions = self.cards.iter().enumerate().filter_map(|(i, card)| {
                    if card.0 == 'J' {
                        Some(i)
                    } else {
                        None
                    }
                });

                let mut kinds = vec![simple_compute_kind(&hand)];

                for pos in j_positions {
                    for card in 0..=12 {
                        let mut hand_mut = self.clone();
                        *hand_mut.cards.get_mut(pos).unwrap() = Card::from(card);

                        let current_kind = simple_compute_kind(&hand_mut);

                        println!("{:?} kind is {:?}", self, current_kind);

                        kinds.push(current_kind.into());
                    }
                }

                let max = kinds.to_owned().into_iter().max().unwrap();
                println!(
                    "the max for {:?} is {max:?} they all were: {:?}",
                    self.clone(),
                    kinds
                );

                max
            }
        }
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let (cards, idk) = line.split_ascii_whitespace().collect_tuple().unwrap();

        let cards = cards.chars().map(Card::from).collect_vec();
        let idk = idk.parse().unwrap();

        Hand { cards, bid: idk }
    }
}

fn solve(parsed: &[Hand]) -> u32 {
    let mut ordered = parsed.to_owned();
    ordered.sort();

    ordered
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

impl Solution for Seven {
    type Output = u32;
    type Parsed = Vec<Hand>;

    fn input() -> &'static str {
        include_str!("../inputs/7.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(Hand::from).collect_vec()
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        solve(parsed)
    }

    fn solve_second(parsed: &Self::Parsed) -> Self::Output {
        *CURRENT_PART.lock().unwrap() = Part::PartTwo;

        let mut ordered = parsed.to_owned();
        ordered.sort();

        println!("final: {}", ordered.len());
        for c in &ordered {
            println!("{:?} -> {:?}", c, c.kind(false));
        }

        ordered
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
            .sum()

        // solve(parsed)
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (245794640, 0)
    }
}
