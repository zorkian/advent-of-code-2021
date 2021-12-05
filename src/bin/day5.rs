use core::cmp::max;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

type Board = Vec<Vec<i32>>;

fn parse_point(input: &str) -> Point {
    let nums: Vec<i32> = input
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    return Point {
        x: nums[0],
        y: nums[1],
    };
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut rv = Vec::new();
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        let point1 = parse_point(&parts[0]);
        let point2 = parse_point(&parts[2]);
        rv.push(Line {
            start: point1,
            end: point2,
        })
    }
    rv
}

impl Line {
    fn plot(self: &Line, board: &mut Board, include_diagonals: bool) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let mut slope_x = 0;
        let mut slope_y = 0;

        if dx != 0 && dy == 0 {
            slope_x = dx / dx.abs();
        } else if dx == 0 && dy != 0 {
            slope_y = dy / dy.abs();
        } else {
            if include_diagonals {
                slope_x = dx / dx.abs();
                slope_y = dy / dy.abs();
            } else {
                return;
            }
        }

        let mut cur_x = self.start.x;
        let mut cur_y = self.start.y;

        loop {
            board[cur_y as usize][cur_x as usize] += 1;

            if cur_x == self.end.x && cur_y == self.end.y {
                break;
            }

            cur_x += slope_x;
            cur_y += slope_y;
        }
    }
}

fn print_board(board: &Board) {
    for y in 0..board.len() {
        for x in 0..board[y as usize].len() {
            print!("{}", board[y as usize][x as usize]);
        }
        println!("");
    }
}

fn part(input: &str, with_diagonals: bool) -> u32 {
    let lines = parse_lines(input);

    let mut max_x = 0;
    let mut max_y = 0;
    for idx in 0..lines.len() {
        max_x = max(max_x, max(lines[idx].start.x, lines[idx].end.x));
        max_y = max(max_y, max(lines[idx].start.y, lines[idx].end.y));
    }

    let mut board: Board = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
    for idx in 0..lines.len() {
        lines[idx].plot(&mut board, with_diagonals);
    }

    // print_board(&board);

    let mut count = 0;
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if board[y as usize][x as usize] >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn part_one(input: &str) -> u32 {
    part(input, false)
}

fn part_two(input: &str) -> u32 {
    part(input, true)
}

fn main() {
    let input = include_str!("day5.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
