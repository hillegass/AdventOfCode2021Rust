use std::collections::HashSet;
use std::io::{self, BufRead};

const ROW_COL_COUNT: usize = 5;

fn read_boards(input: &Vec<String>) -> Vec<[[i32; ROW_COL_COUNT]; ROW_COL_COUNT]> {
    let mut result = Vec::new();
    let mut row_count: usize = 0;
    let mut board_array = [[0; ROW_COL_COUNT]; ROW_COL_COUNT];
    for row in input.iter() {
        let row_vec = row.split_whitespace().collect::<Vec<&str>>();
        if row_vec.len() == 0 {
            row_count = 0;
            result.push(board_array);
            // println!("Pushed {:?}", board_array);
            continue;
        }
        if row_vec.len() != ROW_COL_COUNT {
            panic!("Invalid input");
        }
        for col in 0..ROW_COL_COUNT {
            let v = row_vec[col].parse::<i32>().unwrap();
            board_array[row_count][col] = v;
        }
        row_count += 1;
    }
    if row_count != 0 {
        result.push(board_array);
        // println!("Pushed {:?}", board_array);
    }
    result
}

fn board_completed(
    called_numbers: &HashSet<i32>,
    board: &[[i32; ROW_COL_COUNT]; ROW_COL_COUNT],
) -> bool {
    // Check for complete rows
    for row in 0..ROW_COL_COUNT {
        let mut is_missing = false;
        for col in 0..ROW_COL_COUNT {
            is_missing = !called_numbers.contains(&board[row][col]);
            if is_missing {
                break;
            }
        }
        if !is_missing {
            // println!("Row {} complete with {:?}", row, called_numbers);
            return true;
        }
    }
    // Check for complete columns
    for col in 0..ROW_COL_COUNT {
        let mut is_missing = false;
        for row in 0..ROW_COL_COUNT {
            is_missing = !called_numbers.contains(&board[row][col]);
            if is_missing {
                break;
            }
        }
        if !is_missing {
            // println!("Column {} complete with {:?}", col, called_numbers);
            return true;
        }
    }
    return false;
}

fn indexes_of_completed_board(
    called_numbers: &HashSet<i32>,
    boards: &Vec<[[i32; ROW_COL_COUNT]; ROW_COL_COUNT]>,
) -> Vec<usize> {
    let mut result = Vec::new();
    // If we haven't called 5 numbers, skip this altogether
    if called_numbers.len() < ROW_COL_COUNT {
        return result;
    }
    for (i, board) in boards.iter().enumerate() {
        if board_completed(called_numbers, board) {
            // println!("Board {} completed: {:?}", i, board);
            result.push(i);
        }
    }
    result
}

fn sum_of_uncalled_numbers(
    called_numbers: &HashSet<i32>,
    board: &[[i32; ROW_COL_COUNT]; ROW_COL_COUNT],
) -> i32 {
    let mut result = 0;
    for r in 0..ROW_COL_COUNT {
        for c in 0..ROW_COL_COUNT {
            if !called_numbers.contains(&board[r][c]) {
                result += board[r][c];
            }
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let numbers = lines[0]
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    lines.remove(0);
    lines.remove(0);

    let mut boards = read_boards(&lines);
    // println!("{:?}", numbers);

    let mut current_round: usize = 0;
    let mut called_numbers = HashSet::new();
    let mut winning_board = boards[0];

    while boards.len() > 0 {
        if current_round == numbers.len() {
            break;
        }
        let num = numbers[current_round];
        called_numbers.insert(num);
        let mut completed_boards = indexes_of_completed_board(&called_numbers, &boards);
        completed_boards.reverse();
        for i in completed_boards {
            boards.remove(i);
        }
        // println!("Round {}: Number {}, {} boards remain", current_round, numbers[current_round], boards.len());
        if boards.len() >= 1 {
            winning_board = boards[0];
        }
        current_round += 1;
    }
    let last_number = numbers[current_round - 1];
    let sum = sum_of_uncalled_numbers(&called_numbers, &winning_board);

    println!(
        "Sum:{}, Last num:{}, Product:{}",
        sum,
        last_number,
        last_number * sum
    );
}
