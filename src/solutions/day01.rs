use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn read_input(input_path: PathBuf) -> crate::Result<Vec<i64>> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);
    // TODO: better error handling
    let data = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    Ok(data)
}

pub fn part1(input_path: PathBuf) -> crate::Result<i64> {
    let data = read_input(input_path)?;

    // Really naive O(N^2) implementation
    for (i, x) in data[..(data.len() - 1)].iter().enumerate() {
        for y in &data[i..] {
            if x + y == 2020 {
                return Ok(x * y);
            }
        }
    }

    return Err("No solution found".into());
}

pub fn part2(input_path: PathBuf) -> crate::Result<i64> {
    let data = read_input(input_path)?;

    // Really naive O(N^3) implementation
    for (i, x) in data[..(data.len() - 2)].iter().enumerate() {
        for (j, y) in data[i..(data.len() - 1)].iter().enumerate() {
            for z in &data[(i + j)..] {
                if x + y + z == 2020 {
                    return Ok(x * y * z);
                }
            }
        }
    }

    return Err("No solution found".into());
}
