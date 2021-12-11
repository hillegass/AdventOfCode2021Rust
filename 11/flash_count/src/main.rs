use std::collections::HashSet;
use std::io::{self, Read};

fn step_grid(grid: &mut Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let width = grid[0].len();
    let height = grid.len();
    let mut result = HashSet::new();
    for row in 0..height {
        for col in 0..width {
            grid[row][col] += 1;
            if grid[row][col] > 9 {
                result.insert((row, col));
                grid[row][col] = 0;
            }
        }
    }
    return result;
}
fn zero_flashed(grid: &mut Vec<Vec<u8>>, flashed: &HashSet<(usize, usize)>) {
    for (row, col) in flashed {
        grid[*row][*col] = 0;
    }
}
fn step_neighbors(
    grid: &mut Vec<Vec<u8>>,
    flashed: &HashSet<(usize, usize)>,
    refractory: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut result = HashSet::new();

    for (row, col) in flashed {
        for i in -1..2 as i32 {
            let r = *row as i32 + i;
            for j in -1..2 as i32 {
                if i == 0 && j == 0 {
                    continue;
                }

                let c = *col as i32 + j;

                if r < 0 || c < 0 {
                    continue;
                }

                if r >= height || c >= width {
                    continue;
                }
                let pair = (r as usize, c as usize);
                if refractory.contains(&pair) {
                    continue;
                }

                if result.contains(&pair) {
                    continue;
                }

                grid[pair.0][pair.1] += 1;
                if grid[pair.0][pair.1] > 9 {
                    grid[pair.0][pair.1] = 0;
                    result.insert(pair);
                }
            }
        }
    }
    return result;
}

fn show_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = Vec::new();
    handle.read_to_end(&mut input).unwrap();

    let mut grid = Vec::new();
    let mut line = Vec::new();
    for char in input {
        if char == 10 {
            grid.push(line);
            line = Vec::new();
        } else {
            line.push(char - 48);
        }
    }
    if line.len() > 0 {
        grid.push(line);
    }
    show_grid(&grid);
    let mut sum = 0;
    for i in 1..10000 {
        let initial_flashed = step_grid(&mut grid);
        let mut refractory = HashSet::new();
        let mut new_flashed = step_neighbors(&mut grid, &initial_flashed, &refractory);
        refractory.extend(&initial_flashed);
        while new_flashed.len() > 0 {
            refractory.extend(&new_flashed);
            new_flashed = step_neighbors(&mut grid, &new_flashed, &refractory);
        }
        zero_flashed(&mut grid, &refractory);
        sum += refractory.len();
        if refractory.len() == 100 {
            println!("Everyone flashed on step {}", i);
            break;
        }
        if i == 100 {
            println!("Sum: {}", sum);
        }
        // println!("After {}: {} flashed", i, flashed.len());
        //show_grid(&grid);
    }
}
