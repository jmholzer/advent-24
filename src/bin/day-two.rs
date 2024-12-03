use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_input() -> io::Result<Vec<Vec<u32>>> {
    let path = Path::new("src/bin/day-two-input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut rows: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        rows.push(numbers);
    }

    Ok(rows)
}

fn solve_part_one() {
    let grid = read_input().unwrap();

    let result: u32 = grid
        .iter()
        .filter(|row| {
            let mut increasing = true;
            let mut decreasing = true;
            let mut range_bound = true;
            
            for i in 1..row.len() {
                if row[i] < row[i - 1]  {
                    increasing = false;
                }
                if row[i] > row[i - 1] {
                    decreasing = false;
                }
                if !(1..=3).contains(&(row[i] as i32 - row[i - 1] as i32).abs()) {
                    range_bound = false;
                }
            }
            
            (increasing || decreasing) && range_bound
        })
        .count() as u32;

    println!("Part one: {}", result);
}

fn solve_part_two() -> u32 {
    let grid = read_input().unwrap();
    
    fn check_row(row: &[u32]) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        let mut range_bound = true;
        
        for i in 1..row.len() {
            if row[i] < row[i - 1] {
                increasing = false;
            }
            if row[i] > row[i - 1] {
                decreasing = false;
            }
            if !(1..=3).contains(&(row[i] as i32 - row[i - 1] as i32).abs()) {
                range_bound = false;
            }
        }
        
        (increasing || decreasing) && range_bound
    }

    let result: u32 = grid
        .iter()
        .filter(|row| {
            if check_row(row) {
                return true;
            }
           
            // fuck it it's 1am we're brute forcing
            for i in 0..row.len() {
                let test_row: Vec<u32> = row.iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &x)| x)
                    .collect();
                
                if check_row(&test_row) {
                    return true;
                }
            }
            false
        })
        .count() as u32;

    println!("Part two: {}", result);
    result
}

fn main() {
    solve_part_one();
    solve_part_two();
}
