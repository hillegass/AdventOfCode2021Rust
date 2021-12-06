use std::io::{self, BufRead};

const MAX_AGE: usize = 8;
const GESTATION_PERIOD: usize = 6;
const DAYS: usize = 256;
// const DAYS: usize = 80;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let numbers = lines[0].split(",");
    
    let mut counts:Vec<u64> = vec![0; MAX_AGE + 1];
    for number in numbers {
        counts[number.parse::<usize>().unwrap()] += 1;
    }

    println!("Initial state:{:?}", counts);

    for day in 0..DAYS {
        let child_bearing = counts[0];
        counts.remove(0);
        counts.push(child_bearing);
        counts[GESTATION_PERIOD] += child_bearing;
        println!("After {} day:{:?}", day + 1, counts);
    }
    let mut sum = 0;
    for fcount in counts {
        sum += fcount;
    }
    println!("Total fish:{}", sum);
}
