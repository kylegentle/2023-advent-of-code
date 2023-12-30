use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

pub fn mirage(f: File) -> Result<(), Box<dyn Error>> {
    let mut measurements: Vec<Vec<i64>> = Vec::new();
    let lines = BufReader::new(f).lines();
    for line in lines {
        let parts: Vec<i64> = line?
            .split_whitespace()
            .map(str::parse::<i64>)
            .collect::<Result<_, _>>()?;
        measurements.push(parts);
    }

    let p1 = measurements.iter().try_fold(0, |acc, ms| {
        predict_next(ms).map(|n| acc + n)
    })?;
    println!("Part 1: {}", p1);

    Ok(())
}

fn differences(nums: &Vec<i64>) -> Vec<i64> {
    nums.iter()
        .zip(nums[1..].iter())
        .map(|(a, b)| b - a)
        .collect()
}

fn predict_next(nums: &Vec<i64>) -> Result<i64, Box<dyn Error>> {
    let mut cur = nums.clone();
    let mut last_diffs: Vec<i64> = Vec::new();

    let Some(ln) = nums.iter().last() else {
        return Err("empty nums".into());
    };

    loop {
        let d = differences(&cur);
        if d.iter().all(|d| *d == 0) {
            return Ok(last_diffs.iter().rev().fold(0, |acc, d| acc + d) + ln);
        }

        let Some(ld) = d.last() else {
            return Err("empty diffs".into());
        };

        last_diffs.push(*ld);
        cur = d;
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_differences() {
        let v = vec![1, 1, 2, 3, 5, 8];
        let expected = vec![0, 1, 1, 2, 3];
        let actual = differences(&v);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_predict_next() {
        let v = vec![1, 3, 6, 10, 15, 21];
        let expected = 28;
        let actual = predict_next(&v).unwrap();
        assert_eq!(actual, expected)
    }
}
