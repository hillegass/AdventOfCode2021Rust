use std::io::{self, BufRead};
use std::cmp;

const X_LEN: i32 = 1000;
const Y_LEN: i32 = 1000;

fn get_tuples(input: &str) -> (i32, i32) {
    let strs = input.split(",").collect::<Vec<&str>>();
    let x = strs[0].parse::<i32>().unwrap();
    let y = strs[1].parse::<i32>().unwrap();
    (x, y)
}

fn touches((x, y): (i32, i32), ((x1, y1), (x2, y2)): ((i32, i32), (i32,i32))) -> bool {
    let min_x = cmp::min(x1, x2);
    let max_x = cmp::max(x1, x2);
    let min_y = cmp::min(y1, y2);
    let max_y = cmp::max(y1, y2);
    if x1 == x2 {
        // Vertical line
        return x == x1 && y <= max_y && y >= min_y;
    } else if y1 == y2 {
        // Horizontal line
        return y == y1 && x <= max_y && x >= min_x;
    } else {
    
        return false;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut endpoints:Vec<((i32, i32),(i32,i32))> = Vec::new();
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
