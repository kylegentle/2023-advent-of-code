use std::error::Error;
use std::fs::File;
use std::result::Result;
use std::io::{prelude::*, BufReader};

pub fn trebuchet(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);

    let mut vals = Vec::new(); 

    for line in reader.lines() {
        let val = line.map(|l| parse_calibration_value(&l))??;
        vals.push(val);
    }

    // sum vals
    let sum = vals.iter().sum::<u32>();
    println!("{}", sum);

    Ok(())
}

fn parse_calibration_value(line: &str) -> Result<u32, Box<dyn Error>> {
    let chars = line.chars().collect::<Vec<char>>();
    let (mut i, mut j) = (0, chars.len() - 1);

    let mut calval = 1;

    while i < chars.len() {
        if let Some(val) = chars[i].to_digit(10) {
            calval *= val*10;
            break;
        }
        i += 1;
    }

    while j >= i {
        if let Some(val) = chars[j].to_digit(10) {
            calval += val;
            return Ok(calval)
        }
        j -= 1;
    }

    let errmsg = format!("Could not parse calibration value from line: {}", line);
    Err(errmsg.into())
}