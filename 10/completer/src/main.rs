use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut all_points = Vec::new();

    for line in handle.lines() {
        let line = line.unwrap();
        //println!("{}", line);
        let mut chars = line.chars();
        let mut stack = Vec::new();
        let mut is_corrupted = false;
        for char in chars.by_ref() {
            match char {
                '(' | '[' | '<' | '{' => stack.push(char),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        is_corrupted = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        is_corrupted = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        is_corrupted = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        is_corrupted = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !is_corrupted {
            let mut sum: i64 = 0;
            while stack.len() > 0 {
                let popped = stack.pop().unwrap();
                sum = sum * 5;
                match popped {
                    '(' => {
                        sum += 1;
                    }
                    '[' => {
                        sum += 2;
                    }
                    '{' => {
                        sum += 3;
                    }
                    '<' => {
                        sum += 4;
                    }
                    _ => {}
                }
            }
            all_points.push(sum);
        }
    }
    all_points.sort();
    let indx = all_points.len() / 2;
    println!("{}", all_points[indx]);
}
