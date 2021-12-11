use std::cmp;
use std::io::{self, BufRead};

const X_LEN: i32 = 1000;
const Y_LEN: i32 = 1000;

fn get_tuples(input: &str) -> (i32, i32) {
    let strs = input.split(",").collect::<Vec<&str>>();
    let x = strs[0].parse::<i32>().unwrap();
    let y = strs[1].parse::<i32>().unwrap();
    (x, y)
}

fn touches((x, y): (i32, i32), ((x1, y1), (x2, y2)): ((i32, i32), (i32, i32))) -> bool {
    if x1 == x2 {
        // Vertical line
        return x == x1 && y <= cmp::max(y1, y2) && y >= cmp::min(y1, y2);
    } else if y1 == y2 {
        // Horizontal line
        return y == y1 && x <= cmp::max(x1, x2) && x >= cmp::min(x1, x2);
    } else {
        // Diagonal line
        let x_change = x - x1;
        let y_change = y - y1;

        // Is it on a diagonal?
        if x_change.abs() != y_change.abs() {
            return false;
        }
        // Is it to the left of both points?
        if x_change < 0 && x < x2 {
            return false;
        }
        // Is it to the right of both points?
        if x_change > 0 && x > x2 {
            return false;
        }
        // Is it above both points?
        if y_change < 0 && y < y2 {
            return false;
        }
        // Is it below both points?
        if y_change > 0 && y > y2 {
            return false;
        }
        return true;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut endpoints: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let words = line_str.split_whitespace().collect::<Vec<_>>();
        if words.len() < 3 {
            continue;
        }
        let endpoint1 = get_tuples(words[0]);
        let endpoint2 = get_tuples(words[2]);
        endpoints.push((endpoint1, endpoint2));
    }

    let mut count = 0;
    for x in 0..X_LEN {
        for y in 0..Y_LEN {
            let mut touchcount = 0;
            for endpoint in endpoints.iter() {
                if touches((x, y), *endpoint) {
                    touchcount += 1;
                    if touchcount >= 2 {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
