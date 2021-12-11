use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

#[derive(Debug)]
struct Board {
    cells: HashMap<u32, Octopus>,
    width: u32,
    height: u32,
}

impl Board {
    fn add_energy(self: &mut Board, x: u32, y: u32) {
        self.cells.entry((y * self.width) + x).and_modify(|o| {
            if o.flashed {
                return;
            }
            o.energy += 1;
        });
    }

    fn trigger_flash(self: &mut Board, x: u32, y: u32) -> bool {
        let mut flashed = false;

        self.cells.entry((y * self.width) + x).and_modify(|o| {
            if !o.flashed && o.energy >= 10 {
                // println!("flashed {} {}", x, y);
                o.energy = 0;
                o.flashed = true;
                flashed = true;
            }
        });

        if !flashed {
            return false;
        }

        for ty in max(0, y as i32 - 1)..=min(self.height as i32 - 1, y as i32 + 1) {
            for tx in max(0, x as i32 - 1)..=min(self.width as i32 - 1, x as i32 + 1) {
                self.add_energy(tx as u32, ty as u32);
            }
        }

        true
    }

    fn add_energy_all(self: &mut Board) -> u32 {
        // Reset flashing
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells
                    .entry((y * self.width) + x)
                    .and_modify(|o| o.flashed = false);
            }
        }

        // Add one jolt of energy to everybody
        for y in 0..self.height {
            for x in 0..self.width {
                self.add_energy(x, y);
            }
        }

        // Loop until nobody has flashed
        let mut rv = 0;
        loop {
            let test_rv = rv;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.trigger_flash(x, y) {
                        rv += 1;
                    }
                }
            }
            if test_rv == rv {
                // No flashes left
                break;
            }
        }
        rv
    }

    fn print(self: &Board) {
        for y in 0..self.height {
            println!("");
            for x in 0..self.width {
                let oct = self.cells.get(&((y * self.width) + x)).unwrap();
                print!("{}", oct.energy);
            }
        }
        println!("");
    }
}

fn parse_board(input: &str) -> Board {
    let mut cells: HashMap<u32, Octopus> = HashMap::new();

    let mut width: u32 = 0;
    let mut y: u32 = 0;
    for line in input.split("\n") {
        let mut x: u32 = 0;
        if width == 0 {
            width = line.len() as u32;
        }

        for level in line
            .split("")
            .filter(|s| *s != "")
            .map(|s| s.parse::<u8>().unwrap())
        {
            cells.insert(
                (y * width) + x,
                Octopus {
                    energy: level,
                    flashed: false,
                },
            );
            x += 1;
        }
        y += 1;
    }

    Board {
        cells: cells,
        width: width,
        height: y,
    }
}

fn part_one(input: &str) -> u32 {
    let mut board = parse_board(input);
    //board.print();
    let mut rv = 0;
    for _ in 0..100 {
        rv += board.add_energy_all();
        //board.print();
    }
    rv
}

fn part_two(input: &str) -> u32 {
    let mut board = parse_board(input);
    //board.print();

    let mut rv = 0;
    loop {
        rv += 1;
        if board.add_energy_all() == (board.width * board.height) {
            //board.print();
            return rv;
        }
        //board.print();
    }
}

fn main() {
    let input = include_str!("day11.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
