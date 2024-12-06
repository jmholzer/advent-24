use std::fs;
use std::io;
use std::path::Path;

fn read_input() -> io::Result<Vec<Vec<char>>> {
    let path = Path::new("src/bin/day-four-input.txt");
    let contents = fs::read_to_string(path)?;
    
    let grid = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
        
    Ok(grid)
}

fn solve_part_one() {
    let grid = read_input().unwrap();

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1), (-1, 0), (-1, 1),  // Up-left, Up, Up-right
        (0, -1),           (0, 1),    // Left, Right
        (1, -1),  (1, 0),  (1, 1)     // Down-left, Down, Down-right
    ];

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                for (di, dj) in &directions {
                    let mut valid = true;
                    
                    for step in 1..4 {
                        let new_i = (i as i32 + di * step) as usize;
                        let new_j = (j as i32 + dj * step) as usize;
                        
                        if new_i >= grid.len() || new_j >= grid[0].len() {
                            valid = false;
                            break;
                        }
                        
                        let expected = match step {
                            1 => 'M',
                            2 => 'A', 
                            3 => 'S',
                            _ => unreachable!()
                        };
                        
                        if grid[new_i][new_j] != expected {
                            valid = false;
                            break;
                        }
                    }
                    
                    if valid {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part one: {}", count);
}

fn solve_part_two() {
    let grid = read_input().unwrap();

    let mut count = 0;

    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            if grid[i][j] == 'A' {
                let corners = [
                    (grid[i - 1][j - 1], grid[i - 1][j + 1], grid[i + 1][j - 1], grid[i + 1][j + 1])
                ];

                for (tl, tr, bl, br) in corners {
                    if (tl == 'M' && br == 'S' && tr == 'M' && bl == 'S') ||
                       (tl == 'S' && br == 'M' && tr == 'S' && bl == 'M') ||
                       (tl == 'M' && br == 'S' && tr == 'S' && bl == 'M') ||
                       (tl == 'S' && br == 'M' && tr == 'M' && bl == 'S') {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part two: {}", count);
}

fn main() {
    solve_part_one();
    solve_part_two();
}
