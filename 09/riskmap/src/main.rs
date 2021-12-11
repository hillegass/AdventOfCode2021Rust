use std::io::{self, Read};

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
    let width = grid[0].len();
    let height = grid.len();

    let mut sum: u32 = 0;
    for col in 0..width {
        for row in 0..height {
            let less_than_above = (row == 0) || (grid[row][col] < grid[row - 1][col]);
            let less_than_below = (row == height - 1) || (grid[row][col] < grid[row + 1][col]);
            let less_than_left = (col == 0) || (grid[row][col] < grid[row][col - 1]);
            let less_than_right = (col == width - 1) || (grid[row][col] < grid[row][col + 1]);
            if less_than_above && less_than_below && less_than_left && less_than_right {
                print!("{}", grid[row][col]);
                sum += grid[row][col] as u32 + 1;
            } else {
                print!("-");
            }
        }
        println!("");
    }
    println!("{}", sum);
}
