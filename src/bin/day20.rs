use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Image {
    algo: Vec<bool>,
    image: HashMap<Point, u16>,
}

impl Image {
    fn new() -> Image {
        Image {
            algo: Vec::new(),
            image: HashMap::new(),
        }
    }

    fn get(&self, x: i32, y: i32, void: bool) -> u16 {
        *self
            .image
            .get(&Point { x, y })
            .unwrap_or(if void { &1 } else { &0 })
    }

    fn get_around(&self, x: i32, y: i32, void: bool) -> u16 {
        let mut rv: u16 = 0;
        let mut ctr = 9;
        for ty in y - 1..=y + 1 {
            for tx in x - 1..=x + 1 {
                ctr -= 1;

                rv += self.get(tx, ty, void) << ctr;
            }
        }
        rv
    }

    fn set(&mut self, x: i32, y: i32, lit: u16) {
        *self.image.entry(Point { x, y }).or_default() = lit;
    }

    fn get_bounds(&self) -> (Point, Point) {
        let mut minp = Point {
            x: i32::MAX,
            y: i32::MAX,
        };
        let mut maxp = Point {
            x: i32::MIN,
            y: i32::MIN,
        };
        for point in self.image.keys() {
            minp.x = min(minp.x, point.x - 2);
            minp.y = min(minp.y, point.y - 2);
            maxp.x = max(maxp.x, point.x + 2);
            maxp.y = max(maxp.y, point.y + 2);
        }
        (minp, maxp)
    }

    fn print(&self, void: bool) {
        // Calculate the bounds of our image
        let (minp, maxp) = self.get_bounds();

        for ty in minp.y - 0..=maxp.y + 0 {
            for tx in minp.x - 0..=maxp.x + 0 {
                print!(
                    "{}",
                    if self.get(tx, ty, void) == 1 {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!("");
        }
    }

    fn enhance(&self, void: bool) -> Image {
        // Calculate the bounds of our image
        let (minp, maxp) = self.get_bounds();

        let mut rv = Image::new();
        rv.algo = self.algo.clone();
        for ty in minp.y..=maxp.y {
            for tx in minp.x..=maxp.x {
                let idx = self.get_around(tx, ty, void);
                rv.set(tx, ty, if self.algo[idx as usize] { 1 } else { 0 });
            }
        }

        rv
    }
}

fn parse_input(input: &str) -> Image {
    let mut rv = Image::new();

    let mut lines = input.split("\n");

    let algo = lines.next().unwrap();
    rv.algo = Vec::from_iter(algo.split("").filter(|c| *c != "").map(|c| {
        if c == "#" {
            true
        } else {
            false
        }
    }));
    assert_eq!(rv.algo.len(), 512);

    let _ = lines.next();

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for chr in line.split("") {
            if chr == "" {
                continue;
            }
            rv.set(x, y, if chr == "#" { 1 } else { 0 });
            x += 1;
        }
        y += 1;
    }

    rv
}

fn part_one(input: &str) -> u32 {
    let mut input = parse_input(input);

    let mut void_is_lit = false;
    for idx in 1..=2 {
        println!("Enhancement #{}:", idx);
        input = input.enhance(void_is_lit);
        input.print(void_is_lit);
        // If the void is off, see if it should be lit (position 0 in the algo)
        void_is_lit = if void_is_lit {
            input.algo[511]
        } else {
            input.algo[0]
        };
        println!("");
    }

    let mut rv = 0;
    input.image.values().for_each(|i| rv += *i);
    rv as u32
}

fn part_two(input: &str) -> u32 {
    let mut input = parse_input(input);

    let mut void_is_lit = false;
    for idx in 1..=50 {
        // println!("Enhancement #{}:", idx);
        input = input.enhance(void_is_lit);
        // input.print(void_is_lit);
        // If the void is off, see if it should be lit (position 0 in the algo)
        void_is_lit = if void_is_lit {
            input.algo[511]
        } else {
            input.algo[0]
        };
        // println!("");
    }

    let mut rv = 0;
    input.image.values().for_each(|i| rv += *i);
    rv as u32
}

fn main() {
    let input = include_str!("day20.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
