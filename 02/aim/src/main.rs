use std::io::{self, BufRead};

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut items = line.as_ref().unwrap().split(" ");
        let direction = items.next().unwrap();
        let value = items.next().unwrap().parse::<i32>().unwrap();
        match direction.as_ref() {
            "forward" => {
                horizontal += value;
                depth += value * aim
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("Unknown command:{}", direction),
        }
    }
    println!("Horizontal:{} Depth:{}", horizontal, depth);
    println!("Product {}", horizontal * depth);
}
