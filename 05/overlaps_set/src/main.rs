use std::io::{self, BufRead};
use std::cmp;
use std::collections::HashSet;

fn get_tuples(input: &str) -> (i32, i32) {
    let strs = input.split(",").collect::<Vec<&str>>();
    let x = strs[0].parse::<i32>().unwrap();
    let y = strs[1].parse::<i32>().unwrap();
    (x, y)
}

fn points_for_endpoints(endpoint1: (i32, i32), endpoint2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let (x1, y1) = endpoint1;
    let (x2, y2) = endpoint2;
   
    if x1 == x2 {
        // Vertical line
        let min_y = cmp::min(y1, y2);
        let max_y = cmp::max(y1, y2);
        for y in min_y..=max_y {
            points.push((x1, y));
        }
    } else if y1 == y2 {
        // Horizontal line
        let min_x = cmp::min(x1, x2);
        let max_x = cmp::max(x1, x2);
        for x in min_x..=max_x {
            points.push((x, y1));
        }
    }
    points
}

fn main() {
    let stdin = io::stdin();
    let mut touched_points:HashSet<(i32, i32)> = HashSet::new();
    let mut counted_points:HashSet<(i32, i32)> = HashSet::new();

    let mut count = 0;
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let words = line_str.split_whitespace().collect::<Vec<_>>();
        if words.len() < 3 {
            continue;
        }
        let endpoint1 = get_tuples(words[0]);
        let endpoint2 = get_tuples(words[2]);
        let points:Vec<(i32, i32)> = points_for_endpoints(endpoint1, endpoint2);
        for point in points {

            // 3rd or more time seeing this poing?
            if counted_points.contains(&point) {
                continue;
            }

            // Second time seeing this point?
            if touched_points.contains(&point) {
                count += 1;
                counted_points.insert(point);
            } else {
                // First time seeing this point
                touched_points.insert(point);
            }
        }
    }

    println!("{}", count);
}
