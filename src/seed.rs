use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::num::ParseIntError;
use std::result::Result;

pub fn seed(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);

    let maps = reader
        .lines()
        .map(|l| l.map_err(|e| e.into()))
        .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    let seed_line = maps.iter().next().ok_or("No seed line")?;
    let seed_strs = seed_line
        .split(':')
        .nth(1)
        .ok_or("missing RHS of seed line")?;

    let seeds = seed_strs
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<i64>, ParseIntError>>()?;

    let p1 = locate_seeds(&maps[1..], seeds)
        .and_then(|l| l.iter().min().cloned().ok_or("no location vec".into()))?;

    println!("Part 1: {}", p1);
    Ok(())
}

fn locate_seeds(maps: &[String], mut seeds: Vec<i64>) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut new_seeds = seeds.clone();

    for line in maps {
        if line.is_empty() {
            continue;
        }

        if line.ends_with(':') {
            let mut tmp = seeds;
            seeds = fill_gaps(&tmp, &new_seeds);
            tmp.fill(0);
            new_seeds = tmp;
            continue;
        }

        let nums = line
            .split_whitespace()
            .map(str::parse::<i64>)
            .collect::<Result<Vec<i64>, ParseIntError>>()?;

        let dest = nums[0];
        let src = nums[1];
        let range = nums[2];

        for (i, s) in seeds.iter().enumerate() {
            if src <= *s && *s <= src + range {
                new_seeds[i] = dest + (s - src)
            }
        }
    }

    Ok(fill_gaps(&seeds, &new_seeds))
}

fn fill_gaps(old: &Vec<i64>, new: &Vec<i64>) -> Vec<i64> {
    return new
        .iter()
        .enumerate()
        .map(|(i, s)| if *s == 0 { old[i] } else { *s })
        .collect();
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_locate_seeds() {
        let maps = vec![
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
        ];
        let seeds = vec![79, 14, 55, 13];
        let actual = locate_seeds(&maps, seeds).unwrap();
        assert_eq!(vec![81, 14, 57, 13], actual);
    }
}
