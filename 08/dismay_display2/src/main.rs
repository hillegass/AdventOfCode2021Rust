use std::collections::HashSet;
use std::io::{self, BufRead};

const ONE_SPOT: usize = 0;
// const SEVEN_SPOT: usize = 1;
const FOUR_SPOT: usize = 2;
// const EIGHT_SPOT: usize = 9;

fn main() {
    let stdin = io::stdin();
    let mut grand_total = 0;
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let phrases = line_str.split(" ").collect::<Vec<_>>().clone();
        let mut word_sets: Vec<HashSet<char>> = Vec::new();

        for i in 0..10 {
            let word_set = phrases[i].chars().collect::<HashSet<_>>();
            word_sets.push(word_set);
        }
        word_sets.sort_by(|a, b| a.len().cmp(&b.len()));
        // println!("{:?}", word_sets);
        let mut total = 0;
        for i in 11..15 {
            let input = phrases[i].chars().collect::<HashSet<_>>();

            // Take care of the ones with unique counts
            match input.len() {
                2 => {
                    total = total * 10 + 1;
                    continue;
                }
                3 => {
                    total = total * 10 + 7;
                    continue;
                }
                4 => {
                    total = total * 10 + 4;
                    continue;
                }
                7 => {
                    total = total * 10 + 8;
                    continue;
                }
                _ => {}
            }
            if input.len() == 5 {
                // Is it a 3? (1 is a subset of 3)
                if word_sets[ONE_SPOT].is_subset(&input) {
                    total = total * 10 + 3;
                    continue;
                }

                // Is it a 5? (4-1 is a subset of 5)
                let mut bend = word_sets[FOUR_SPOT].clone();
                for c in word_sets[ONE_SPOT].iter() {
                    bend.remove(c);
                }
                if bend.is_subset(&input) {
                    total = total * 10 + 5;
                    continue;
                }

                // Must be a 2;
                total = total * 10 + 2;
                continue;
            }

            if input.len() == 6 {
                // Is it a 9? (4 is a subset of 9)
                if word_sets[FOUR_SPOT].is_subset(&input) {
                    total = total * 10 + 9;
                    continue;
                }

                // Is it a 6? (1 is *not* a subset of 6)
                if !(word_sets[ONE_SPOT].is_subset(&input)) {
                    total = total * 10 + 6;
                    continue;
                }

                // Must be a 0
                total = total * 10;
                continue;
            }
            println!("Unknown {:?}!?!?", input);
        }
        let codes = &phrases[11..15];
        println!("Code:{:?} {}", codes, total);
        grand_total += total;
    }

    println!("Total: {}", grand_total);
}
