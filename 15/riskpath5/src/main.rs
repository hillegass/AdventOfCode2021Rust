use std::io::{self, Read};
use ndarray::Array2;
use std::collections::HashSet;

const PAGES: usize = 5;

fn update_neighbors(distance: &mut Array2<i32>, grid:&Array2<i32>, r: usize, c: usize, 
    unvisited: &mut HashSet<(usize, usize)>) {
    
    if r > 0 {
        let new_value = distance[(r, c)] + grid[(r-1, c)];  
        if new_value < distance[(r-1, c)] {
            distance[(r-1, c)] = new_value;
            unvisited.insert((r-1, c));
        }  
    }  
    if r < distance.nrows() - 1 {
        let new_value = distance[(r, c)] + grid[(r+1, c)];  
        if new_value < distance[(r+1, c)] {
            distance[(r+1, c)] = new_value;
            unvisited.insert((r+1, c));
        }  
    }
    if c > 0 {
        let new_value = distance[(r, c)] + grid[(r, c-1)];  
        if new_value < distance[(r, c-1)] {
            distance[(r, c-1)] = new_value;
            unvisited.insert((r, c-1));
        }  
    }
    if c < distance.ncols() - 1 {
        let new_value = distance[(r, c)] + grid[(r, c+1)];  
        if new_value < distance[(r, c+1)] {
            distance[(r, c+1)] = new_value;
            unvisited.insert((r, c+1));
        }  
    }
}

fn show_grids(distance_array:&Array2<i32>, grid:&Array2<i32>) {
    for i in 0..distance_array.nrows() {
        for j in 0..distance_array.ncols() {
            print!("{}({:>3})  ", grid[[i,j]], distance_array[[i, j]]);
        }
        println!();
    }
}
fn lowest_distance(distance_array:&Array2<i32>, unvisited:&HashSet<(usize, usize)>) -> (usize, usize) {
    let mut lowest = std::i32::MAX;
    let mut pair = (0, 0);
    for (r, c) in unvisited {
        if distance_array[[*r, *c]] < lowest {
            lowest = distance_array[[*r, *c]];
            pair = (*r, *c);
        }
    }
    pair
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
    let nrows = grid.len();
    let ncols = grid[0].len();

    println!("Read input:{} rows, {} cols", nrows, ncols);

    let mut big_grid = Array2::zeros ((nrows * PAGES, ncols * PAGES));
    for i in 0..PAGES {
        let start_row = i * nrows;
        for j in 0..PAGES {
            let start_col = j * ncols;
            let sum =  i + j; 
            for r in 0..nrows {
                let row_data = &grid[r];
                for c in 0..ncols {
                    let mut new_value = row_data[c] + sum as i32;
                    if new_value > 9 {
                        new_value = new_value - 9;
                    }
                    big_grid[[start_row + r, start_col + c]] =  new_value;
                }
            }
        }
    }
    println!("Created big grid:{} rows, {} cols", big_grid.nrows(), big_grid.ncols());

    let mut distance = Array2::<i32>::zeros((PAGES * ncols, PAGES * nrows));
    distance.fill(i32::max_value());
    let mut unvisited = HashSet::new();
    let mut visited = HashSet::new();
    distance[[0, 0]] = 0;
    visited.insert((0, 0));
    unvisited.insert((1, 0));
    unvisited.insert((0, 1));
    let mut counter:i64 = 0;
    while !visited.contains(&(PAGES * ncols - 1, PAGES * nrows - 1)) {
        let to_visit = lowest_distance(&distance, &unvisited);
        update_neighbors(&mut distance, &big_grid, to_visit.0, to_visit.1, &mut unvisited);
        unvisited.remove(&to_visit);
        visited.insert(to_visit);
    }

    //show_grids(&distance, &big_grid);
    println!("{}", distance[[(PAGES*nrows)-1, (PAGES * ncols)-1]]);
}
