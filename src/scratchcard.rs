use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

struct ScratchCard {
    id: u16,
    score: u32,
}

impl ScratchCard {
    fn parse(line: &String) -> Result<ScratchCard, Box<dyn Error>> {
        let parts = line.split(':').collect::<Vec<&str>>();
        let numbers = parts.get(1).ok_or("Invalid line format")?;
        let series = numbers.split('|').collect::<Vec<&str>>();

        let id = parts[0]
            .split_whitespace()
            .nth(1)
            .ok_or("Invalid line format")
            .map(|n| n.parse::<u16>())??;

        let win_nums = series[0]
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<HashSet<u8>>();

        let nums = series[1]
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<HashSet<u8>>();

        let winners = win_nums.intersection(&nums).count() as u32;

        let score = match winners {
            0 => 0,
            n => 2_i32.pow(n - 1) as u32,
        };

        Ok(ScratchCard {
            id,
            score,
        })
    }
}

pub fn scratchcard(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);

    let lines = reader
        .lines()
        .map(|l| l.map_err(|e| e.into()))
        .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    let cardmap = lines.iter().fold(HashMap::new(), |mut acc, line| {
        let card = ScratchCard::parse(line).unwrap();
        acc.insert(card.id, card);
        acc
    });

    let score = cardmap.values().fold(0, |acc, card| acc + card.score);
    let cards_won = cardmap.keys().fold(0, |acc, id| {
        acc + card_wins(&cardmap, id, 1)
    });

    println!("Part 1: {}", score);
    println!("Part 2: {}", cards_won);
    Ok(())
}

fn card_wins(cards: &HashMap<u16, ScratchCard>, id: &u16, memo: u32) -> u32 {
    match cards.get(&id) {
        None => return memo,
        Some(card) => {
            if card.score == 0 {
                return memo;
            }

            let winners = (card.score.ilog2() + 1) as u16;
            ((id+1)..(id+winners+1)).fold(0, |acc, card| {
                acc + card_wins(cards, &card, memo)
            }) + 1
        }
    }
}
