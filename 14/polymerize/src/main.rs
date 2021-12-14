use std::collections::HashMap;
use std::io::{self, BufRead, Write};

const SEQ_RUNS : usize = 1;

const CODEBOOK_RUNS : usize = 20;

const CHEM_COUNT : usize = 10;

const CHEMICALS : [char; CHEM_COUNT] = ['K', 'H', 'O', 'N', 'P','F','S', 'V', 'B','C'];

fn add_counts(seq: &Vec<char>, count_map: &mut HashMap<char, u64>) {
    for c in seq {
        let count = count_map.entry(*c).or_insert(0);
        *count += 1;
    }
}

fn with_insertions(seq: &Vec<char>, insertions: &HashMap<(char, char), char>) -> Vec<char> {
    let mut result = Vec::with_capacity(seq.len() * 2);
    result.push(seq[0]);
    for i in 1..seq.len() {
        let pair = (seq[i - 1], seq[i]);
        if let Some(insertion) = insertions.get(&pair) {
            result.push(*insertion);
        } 
        result.push(seq[i]);
    }
    result
}

fn main() {

    // Read the input
    let mut map: HashMap<(char, char), char> = HashMap::new();
    let mut input: Vec<char> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let linestr = line.unwrap().clone();
        let values = linestr.split_whitespace().collect::<Vec<_>>();
        if values.len() == 3 {
            let a = String::from(values[0]);
            let b = String::from(values[2]);
            map.insert((a.chars().nth(0).unwrap(), a.chars().nth(1).unwrap()), b.chars().nth(0).unwrap());
        } else if values.len() == 1 {
            input = values[0].chars().collect();
        }
    }

    // Make two code books
    println!("Making codebooks representing pairs expanded {} times:", CODEBOOK_RUNS);
    let mut bimap:HashMap<(char, char), Vec<char>> = HashMap::new();
    let mut bimap_counts: HashMap<(char, char), HashMap<char, u64>> = HashMap::new();
    for i in 0..CHEM_COUNT {
        for j in 0..CHEM_COUNT {
            print!("{}-{} ", CHEMICALS[i], CHEMICALS[j]);
            std::io::stdout().flush().unwrap();
            let pair = (CHEMICALS[i], CHEMICALS[j]);
            let mut seq = vec![CHEMICALS[i], CHEMICALS[j]];
            for _ in 0..CODEBOOK_RUNS {
                seq = with_insertions(&seq, &map);
            }

            // Note count
            let mut counts:HashMap<char, u64> = HashMap::new();
            add_counts(&seq, &mut counts);
            bimap_counts.insert(pair, counts);

            // Store whole sequence
            bimap.insert(pair, seq);
              
        }
        println!("");
    }
    let entry_size = bimap.iter().next().unwrap().1.len();
    println!("Entry size: {:?}", entry_size);
    println!("Input: {:?}", input);

    // Expand using bimap codebook
    let mut buffer = Vec::with_capacity(entry_size * input.len());
    for i in 0..SEQ_RUNS {
        for j in 0..input.len() - 1 {
            let pair = (input[j], input[j + 1]);
            if let Some(seq) = bimap.get(&pair) {
                buffer.extend(seq);
                buffer.pop();
            } else {
                panic!("No sequence for {:?}", pair);
            }
        }
        buffer.push(input[input.len() - 1]);
        input = buffer;
        buffer = Vec::with_capacity(entry_size * input.len());
        println!("After expanding {} times: {} members", i + 1, input.len());
    }

    // Get counds using bimap_counts codebook
    let mut counts: HashMap<char, u64> = HashMap::new();
    for j in 0..input.len() - 1 {
        let pair = (input[j], input[j + 1]);
        if let Some(count_map) = bimap_counts.get(&pair) {
            for (c, count) in count_map {
                let count_entry = counts.entry(*c).or_insert(0);
                *count_entry += *count;
            }
            let last_count = counts.entry(input[j + 1]).or_insert(0);
            *last_count -= 1;
        }
    }
    let real_last_count = counts.entry(input[input.len() - 1]).or_insert(0);
    *real_last_count += 1;
    println!("Final counts: {:?}", counts);

    // Find max and min
    let mut min = std::u64::MAX;
    let mut max = 0;
    for (_k, v) in counts.iter() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }
    println!("{} - {} = {}", max, min, max - min);
}
