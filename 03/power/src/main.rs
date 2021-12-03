use std::io::{self, BufRead};

const BIT_COUNT: usize = 12;

fn main() {
    let mut count = 0;
    let mut one_count: [i32; BIT_COUNT] = [0; BIT_COUNT];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let bitstring: Vec<char> = line.unwrap().chars().collect();
        for i in 0..BIT_COUNT {
            let c = bitstring[i];
            if c == '1' {
                one_count[i] += 1;
            }
        }
        count += 1;
    }
    println!("Counts: {:?} of {}", one_count, count);
    let half: i32 = count / 2;
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for i in 0..BIT_COUNT {
        gamma *= 2;
        epsilon *= 2;
        if one_count[i] > half {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("Gamma:{}", gamma);
    println!("Epsilon:{}", epsilon);
    println!("Product: {}", gamma * epsilon);
}
