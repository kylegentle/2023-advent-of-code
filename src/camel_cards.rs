use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

const CARD_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const P2_CARD_STRENGTH: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bet: i32,
    p2: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn parse(line: &String, p2: bool) -> Result<Hand, Box<dyn Error>> {
        let mut parts = line.split_whitespace();

        let cards: Vec<char> = parts
            .next()
            .ok_or("missing cards in hand")
            .map(|s| s.chars())?
            .collect();

        if cards.len() != 5 {
            return Err(format!("Too many cards for a hand, line={}", line).into());
        }

        let bet = parts
            .next()
            .ok_or(format!("missing bet in hand, line={}", line))
            .map(str::parse::<i32>)??;

        Ok(Hand { cards, bet, p2 })
    }

    fn hand_type(&self) -> HandType {
        use HandType::*;

        let mut counter: HashMap<char, i8> = HashMap::new();
        for c in &self.cards {
            *counter.entry(*c).or_insert(0) += 1;
        }

        let joker_count = if self.p2 {
            counter.remove(&'J').unwrap_or(0)
        } else {
            0
        };

        if joker_count == 5 {
            return FiveOfAKind;
        }

        let mut counts = counter.values_mut().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        *counts[0] += joker_count;

        if *counts[0] == 5 {
            return FiveOfAKind;
        }

        match (*counts[0], *counts[1]) {
            (4, _) => FourOfAKind,
            (3, 2) => FullHouse,
            (3, _) => ThreeOfAKind,
            (2, 2) => TwoPair,
            (2, _) => Pair,
            _ => HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let strength = if self.p2 {
            P2_CARD_STRENGTH
        } else {
            CARD_STRENGTH
        };
        match self.hand_type().cmp(&other.hand_type()) {
            Less => Less,
            Greater => Greater,
            Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let self_strength = strength.iter().position(|&x| x == *self_card).unwrap();
                    let other_strength = strength.iter().position(|&x| x == *other_card).unwrap();

                    match self_strength.cmp(&other_strength) {
                        Less => return Less,
                        Greater => return Greater,
                        _ => (),
                    }
                }
                Equal
            }
        }
    }
}

pub fn camel_cards(f: File) -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = BufReader::new(f).lines().collect::<Result<_, _>>()?;

    let p1_hands = lines
        .iter()
        .map(|l| Hand::parse(l, false))
        .collect::<Result<Vec<Hand>, _>>()?;

    let p2_hands = lines
        .iter()
        .map(|l| Hand::parse(l, true))
        .collect::<Result<Vec<Hand>, _>>()?;

    let p1 = winnings(p1_hands);
    println!("Part 1: {}", p1);

    let p2 = winnings(p2_hands);
    println!("Part 2: {}", p2);

    Ok(())
}

fn winnings(mut hands: Vec<Hand>) -> i32 {
    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bet * (i as i32 + 1))
}

mod tests {
    #[cfg(test)]
    use super::HandType::*;
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_hand_type() {
        let cases = [
            (
                Hand::parse(&"T55J5 0".to_string(), false).unwrap(),
                ThreeOfAKind,
            ),
            (
                Hand::parse(&"QQQJA 0".to_string(), false).unwrap(),
                ThreeOfAKind,
            ),
            (Hand::parse(&"32T3K 0".to_string(), false).unwrap(), Pair),
            (Hand::parse(&"KK677 0".to_string(), false).unwrap(), TwoPair),
        ];

        for c in cases {
            assert_eq!(c.0.hand_type(), c.1)
        }
    }

    #[test]
    fn test_p2_hand_type() {
        let cases = [
            (
                Hand::parse(&"T55J5 0".to_string(), true).unwrap(),
                FourOfAKind,
            ),
            (
                Hand::parse(&"QQQJA 0".to_string(), true).unwrap(),
                FourOfAKind,
            ),
            (
                Hand::parse(&"KTJJT 0".to_string(), true).unwrap(),
                FourOfAKind,
            ),
            (Hand::parse(&"32T3K 0".to_string(), true).unwrap(), Pair),
            (Hand::parse(&"KK677 0".to_string(), true).unwrap(), TwoPair),
        ];

        for c in cases {
            assert_eq!(c.0.hand_type(), c.1)
        }
    }

    #[test]
    fn test_hand_cmp() {
        let cases = [
            (
                Hand::parse(&"T55J5 0".to_string(), false).unwrap(),
                Hand::parse(&"QQQJA 0".to_string(), false).unwrap(),
                Less,
            ),
            (
                Hand::parse(&"KK677 0".to_string(), false).unwrap(),
                Hand::parse(&"KTJJT 0".to_string(), false).unwrap(),
                Greater,
            ),
        ];

        for c in cases {
            assert_eq!(c.0.cmp(&c.1), c.2)
        }
    }

    #[test]
    fn test_winnings() {
        let hands = vec![
            Hand::parse(&"32T3K 765".to_string(), false).unwrap(),
            Hand::parse(&"T55J5 684".to_string(), false).unwrap(),
            Hand::parse(&"KK677 28".to_string(), false).unwrap(),
            Hand::parse(&"KTJJT 220".to_string(), false).unwrap(),
            Hand::parse(&"QQQJA 483".to_string(), false).unwrap(),
        ];

        let actual = winnings(hands);
        assert_eq!(actual, 6440);
    }

    #[test]
    fn test_p2_winnings() {
        let hands = vec![
            Hand::parse(&"32T3K 765".to_string(), true).unwrap(),
            Hand::parse(&"T55J5 684".to_string(), true).unwrap(),
            Hand::parse(&"KK677 28".to_string(), true).unwrap(),
            Hand::parse(&"KTJJT 220".to_string(), true).unwrap(),
            Hand::parse(&"QQQJA 483".to_string(), true).unwrap(),
        ];

        let actual = winnings(hands);
        assert_eq!(actual, 5905);
    }
}
