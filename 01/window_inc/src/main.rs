use std::io::{self, BufRead};
use std::collections::VecDeque;

fn sum_collection(collection: &VecDeque<i32>) -> i32 {
    let mut sum = 0;
    for i in collection {
        sum += i;
    }
    sum
}

fn main() {
    let mut count = 0;
    let mut last_sum = i32::MAX;
    let mut buf = VecDeque::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        buf.push_back(value);
        while buf.len() > 3 {
            buf.pop_front();
        }        

        if buf.len() == 3 {
            let sum = sum_collection(&buf);
            if  sum > last_sum {
                count += 1;
            }
            last_sum = sum;
        }
    }
    println!("{}", count);
}
