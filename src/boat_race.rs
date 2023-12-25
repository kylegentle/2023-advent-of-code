use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

pub fn boat_race(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let times: Vec<i64> = lines[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    let distances: Vec<i64> = lines[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    let race_specs = times.iter().zip(distances.iter());
    let p1 = race_specs.fold(1, |acc: i64, new| acc * ways_to_win(*new.0, *new.1));

    let p2_time = lines[0]
        .replace("Time: ", "")
        .replace(" ", "")
        .parse::<i64>()?;

    let p2_distance = lines[1]
        .replace("Distance: ", "")
        .replace(" ", "")
        .parse::<i64>()?;

    println!("Part 1: {}", p1);
    println!("Part 2: {}", ways_to_win(p2_time, p2_distance));

    Ok(())
}

fn ways_to_win(time: i64, distance: i64) -> i64 {
    let mut ret = 0;
    for wait in 0..time {
        let speed = wait;
        let new_dist = speed * (time - wait);
        if new_dist > distance {
            ret += 1;
        }
    }

    ret
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_ways_to_win() {
        let actual = ways_to_win(30, 200);
        let expected = 9;
        assert_eq!(actual, expected);
    }
}
