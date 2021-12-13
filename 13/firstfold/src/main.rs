use std::collections::HashSet;
use std::io::{self, BufRead};

fn show_set(set: &HashSet<(i32, i32)>) {
    for r in 0..7 {
        for c in 0..40 {
            if set.contains(&(c, r)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fold_set(in_set: &HashSet<(i32, i32)>, axis:char, location:i32) -> HashSet<(i32, i32)> {
    let mut out_set = HashSet::new();
    for &(x, y) in in_set.iter() {
        let mut new_pair = (x, y);
        if axis == 'x' && x > location {
            new_pair.0 = location - (x - location);
        }
        if axis == 'y' && y > location {
            new_pair.1 = location - (y - location);
        }
        out_set.insert(new_pair);   
    }
    out_set
}

fn main() {
    let stdin = io::stdin();
    let mut opaque_points: HashSet<(i32, i32)> = HashSet::new();
    let mut folds:Vec<(char, i32)> = Vec::new();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let strs = line_str.split(",").collect::<Vec<_>>();
        if strs.len() == 2 {
            let x = strs[0].parse::<i32>().unwrap();
            let y = strs[1].parse::<i32>().unwrap();
            opaque_points.insert((x, y));
        } else {
            let strs = line_str.split("=").collect::<Vec<_>>();
            if strs.len() == 2 {
                let c = strs[0].chars().last().unwrap();
                let n = strs[1].parse::<i32>().unwrap();
                folds.push((c, n));
            }
        }
    }
    //println!("{:?}", opaque_points);
    //println!("{:?}", folds);
    for fold in folds {
        opaque_points = fold_set(&opaque_points, fold.0, fold.1);
    }
    show_set(&opaque_points);
}
