use std::cmp::{max, min};
use std::collections::HashMap;

fn fuel_calc(from_pos: u32, to_pos: u32, part: u32) -> u32 {
    let min_pos = min(from_pos, to_pos);
    let max_pos = max(from_pos, to_pos);
    let delta: f32 = max_pos as f32 - min_pos as f32;

    if part == 1 {
        delta as u32
    } else {
        let result = (delta / 2 as f32) * (1 + (max_pos - min_pos)) as f32;
        result as u32
    }
}

fn move_crabs(input: &str, part: u32) -> u32 {
    let nums: Vec<u32> = input
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut crabs: HashMap<u32, u32> = HashMap::new();
    let mut min_pos = 24092834;
    let mut max_pos = 0;
    for num in nums {
        let count = crabs.entry(num).or_insert(0);
        *count += 1;
        min_pos = min(num, min_pos);
        max_pos = max(num, max_pos);
    }

    let mut min_fuel = 230400234;

    for idx in min_pos..max_pos + 1 {
        let mut fuel = 0;
        for (crab, count) in &crabs {
            if *crab == idx {
                continue;
            } else if *crab > idx {
                fuel += fuel_calc(*crab, idx, part) * count
            } else {
                fuel += fuel_calc(*crab, idx, part) * count
            }
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn part_one(input: &str) -> u32 {
    move_crabs(input, 1)
}

fn part_two(input: &str) -> u32 {
    move_crabs(input, 2)
}

fn main() {
    let input = include_str!("day7.txt");

    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
