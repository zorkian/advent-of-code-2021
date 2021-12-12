use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum CaveSize {
    Unknown,
    Small,
    Large,
}

#[derive(Debug)]
struct Cave {
    name: String,
    size: CaveSize,
    exits: HashSet<String>,
}

impl Cave {
    fn new(name: &String) -> Cave {
        let mut size: CaveSize = CaveSize::Unknown;
        match name.chars().next().unwrap() {
            'a'..='z' => size = CaveSize::Small,
            'A'..='Z' => size = CaveSize::Large,
            _ => {}
        }
        Cave {
            name: name.clone(),
            size: size,
            exits: HashSet::new(),
        }
    }

    fn count_paths(
        self: &Cave,
        caves: &HashMap<String, Cave>,
        path: &mut Vec<String>,
        allow_duplicate_small: bool,
    ) -> u32 {
        // If we are the end, return 1 (exit condition)
        if self.name == "end" {
            return 1;
        }

        // If we're the start and we recursed, don't do that
        if self.name == "start" && path.len() > 0 {
            return 0;
        }

        // If we are small, and we are already in the path, then we no longer
        // allow duplicate smalls
        let mut should_allow_duplicate_small = allow_duplicate_small;
        if self.size == CaveSize::Small && path.contains(&self.name) {
            should_allow_duplicate_small = false;
        }

        // We have now visited this room
        path.push(self.name.clone());

        // Try all paths that we haven't already visited
        let mut rv = 0;
        for exit in &self.exits {
            if let Some(cave) = caves.get(exit) {
                if cave.size == CaveSize::Small
                    && path.contains(&cave.name)
                    && !should_allow_duplicate_small
                {
                    // Already visited this small cave and we aren't allowing duplicates
                    // at this time
                    continue;
                }

                // Visit this large cave
                rv += cave.count_paths(&caves, path, should_allow_duplicate_small);
            }
        }

        // Before we exit, remove ourselves from the path
        path.pop();

        rv
    }
}

fn parse_input(input: &str) -> HashMap<String, Cave> {
    let mut rv: HashMap<String, Cave> = HashMap::new();

    for line in input.split("\n") {
        let parts: Vec<String> = line.split("-").map(|s| s.to_string()).collect();

        // Insert the forward path
        let cave = rv.entry(parts[0].clone()).or_insert(Cave::new(&parts[0]));
        cave.exits.insert(parts[1].clone());

        // Insert the backwards path
        let cave_dest = rv.entry(parts[1].clone()).or_insert(Cave::new(&parts[1]));
        cave_dest.exits.insert(parts[0].clone());
    }

    rv
}

fn part_one(input: &str) -> u32 {
    let caves = parse_input(input);

    // Start at room, maintain list of rooms we've been in at least once as
    // a set, and see how many ways we can get to end
    let mut path: Vec<String> = Vec::new();
    let start = caves.get("start").unwrap();

    start.count_paths(&caves, &mut path, false)
}

fn part_two(input: &str) -> u32 {
    let caves = parse_input(input);

    // Start at room, maintain list of rooms we've been in at least once as
    // a set, and see how many ways we can get to end
    let mut path: Vec<String> = Vec::new();
    let start = caves.get("start").unwrap();

    start.count_paths(&caves, &mut path, true)
}

fn main() {
    let input = include_str!("day12.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
