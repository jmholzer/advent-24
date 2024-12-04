use std::fs;
use std::io;
use std::path::Path;
use regex::Regex;

fn read_input() -> io::Result<String> {
    let path = Path::new("src/bin/day-three-input.txt");
    fs::read_to_string(path)
}

fn solve_part_one() {
    let input = read_input().unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re.captures_iter(&input)
        .map(|caps| {
            let first_number = caps[1].parse::<i64>().unwrap();
            let second_number = caps[2].parse::<i64>().unwrap();
            first_number * second_number
        })
        .sum::<i64>();

    
    println!("Part one: {}", sum);
}

fn solve_part_two() {
    let input = read_input().unwrap() + "do()";

    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let processed_input = re.replace_all(&input, "");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re.captures_iter(&processed_input)
        .map(|caps| {
            let first_number = caps[1].parse::<i64>().unwrap();
            let second_number = caps[2].parse::<i64>().unwrap();
            first_number * second_number
        })
        .sum::<i64>();

    
    println!("Part two: {}", sum);
}

fn main() {
    solve_part_one();
    solve_part_two();
}
