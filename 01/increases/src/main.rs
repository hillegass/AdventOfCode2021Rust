use std::io::{self, BufRead};

fn main() {
    let mut count = 0;
    let mut last_value = i32::MAX;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        if value > last_value {
            count += 1;
        }
        last_value = value;
    }
    println!("{}", count);
}
