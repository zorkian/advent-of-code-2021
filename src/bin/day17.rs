use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum ProbeLocation {
    Undershoot,
    Inside,
    Overshoot,
}

#[derive(Debug)]
struct Bounds {
    tl: Point,
    br: Point,
}

impl Bounds {
    fn compare(self: &Bounds, probe: &Probe) -> ProbeLocation {
        // If it's past the right edge or below the bottom edge, it has overshot
        // and is never coming back
        if probe.x > self.br.x || probe.y < self.br.y {
            return ProbeLocation::Overshoot;
        }

        if probe.x >= self.tl.x && probe.y <= self.tl.y {
            if probe.x <= self.br.x && probe.y >= self.br.y {
                return ProbeLocation::Inside;
            } else {
                return ProbeLocation::Overshoot;
            }
        } else {
            return ProbeLocation::Undershoot;
        }
    }
}

#[derive(Debug)]
struct Probe {
    step: i32,
    x: i32,
    y: i32,
    xv: i32,
    yv: i32,
}

impl Probe {
    fn new(xv: i32, yv: i32) -> Probe {
        Probe {
            step: 0,
            x: 0,
            y: 0,
            xv,
            yv,
        }
    }

    fn step(self: &mut Probe) {
        self.step += 1;
        self.x += self.xv;
        self.y += self.yv;
        self.xv += match self.xv {
            i32::MIN..=-1 => 1,
            0 => 0,
            1..=i32::MAX => -1,
        };
        self.yv -= 1;
    }
}

fn parse_input(input: &str) -> Bounds {
    let input_regex = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();

    let caps = input_regex.captures(input).unwrap();

    Bounds {
        tl: Point {
            x: caps[1].parse::<i32>().unwrap(),
            y: caps[4].parse::<i32>().unwrap(),
        },
        br: Point {
            x: caps[2].parse::<i32>().unwrap(),
            y: caps[3].parse::<i32>().unwrap(),
        },
    }
}

fn part_one(input: &str) -> u32 {
    let bounds = parse_input(input);

    // Start probing X and Y in a square until we've figure out the best Y
    let mut rv = 0;

    for yv in 1..100 {
        for xv in 1..100 {
            let mut probe = Probe::new(xv, yv);
            let mut rv_this = 0;
            let mut was_inside = false;
            loop {
                probe.step();
                rv_this = max(rv_this, probe.y);
                match bounds.compare(&probe) {
                    ProbeLocation::Undershoot => (),
                    ProbeLocation::Inside => was_inside = true,
                    ProbeLocation::Overshoot => break,
                }
                if probe.step > 1000 {
                    panic!("probe too long!?");
                }
            }
            if was_inside {
                rv = max(rv, rv_this);
            }
        }
    }

    rv as u32
}

fn part_two(input: &str) -> u32 {
    let bounds = parse_input(input);

    // Start probing X and Y in a square until we've figure out the count
    let mut rv = 0;

    for yv in -150..200 {
        for xv in 1..200 {
            let mut probe = Probe::new(xv, yv);
            loop {
                probe.step();
                match bounds.compare(&probe) {
                    ProbeLocation::Undershoot => (),
                    ProbeLocation::Inside => {
                        rv += 1;
                        break;
                    }
                    ProbeLocation::Overshoot => break,
                }
                if probe.step > 1000 {
                    panic!("probe too long!?");
                }
            }
        }
    }

    rv as u32
}

fn main() {
    let input = include_str!("day17.txt");
    // println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
