use std::collections::HashMap;

struct Cave {
    cells: Vec<Vec<u8>>,
    basin_id: HashMap<u32, u32>,
    basin_size: HashMap<u32, u32>,
}

impl Cave {
    fn new() -> Cave {
        Cave {
            cells: Vec::new(),
            basin_id: HashMap::new(),
            basin_size: HashMap::new(),
        }
    }

    fn get_cell(self: &Cave, ix: i32, iy: i32) -> Option<u8> {
        if iy < 0
            || iy >= self.cells.len() as i32
            || ix < 0
            || ix >= self.cells[iy as usize].len() as i32
        {
            return None;
        }
        Some(self.cells[iy as usize][ix as usize])
    }

    fn is_low_point(self: &Cave, x: u32, y: u32) -> bool {
        let ix = x as i32;
        let iy = y as i32;

        let points = [
            self.get_cell(ix - 1, iy),
            self.get_cell(ix, iy - 1),
            self.get_cell(ix + 1, iy),
            self.get_cell(ix, iy + 1),
        ];

        let self_height = self.get_cell(ix, iy).unwrap();

        for point in points {
            match point {
                Some(height) => {
                    if height <= self_height {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn start_new_basin(self: &mut Cave, x: u32, y: u32) {
        let basin_id = *self.basin_size.keys().max().unwrap_or(&0) + 1;
        self.infect_or_exit(x as i32, y as i32, basin_id);
    }

    fn infect_or_exit(self: &mut Cave, ix: i32, iy: i32, basin_id: u32) {
        if self.get_cell(ix, iy).unwrap_or(9) == 9 {
            // No cell or it's a wall, move on
            return;
        }

        if self.get_basin_id(ix, iy).unwrap_or(0) > 0 {
            // Already infected, don't recurse forever
            return;
        }

        println!("Infected {}, {} with {}", ix, iy, basin_id);
        self.set_basin_id(ix as u32, iy as u32, basin_id);

        // Begin infecting from neighboring points
        self.infect_or_exit(ix - 1, iy, basin_id);
        self.infect_or_exit(ix, iy - 1, basin_id);
        self.infect_or_exit(ix + 1, iy, basin_id);
        self.infect_or_exit(ix, iy + 1, basin_id);
    }

    fn set_basin_id(self: &mut Cave, x: u32, y: u32, basin_id: u32) {
        self.basin_id
            .entry(y * self.cells[0].len() as u32 + x)
            .or_insert(basin_id);
        *self.basin_size.entry(basin_id).or_insert(0) += 1;
    }

    fn get_basin_id(self: &Cave, ix: i32, iy: i32) -> Option<u32> {
        if iy < 0
            || iy >= self.cells.len() as i32
            || ix < 0
            || ix >= self.cells[iy as usize].len() as i32
        {
            return None;
        }

        let addr = (iy * self.cells[0].len() as i32 + ix) as u32;
        Some(*self.basin_id.get(&addr).unwrap_or(&0))
    }

    fn print_basin_map(self: &Cave) {
        for y in 0..self.cells.len() as u32 {
            println!("");
            for x in 0..self.cells[y as usize].len() as u32 {
                let basin_id = self.get_basin_id(x as i32, y as i32).unwrap_or(9);
                print!("{}", basin_id);
            }
        }
    }
}

fn parse_cave(input: &str) -> Cave {
    let mut cave = Cave::new();
    for line in input.split("\n") {
        let points: Vec<u8> = line
            .split("")
            .filter(|f| *f != "")
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        cave.cells.push(points);
    }
    cave
}

fn part_one(input: &str) -> u32 {
    let cave = parse_cave(input);

    let mut risk: u32 = 0;
    for y in 0..cave.cells.len() {
        for x in 0..cave.cells[y].len() {
            if cave.is_low_point(x as u32, y as u32) {
                risk += cave.get_cell(x as i32, y as i32).unwrap() as u32 + 1;
            }
        }
    }

    risk
}

fn part_two(input: &str) -> u32 {
    let mut cave = parse_cave(input);

    for y in 0..cave.cells.len() as u32 {
        for x in 0..cave.cells[y as usize].len() as u32 {
            cave.start_new_basin(x, y);
        }
    }

    let mut sorted_basin_ids: Vec<u32> = cave.basin_size.keys().map(|f| *f).collect();
    sorted_basin_ids.sort_by(|a, b| {
        cave.basin_size
            .get(b)
            .unwrap()
            .cmp(cave.basin_size.get(a).unwrap())
    });

    // cave.print_basin_map();

    let mut rv: u32 = 1;
    rv *= cave.basin_size.get(&sorted_basin_ids[0]).unwrap();
    rv *= cave.basin_size.get(&sorted_basin_ids[1]).unwrap();
    rv *= cave.basin_size.get(&sorted_basin_ids[2]).unwrap();
    rv
}

fn main() {
    let input = include_str!("day9.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
