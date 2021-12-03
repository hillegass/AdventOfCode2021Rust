use std::io::{self, BufRead};

const BIT_COUNT: usize = 12;

fn to_keep(bit:usize, bitstrings:&Vec<Vec<char>>, use_commonest:bool) -> char {
    let mut one_count:usize = 0;
    for v in bitstrings {
        if v[bit] == '1'{
            one_count += 1;
        }
    }

    let double_one_count = 2 * one_count;

    if use_commonest {
        if double_one_count >= bitstrings.len() {
            return '1'
        } else {
            return '0'
        }
    } else {
        if double_one_count >= bitstrings.len() {
            return '0'
        } else {
            return '1'
        }
    }
}

fn last_match(all_bitstrings:&Vec<Vec<char>>, use_commonest:bool) -> Vec<char> {
    let mut bitstrings = all_bitstrings.clone();
    let mut bit:usize = 0;
    while bitstrings.len() > 1 {
        let match_char = to_keep(bit, &bitstrings, use_commonest);
    
        let mut j:usize = 0;
        while j < bitstrings.len() {
            if bitstrings[j][bit] == match_char {
                j+=1;
            } else {
                bitstrings.swap_remove(j);
            }
        }
        bit += 1;
    }
    return bitstrings[0].clone();
}

fn int_for_bitstring(bs:&Vec<char>) -> i32 {
    let mut result:i32 = 0;
    for i in 0..BIT_COUNT {
        result *= 2;
        if bs[i] == '1' {
            result += 1;
        } 
    }
    return result;
}

fn main() {
    let mut all_bitstrings: Vec<Vec<char>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let bitstring: Vec<char> = line.unwrap().chars().collect();
        all_bitstrings.push(bitstring);
    }

    let oxygen_bitstring = last_match(&all_bitstrings, true);
    let co2_bitstring = last_match(&all_bitstrings, false);

    let oxygen = int_for_bitstring(&oxygen_bitstring);
    let co2 = int_for_bitstring(&co2_bitstring);

    println!("Oxygen:{}, CO2:{}, Product:{}", oxygen, co2, oxygen * co2);
    
}
