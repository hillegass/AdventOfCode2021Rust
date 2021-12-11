use std::collections::HashSet;
use std::io::{self, Read};

fn add_to_basin(
    input: &Vec<Vec<u8>>,
    partial_basin: &mut HashSet<(usize, usize)>,
    height: usize,
    width: usize,
) -> bool {
    let mut something_added = false;
    let to_check = partial_basin.clone();
    for (r, c) in to_check {
        if c > 0 && !partial_basin.contains(&(r, c - 1)) && input[r][c - 1] != 9 {
            partial_basin.insert((r, c - 1));
            something_added = true;
        }
        if c < (width - 1) && !partial_basin.contains(&(r, c + 1)) && input[r][c + 1] != 9 {
            partial_basin.insert((r, c + 1));
            something_added = true;
        }
        if r > 0 && !partial_basin.contains(&(r - 1, c)) && input[r - 1][c] != 9 {
            partial_basin.insert((r - 1, c));
            something_added = true;
        }
        if r < (height - 1) && !partial_basin.contains(&(r + 1, c)) && input[r + 1][c] != 9 {
            partial_basin.insert((r + 1, c));
            something_added = true;
        }
    }
    return something_added;
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
    let width = grid[0].len();
    let height = grid.len();

    let mut basin_sizes = Vec::new();
    for col in 0..width {
        for row in 0..height {
            let less_than_above = (row == 0) || (grid[row][col] < grid[row - 1][col]);
            let less_than_below = (row == height - 1) || (grid[row][col] < grid[row + 1][col]);
            let less_than_left = (col == 0) || (grid[row][col] < grid[row][col - 1]);
            let less_than_right = (col == width - 1) || (grid[row][col] < grid[row][col + 1]);
            if less_than_above && less_than_below && less_than_left && less_than_right {
                let mut basin = HashSet::new();
                basin.insert((row, col));
                while add_to_basin(&grid, &mut basin, height, width) {}
                // print!("#");
                basin_sizes.push(basin.len());
            } else {
                // print!("-");
            }
        }
        // println!("");
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!("{:?}", &basin_sizes[0..3]);
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
