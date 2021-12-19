use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Board {
    cells: HashMap<u32, u8>,
    visited: HashSet<u32>,
    width: u32,
    height: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    X: u32,
    Y: u32,
}

impl Board {
    fn raw_risk_at(self: &Board, x: u32, y: u32) -> u8 {
        *self.cells.get(&((y * self.height) + x)).unwrap()
    }

    fn h(self: &Board, point: &Point) -> u32 {
        // Heuristic cost estimation of how much effort it takes to get from this
        // point to the goal, we just use a measurement of how many hops it is as
        // an approximation -- this must be the best case cost
        (self.width - point.X - 1) + (self.height - point.Y - 1)
    }

    fn neighbors(self: &Board, point: &Point) -> Vec<Point> {
        let mut rv = Vec::new();

        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let ix = (point.X as i32 + offset.0) as i32;
            let iy = (point.Y as i32 + offset.1) as i32;
            if ix < 0 || ix >= self.width as i32 || iy < 0 || iy >= self.height as i32 {
                continue;
            }
            rv.push(Point {
                X: ix as u32,
                Y: iy as u32,
            });
        }

        rv
    }

    fn print(self: &Board) {
        // println!("Board size: {} by {}", self.width, self.height);
        for y in 0..self.height {
            println!("");
            for x in 0..self.width {
                let pos = (y * self.width) + x;
                let risk = self.cells.get(&pos).unwrap();
                let visited = self.visited.contains(&pos);
                if visited {
                    print!("*");
                } else {
                    print!("{}", risk);
                }
            }
        }
        println!("");
    }
}

fn parse_input(input: &str, part_two: bool) -> Board {
    let mut cells: HashMap<u32, u8> = HashMap::new();

    let mut width: u32 = 0;
    let mut y: u32 = 0;
    for line in input.split("\n") {
        let mut x: u32 = 0;
        if width == 0 {
            width = line.len() as u32;
            if part_two {
                width *= 5;
            }
        }

        for level in line
            .split("")
            .filter(|s| *s != "")
            .map(|s| s.parse::<u8>().unwrap())
        {
            cells.insert((y * width) + x, level);
            if part_two {
                for ym in 0..=4 as u32 {
                    for xm in 0..=4 as u32 {
                        if ym == 0 && xm == 0 {
                            continue;
                        }
                        let nlevel = ((level as u32 + (ym + xm) - 1) % 9) + 1;
                        cells.insert(((y + (100 * ym)) * width) + (x + (xm * 100)), nlevel as u8);
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }

    Board {
        cells,
        visited: HashSet::new(),
        width,
        height: if part_two { y * 5 } else { y },
    }
}

fn traverse_astar(board: &mut Board) -> u32 {
    let start = Point { X: 0, Y: 0 };
    let goal = Point {
        X: board.width - 1,
        Y: board.height - 1,
    };

    // Nodes we haven't visited
    let mut open_set = PriorityQueue::new();
    open_set.push(start, u32::MAX);

    // Track how we got to the node
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: HashMap<Point, u32> = HashMap::new();
    *g_score.entry(start).or_insert(0) = 0;

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    let mut f_score: HashMap<Point, u32> = HashMap::new();
    *f_score.entry(start).or_insert(0) = board.h(&start);

    // The main loop
    while !open_set.is_empty() {
        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        let current = open_set.pop().unwrap().0;
        // println!("Current = {}, {}", current.X, current.Y);

        // If we have our goal, we're done
        if current == goal {
            // Reconstruct by adding up all the risks until we get back to the beginning
            let mut risk = 0;
            let mut test = current;
            loop {
                // println!("Scoring up {}, {}", test.X, test.Y);
                board.visited.insert((test.Y * board.width) + test.X);
                if test == start {
                    return risk;
                }
                risk += board.raw_risk_at(test.X, test.Y) as u32;
                test = *came_from.get(&test).unwrap();
            }
        }

        // Get neighbors
        for neighbor in board.neighbors(&current) {
            let risk_at = board.raw_risk_at(neighbor.X, neighbor.Y);
            let tentative_g_score = g_score.get(&current).unwrap() + risk_at as u32;
            let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&u32::MAX);
            if tentative_g_score < neighbor_g_score {
                *came_from.entry(neighbor).or_insert(start) = current;
                *g_score.entry(neighbor).or_insert(u32::MAX) = tentative_g_score;
                let my_f_score = tentative_g_score + board.h(&neighbor);
                *f_score.entry(neighbor).or_insert(u32::MAX) = my_f_score;
                open_set.push(neighbor, u32::MAX - my_f_score);
            }
        }
    }

    panic!("no result!");
}

fn part_one(input: &str) -> u32 {
    let mut input = parse_input(input, false);
    traverse_astar(&mut input)
}

fn part_two(input: &str) -> u32 {
    let mut input = parse_input(input, true);
    let rv = traverse_astar(&mut input);
    //input.print();
    rv
}

fn main() {
    let input = include_str!("day15.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
