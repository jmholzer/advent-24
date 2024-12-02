use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

fn read_input() -> io::Result<(Vec<u32>, Vec<u32>)> {
    let path = Path::new("src/bin/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if numbers.len() >= 2 {
            col1.push(numbers[0]);
            col2.push(numbers[1]);
        }
    }

    Ok((col1, col2))
}

fn solve_part_one() -> io::Result<()> {
    let (mut col1, mut col2) = read_input()?;

    col1.sort();
    col2.sort();

    let total_diff: u64 = col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| (*a as i64 - *b as i64).abs() as u64)
        .sum();

    println!("{}", total_diff);

    Ok(())
}

fn solve_part_two() -> io::Result<()> {
    let (col1, col2) = read_input()?;

    let col2_frequency: HashMap<_, _> = col2.iter().fold(
        HashMap::new(),
        |mut map, &value| {
            *map.entry(value).or_insert(0) += 1;
            map
        }
    );

    let similarity_score: u64 = col1.iter().map(|value| {
        (*value as u64) * (*col2_frequency.get(value).unwrap_or(&0) as u64)
    }).sum();

    println!("{}", similarity_score);

    Ok(())
}

fn main() -> io::Result<()> {
    solve_part_one()?;
    solve_part_two()?;
    Ok(())
}
