use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut sum = 0;
    for line in handle.lines() {
        let line = line.unwrap();
        //println!("{}", line);
        let mut chars = line.chars();
        let mut stack = Vec::new();
        let mut line_points = 0;
        for char in chars.by_ref() {
            match char {
                '(' | '[' | '<' | '{' => stack.push(char),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        line_points = 3;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        line_points = 57;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        line_points = 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        line_points = 25137;
                        break;
                    }
                }
                _ => {}
            }
        }
        sum += line_points;
    }
    println!("{}", sum);
}
