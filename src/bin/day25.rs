use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct World {
    width: u32,
    height: u32,
    points: HashMap<Point, char>,
}

fn parse_input(input: &str) -> World {
    let mut y = 0;
    let mut x = 0;

    let mut rv = World {
        width: 0,
        height: 0,
        points: HashMap::new(),
    };

    for line in input.split("\n") {
        x = 0;
        for char in line.chars() {
            match char {
                '.' => {}
                'v' => {
                    rv.points.entry(Point { x, y }).or_insert('v');
                }
                '>' => {
                    rv.points.entry(Point { x, y }).or_insert('>');
                }
                _ => panic!("invalid char"),
            };
            x += 1;
        }
        y += 1;
    }

    rv.width = x;
    rv.height = y;

    rv
}

impl World {
    fn step(self: &World) -> (u32, World) {
        let mut rv = World {
            width: self.width,
            height: self.height,
            points: HashMap::new(),
        };

        let mut east: Vec<Point> = Vec::new();
        let mut south: Vec<Point> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(point) = self.points.get(&Point { x, y }) {
                    if *point == '>' {
                        east.push(Point { x, y });
                    } else {
                        south.push(Point { x, y });
                    }
                }
            }
        }

        let mut movements = 0;

        for point in east {
            let test_point = Point {
                x: (point.x + 1) % (self.width),
                y: point.y,
            };
            if self.points.get(&test_point).is_none() {
                movements += 1;
                rv.points.entry(test_point).or_insert('>');
            } else {
                rv.points.entry(point).or_insert('>');
            }
        }

        for point in south {
            let test_point = Point {
                x: point.x,
                y: (point.y + 1) % (self.height),
            };

            // Have to check rv (new) for moved >'s and self (old) for existing
            // and unmoved v's (only)
            let mut did_collide = false;
            if let Some(test_char) = self.points.get(&test_point) {
                did_collide = *test_char == 'v';
            }

            if rv.points.get(&test_point).is_none() && !did_collide {
                movements += 1;
                rv.points.entry(test_point).or_insert('v');
            } else {
                rv.points.entry(point).or_insert('v');
            }
        }

        (movements, rv)
    }

    fn print(self: &World) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(point) = self.points.get(&Point { x, y }) {
                    print!("{}", *point);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

fn part_one(input: &str) -> u32 {
    let mut input = parse_input(input);

    let mut rv = 0;
    loop {
        rv += 1;
        // input.print();
        let (steps, tinput) = input.step();
        if steps == 0 {
            break;
        }
        input = tinput;
    }

    rv
}

fn part_two(input: &str) -> u32 {
    let mut input = parse_input(input);

    0
}

fn main() {
    let input = include_str!("day25.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
