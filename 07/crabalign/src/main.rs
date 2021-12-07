use std::io::{self, BufRead};

fn fuel_required(guess:i32, numbers:&Vec<i32>) -> (i32, i32, i32, i32){
    let mut right_fuel = 0; // Fuel to move low crabs to the right
    let mut low_crabs = 0;
    let mut left_fuel = 0;
    let mut high_crabs = 0;  
    for number in numbers {
        if *number < guess {
            right_fuel += guess - *number;
            low_crabs += 1;
        } else if *number > guess {
            left_fuel += *number - guess;
            high_crabs += 1;
        }
    }
    return (right_fuel, left_fuel, low_crabs, high_crabs);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut numbers = lines[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    numbers.sort();
    println!("Input:{:?}", numbers);

    // The median minimizes L1 distance
    let guess:i32;
    if numbers.len() % 2 == 0 {
        guess = (numbers[numbers.len()/2 - 1] + numbers[numbers.len()/2]) / 2;
    } else {
        guess = numbers[(numbers.len() - 1)/2];    
    }

    let (right_fuel, left_fuel, low_crabs, high_crabs) = fuel_required(guess, &numbers);
    println!("{} {}:{} {}:{} {}", guess, right_fuel, low_crabs, left_fuel, high_crabs, right_fuel + left_fuel);
   
    let fuels = fuel_required(guess, &numbers);
    println!("{:?}", fuels.0 + fuels.1);
}