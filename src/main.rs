use std::env;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

mod boat_race;
mod camel_cards;
mod cube;
mod gear;
mod mirage;
mod scratchcard;
mod seed;
mod trebuchet;
mod wasteland;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let day_subdir = env::args()
        .nth(1)
        .ok_or("Provide a day subdirectory")?;
    let input_path = PathBuf::from(day_subdir).join(INPUT_FILE_NAME);
    let input_file = File::open(&input_path).map_err(|_| "Could not open input file")?;

    let day = input_path
        .parent()
        .ok_or("Couldn't resolve day subdir")?
        .to_str()
        .ok_or("Could not convert day subdirectory path to string")?
        .get(0..2)
        .ok_or("Could not get day from subdir name")?;

    return match day {
        "01" => trebuchet::trebuchet(input_file),
        "02" => cube::cube(input_file),
        "03" => gear::gear(input_file),
        "04" => scratchcard::scratchcard(input_file),
        "05" => seed::seed(input_file),
        "06" => boat_race::boat_race(input_file),
        "07" => camel_cards::camel_cards(input_file),
        "08" => wasteland::wasteland(input_file),
        "09" => mirage::mirage(input_file),
        _ => Err(format!("Day {} not implemented", day).into()),
    }
}
