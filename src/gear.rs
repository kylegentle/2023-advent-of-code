use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result::Result;

pub fn gear(f: File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(f);

    let schematic = reader
        .lines()
        .map(|l| l.map_err(|e| e.into()))
        .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    let mut adjacency_matrix: Vec<Vec<bool>> = Vec::new();
    for _ in 0..schematic.len() {
        adjacency_matrix.push(vec![false; schematic[0].len()]);
    }

    let symbol_pattern = Regex::new(r"[^\d\.]").unwrap();

    // Mark adjacency
    for (i, row) in schematic.iter().enumerate() {
        for m in symbol_pattern.find_iter(row) {
            mark_adjacent(i as i16, m.start() as i16, &mut adjacency_matrix)
        }
    }

    let mut p1_result = 0;

    let num_pattern = Regex::new(r"(\d+)").unwrap();
    for (i, row) in schematic.iter().enumerate() {
        for m in num_pattern.find_iter(row) {
            for j in m.start()..m.end() {
                if adjacency_matrix[i][j] {
                    p1_result += m.as_str().parse::<u32>().unwrap();
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", p1_result);

    Ok(())
}

fn mark_adjacent(row: i16, col: i16, adjacency_matrix: &mut Vec<Vec<bool>>) {
    let cells = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];

    for cell in cells.iter() {
        if cell.0 >= 0
            && cell.1 >= 0
            && cell.0 < adjacency_matrix.len() as i16
            && cell.1 < adjacency_matrix[0].len() as i16
        {
            adjacency_matrix[cell.0 as usize][cell.1 as usize] = true;
        }
    }
}
