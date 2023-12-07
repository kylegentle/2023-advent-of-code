use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

struct Game {
    id: u16,
    rounds: Vec<Round>,
}

struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

const ROUNDLIMIT: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

impl Round {
    fn new() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn parse(line: &str) -> Result<Round, Box<dyn Error>> {
        let cube_draws = line.split(", ").collect::<Vec<&str>>();
        let mut round = Round::new();

        for c in cube_draws {
            let parts = c.trim().split(' ').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(format!("Invalid cube draw format: {}", c).into());
            }

            let count = parts[0].parse::<u8>()?;
            let color = parts[1].to_lowercase();

            match color.as_str() {
                "red" => round.red += count,
                "green" => round.green += count,
                "blue" => round.blue += count,
                _ => return Err(format!("Invalid color: {}", color).into()),
            }
        }

        return Ok(round);
    }

    fn possible(&self) -> bool {
        if self.red > ROUNDLIMIT.red || self.green > ROUNDLIMIT.green || self.blue > ROUNDLIMIT.blue
        {
            return false;
        }

        return true;
    }
}

impl Game {
    fn parse(line: &str) -> Result<Game, Box<dyn Error>> {
        let parts = line.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("Invalid line format, too many colons: {}", line).into());
        };

        let id = parts[0]
            .split(' ')
            .nth(1)
            .ok_or(format!("Invalid game segment: {}", parts[0]))?
            .parse()?;

        let rounds = parts[1]
            .split("; ")
            .map(|s| Round::parse(s))
            .collect::<Result<Vec<Round>, Box<dyn Error>>>()?;

        Ok(Game { id, rounds })
    }

    fn min_round(&self) -> Round {
        return self.rounds.iter().fold(Round::new(), |mut acc, r| {
            acc.red = std::cmp::max(acc.red, r.red);
            acc.green = std::cmp::max(acc.green, r.green);
            acc.blue = std::cmp::max(acc.blue, r.blue);
            acc
        })
    }

    fn power(&self) -> u32 {
        let mr = self.min_round();
        return mr.red as u32 * mr.green as u32 * mr.blue as u32;
    }
}

pub fn cube(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);

    let mut sum = 0;
    let mut powersum: u32 = 0;

    for line in reader.lines() {
        let game = Game::parse(&line?)?;
        if game.rounds.iter().filter(|r| !r.possible()).count() == 0 {
            sum += game.id;
        }
        powersum += game.power();
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", powersum);
    Ok(())
}

mod tests {
    #[cfg(test)]
    use super::{Game, Round};

    #[test]
    fn test_round_parse() {
        let round = Round::parse("1 red, 2 green, 3 blue").unwrap();
        assert_eq!(round.red, 1);
        assert_eq!(round.green, 2);
        assert_eq!(round.blue, 3);
    }

    #[test]
    fn test_game_parse() {
        let game = Game::parse("Game 1: 1 red, 2 green, 3 blue; 4 red, 5 green, 6 blue").unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.rounds.len(), 2);
        assert_eq!(game.rounds[0].red, 1);
        assert_eq!(game.rounds[0].green, 2);
        assert_eq!(game.rounds[0].blue, 3);
    }
}
