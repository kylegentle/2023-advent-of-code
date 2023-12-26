use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

const CARD_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bet: i32,
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
    fn parse(line: &String) -> Result<Hand, Box<dyn Error>> {
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

        Ok(Hand { cards, bet })
    }

    fn hand_type(&self) -> HandType {
        use HandType::*;

        let mut counter: HashMap<char, i8> = HashMap::new();
        for c in &self.cards {
            *counter.entry(*c).or_insert(0) += 1;
        }

        let mut counts = counter.values().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));

        if *counts[0] == 5 {
            return FiveOfAKind;
        }

        match (counts[0], counts[1]) {
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
        match self.hand_type().cmp(&other.hand_type()) {
            Less => Less,
            Greater => Greater,
            Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let self_strength =
                        CARD_STRENGTH.iter().position(|&x| x == *self_card).unwrap();
                    let other_strength = CARD_STRENGTH
                        .iter()
                        .position(|&x| x == *other_card)
                        .unwrap();

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

    let hands = lines
        .iter()
        .map(|l| Hand::parse(l))
        .collect::<Result<Vec<Hand>, _>>()?;

    let p1 = winnings(hands);
    println!("Part 1: {}", p1);

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
            (Hand::parse(&"T55J5 0".to_string()).unwrap(), ThreeOfAKind),
            (Hand::parse(&"QQQJA 0".to_string()).unwrap(), ThreeOfAKind),
            (Hand::parse(&"32T3K 0".to_string()).unwrap(), Pair),
            (Hand::parse(&"KK677 0".to_string()).unwrap(), TwoPair),
        ];

        for c in cases {
            assert_eq!(c.0.hand_type(), c.1)
        }
    }

    #[test]
    fn test_hand_cmp() {
        let cases = [
            (
                Hand::parse(&"T55J5 0".to_string()).unwrap(),
                Hand::parse(&"QQQJA 0".to_string()).unwrap(),
                Less,
            ),
            (
                Hand::parse(&"KK677 0".to_string()).unwrap(),
                Hand::parse(&"KTJJT 0".to_string()).unwrap(),
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
            Hand::parse(&"32T3K 765".to_string()).unwrap(),
            Hand::parse(&"T55J5 684".to_string()).unwrap(),
            Hand::parse(&"KK677 28".to_string()).unwrap(),
            Hand::parse(&"KTJJT 220".to_string()).unwrap(),
            Hand::parse(&"QQQJA 483".to_string()).unwrap(),
        ];

        let actual = winnings(hands);
        assert_eq!(actual, 6440);
    }
}
