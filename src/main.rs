use std::env;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

mod trebuchet;
mod cube;

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
        _ => Err(format!("Day {} not implemented", day).into()),
    }
}
