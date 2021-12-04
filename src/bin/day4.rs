use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    won: bool,
    rows: Vec<HashSet<u32>>,
    cols: Vec<HashSet<u32>>,
}

fn board_score(board: &Board) -> u32 {
    let mut result = 0;
    for idx in 0..5 {
        for num in &board.rows[idx] {
            result += num;
        }
    }

    result
}

fn new_board() -> Board {
    Board {
        won: false,
        rows: Vec::new(),
        cols: vec![HashSet::new(); 5],
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut numbers: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    // Parse the input into boards
    let mut idx = 0;
    let mut board = new_board();
    for line in input.split("\n") {
        if idx == 0 {
            numbers = line.split(",").map(|n| n.parse::<u32>().unwrap()).collect();
        } else if line == "" {
            if board.rows.len() == 5 {
                boards.push(board);
            }
            board = new_board();
        } else {
            let row: Vec<u32> = line
                .split(" ")
                .filter(|l| *l != "")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            for idx in 0..5 {
                board.cols[idx].insert(row[idx]);
            }
            let mut rownums: HashSet<u32> = HashSet::new();
            for num in row {
                rownums.insert(num);
            }
            board.rows.push(rownums);
        }
        idx += 1;
    }
    if board.rows.len() == 5 {
        boards.push(board);
    }

    (numbers, boards)
}

fn part_one(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    // Iterate the numbers removing from everything
    for num in numbers {
        for idx in 0..boards.len() {
            for rowcol in 0..5 {
                if boards[idx].rows[rowcol].remove(&num) {
                    if boards[idx].rows[rowcol].len() == 0 {
                        boards[idx].won = true;
                    }
                }
                if boards[idx].cols[rowcol].remove(&num) {
                    if boards[idx].cols[rowcol].len() == 0 {
                        boards[idx].won = true;
                    }
                }
            }
            if boards[idx].won {
                return board_score(&boards[idx]) * num;
            }
        }
    }

    0
}

fn part_two(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    let mut boards_won = 0;

    // Iterate the numbers removing from everything
    for num in numbers {
        for idx in 0..boards.len() {
            if boards[idx].won {
                continue;
            }
            for rowcol in 0..5 {
                if boards[idx].rows[rowcol].remove(&num) {
                    if boards[idx].rows[rowcol].len() == 0 {
                        boards[idx].won = true;
                    }
                }
                if boards[idx].cols[rowcol].remove(&num) {
                    if boards[idx].cols[rowcol].len() == 0 {
                        boards[idx].won = true;
                    }
                }
            }
            if boards[idx].won {
                boards_won += 1;
                if boards_won == boards.len() {
                    return board_score(&boards[idx]) * num;
                }
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("day4.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
