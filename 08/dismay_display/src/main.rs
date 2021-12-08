use std::io::{self, BufRead};

fn main() {
    let mut counters = [0; 8];
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let phrases = line_str.split(" ").collect::<Vec<_>>().clone();
        for i in 11..15 {
            let word_str = phrases[i];
            let word_len = word_str.len();
            counters[word_len] += 1;
        }
    }
    println!("{:?}", counters);
    println!("Total: {:}", counters[2] + counters[3] + counters[4] + counters[7]);
}
