use std::io::{self, Read};
use ndarray::Array2;
use std::collections::HashSet;

fn update_distance(distance: &mut Array2<i32>, grid: &Vec<Vec<i32>>, r: usize, c: usize, to_process: &mut HashSet<(usize, usize)>) {
    let mut min_neighbor = std::i32::MAX;
    
    if r > 0  && distance[[r-1,c]] < min_neighbor {
        min_neighbor = distance[[r-1, c]];
    }
    if r < distance.nrows() - 1 && distance[[r+1, c]] < min_neighbor{
        min_neighbor = distance[[r+1, c]];
    }
    if c > 0 && distance[[r,c-1]] < min_neighbor {
        min_neighbor = distance[[r, c-1]];
    }
    if c < distance.ncols() - 1 && distance[[r,c+1]] < min_neighbor {
        min_neighbor = distance[[r, c+1]];
    }
    let new_value = min_neighbor + grid[r][c];
    if new_value < distance[[r, c]] {
        distance[[r, c]] = new_value;
        if r > 0  && distance[[r-1, c]] > new_value {
            to_process.insert((r-1, c));
        }
        if r < distance.nrows() - 1 && distance[[r+1, c]] > new_value {
            to_process.insert((r+1, c));
        }
        if c > 0 && distance[[r,c-1]] > new_value {
            to_process.insert((r, c-1));
        }
        if c < distance.ncols() - 1 && distance[[r,c+1]] > new_value {
            to_process.insert((r, c+1));
        } 
    }
}

fn show_grids(distance_array:&Array2<i32>, grid:&Vec<Vec<i32>>) {
    for i in 0..distance_array.nrows() {
        for j in 0..distance_array.ncols() {
            print!("{}({:>3})  ", grid[i][j], distance_array[[i, j]]);
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
    let mut line:Vec<i32> = Vec::new();
    for char in input {
        if char == 10 {
            grid.push(line);
            line = Vec::new();
        } else {
            line.push((char - 48) as i32);
        }
    }
    if line.len() > 0 {
        grid.push(line);
    }
    let width = grid[0].len();
    let height = grid.len();

    let mut distance = Array2::<i32>::zeros((width, height));
    distance.fill(i32::max_value());
    let mut to_process = HashSet::new();
    distance[[0, 0]] = 0;
    to_process.insert((1, 0));
    to_process.insert((0, 1));
    while to_process.len() > 0 {
        let pair = to_process.iter().next().unwrap().clone();
        to_process.remove(&pair);
        update_distance(&mut distance, &grid, pair.0, pair.1, &mut to_process);
    }

    //show_grids(&distance, &grid);
    println!("{}", distance[[height-1, width-1]]);
}
