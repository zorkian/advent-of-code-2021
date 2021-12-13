use std::cmp::{max, min};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: FoldAxis,
    index: u32,
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Point>,
}

impl Paper {
    fn new() -> Paper {
        Paper { dots: Vec::new() }
    }

    fn fold(self: &Paper, fold: &Fold) -> Paper {
        let mut rv = Paper::new();

        for point in &self.dots {
            let mut new_dot = Point { x: 0, y: 0 };

            // X fold = fold up
            match fold.axis {
                FoldAxis::X => {
                    new_dot.x = if point.x < fold.index {
                        point.x
                    } else {
                        fold.index - (point.x - fold.index)
                    };
                    new_dot.y = point.y;
                }
                FoldAxis::Y => {
                    new_dot.x = point.x;
                    new_dot.y = if point.y < fold.index {
                        point.y
                    } else {
                        fold.index - (point.y - fold.index)
                    };
                }
            }

            // See if we've seen this one
            let mut seen = false;
            for test_dot in &rv.dots {
                if test_dot.x == new_dot.x && test_dot.y == new_dot.y {
                    seen = true;
                    break;
                }
            }
            if !seen {
                rv.dots.push(new_dot);
            }
        }

        rv
    }

    fn print(self: &Paper) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for point in &self.dots {
            min_x = min(min_x, point.x);
            min_y = min(min_y, point.y);
            max_x = max(max_x, point.x);
            max_y = max(max_y, point.y);
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut exists = false;
                for point in &self.dots {
                    if point.x == x && point.y == y {
                        exists = true;
                        break;
                    }
                }
                if exists {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let mut rv = Paper::new();
    let mut folds = Vec::new();

    let mut parsing_folds = false;
    for line in input.split("\n") {
        if parsing_folds {
            // fold along x=5
            let (left, right) = line.split(" ").collect::<Vec<&str>>()[2]
                .split_once("=")
                .unwrap();
            folds.push(Fold {
                axis: if left == "x" {
                    FoldAxis::X
                } else {
                    FoldAxis::Y
                },
                index: right.parse::<u32>().unwrap(),
            });
            continue;
        }

        if line == "" {
            parsing_folds = true;
            continue;
        }

        let (x, y) = line.split_once(",").unwrap();
        rv.dots.push(Point {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        })
    }

    (rv, folds)
}

fn part_one(input: &str) -> u32 {
    let (mut paper, folds) = parse_input(input);

    paper = paper.fold(&folds[0]);
    return paper.dots.len() as u32;
}

fn part_two(input: &str) -> u32 {
    let (mut paper, folds) = parse_input(input);

    for fold in &folds {
        paper = paper.fold(fold);
    }
    paper.print();

    0
}

fn main() {
    let input = include_str!("day13.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
