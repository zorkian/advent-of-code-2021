use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn segmentize(segment: &str) -> u8 {
    let mut rv = 0;

    for char in segment.split("") {
        match char {
            "a" => rv = 0,
            "b" => rv = 1,
            "c" => rv = 2,
            "d" => rv = 3,
            "e" => rv = 4,
            "f" => rv = 5,
            "g" => rv = 6,
            _ => panic!("nope"),
        };
    }

    rv
}

fn digit_to_segments(digit: u8) -> Vec<u8> {
    match digit {
        0 => vec![0, 1, 2, 4, 5, 6],    //vec!["a", "b", "c", "e", "f", "g"],
        1 => vec![2, 5],                //vec!["c", "f"],
        2 => vec![0, 2, 3, 4, 6],       //vec!["a", "c", "d", "e", "g"],
        3 => vec![0, 2, 3, 4, 5, 6],    //vec!["a", "c", "d", "e", "f", "g"],
        4 => vec![1, 2, 3, 5],          //vec!["b", "c", "d", "f"],
        5 => vec![0, 1, 3, 5, 6],       //vec!["a", "b", "d", "f", "g"],
        6 => vec![0, 1, 3, 4, 5, 6],    //vec!["a", "b", "d", "e", "f", "g"],
        7 => vec![0, 2, 5],             //vec!["a", "c", "f"],
        8 => vec![0, 1, 2, 3, 4, 5, 6], //vec!["a", "b", "c", "d", "e", "f", "g"],
        9 => vec![0, 1, 2, 3, 5, 6],    //vec!["a", "b", "c", "d", "f", "g"],
        _ => panic!("nope"),
    }
}

fn ids_to_possibles(digit: &str) -> Vec<u8> {
    let mut rv = Vec::new();

    match digit.len() {
        2 => rv.push(1),
        3 => rv.push(7),
        4 => rv.push(4),
        5 => {
            rv.push(2);
            rv.push(3);
            rv.push(5);
        }
        6 => {
            rv.push(0);
            rv.push(6);
            rv.push(9);
        }
        7 => rv.push(8),
        _ => panic!("no matches"),
    }

    rv
}

fn digitize(segment: &str) -> HashSet<&str> {
    HashSet::from_iter(segment.split("").filter(|f| *f != ""))
}

fn part_one(input: &str) -> u32 {
    let mut rv = 0;
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" | ").collect();
        for part in parts[1].split(" ") {
            match part.len() {
                2 => rv += 1,
                3 => rv += 1,
                4 => rv += 1,
                7 => rv += 1,
                _ => {}
            }
        }
    }

    rv
}

/*
      0
    1   2
      3
    4   5
      6

dafe fe cef agdebfc fecdg bedcg bdcafg beafcg dcegaf fdcag

    fe => { f => {2, 5}, e => {2, 5} }
    cef => { c => {0}, e => {2, 5}, f => {2, 5} }
    dafe => { e => {2, 5}, f => {2, 5}, d => {1, 3}, a => {1, 3} }

    fe => { 2 => { f, e }, 5 => { f, e } }
    cef => { 0 => c, 2 => { f, e }, 5 => { f, e } }
    dafe => { 2 => { f, e }, 5 => { f, e }, 3 => { d, a }, 1 => { d, a } }
    fecdg =>
        { 0 c, 2 f e, 3 4 6 }

*/

fn validate_number(number: u8) -> u8 {
    match number {
        0b01110111u8 => 0,
        0b00100100u8 => 1,
        0b01011101u8 => 2,
        0b01101101u8 => 3,
        0b00101110u8 => 4,
        0b01101011u8 => 5,
        0b01111011u8 => 6,
        0b00100101u8 => 7,
        0b01111111u8 => 8,
        0b01101111u8 => 9,
        _ => 255,
    }
}

fn to_number(mapping: &HashMap<&str, u8>, number: &str) -> u8 {
    // Convert a set of segments to a number based on what lights up
    let mut num = 0;
    for dig in number.split("").filter(|s| *s != "") {
        if !mapping.contains_key(dig) {
            panic!("segment not in map");
        }
        num += 1 << mapping[dig];
    }
    validate_number(num)
}

fn part_two(input: &str) -> u32 {
    let mut rv = 0;
    let all_segments: HashSet<&str> = HashSet::from_iter(["a", "b", "c", "d", "e", "f", "g"]);

    for line in input.split("\n") {
        let mut mappings: HashMap<&str, u8> = HashMap::new();
        let mut known_segments: HashSet<&str> = HashSet::new();

        // parse input
        let parts: Vec<&str> = line.split(" | ").collect();
        let mut left: Vec<&str> = parts[0].split(" ").collect();
        let right: Vec<&str> = parts[1].split(" ").collect();

        // Sort the left by size (does this matter? who knows)
        left.sort_by(|a, b| a.len().cmp(&b.len()));

        // Digitize the segments
        let digs: Vec<HashSet<&str>> = left
            .iter()
            .map(|s| HashSet::from_iter(s.split("").filter(|f| *f != "")))
            .collect();

        // 0 = 1
        // 1 = 7
        // 2 = 4
        // 3 = 2, 3, 5
        // 4 = 2, 3, 5
        // 5 = 2, 3, 5
        // 6 = 0, 6, 9
        // 7 = 0, 6, 9
        // 8 = 0, 6, 9
        // 9 = 8

        // Top LED (0) is difference of first two
        let top_segment: &str = &digs[1]
            .difference(&digs[0])
            .map(|f| *f)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(&top_segment);
        mappings.insert(top_segment, 0);

        // Intersect to get bottom segment
        let bottom_segment: &str = digs[3]
            .iter()
            .filter(|k| digs[4].contains(*k))
            .filter(|k| digs[5].contains(*k))
            .filter(|k| !digs[1].contains(*k))
            .filter(|k| !digs[2].contains(*k))
            .map(|k| *k)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(&bottom_segment);

        // Get middle segment by redoing half the work from above
        let middle_segment: &str = digs[3]
            .iter()
            .filter(|k| digs[4].contains(*k))
            .filter(|k| digs[5].contains(*k))
            .filter(|k| *k != &top_segment && *k != &bottom_segment)
            .map(|k| *k)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(&middle_segment);

        // Get bottom left by starting with everything and removing what we
        // know and 4
        let bottom_left_segment: &str = all_segments
            .iter()
            .filter(|k| !digs[2].contains(*k))
            .filter(|k| !known_segments.contains(*k))
            .map(|k| *k)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(&bottom_left_segment);

        // Find the top right by testing each of the 5-lengths until we get
        // the 2
        let mut top_right_segment: &str = "NOPE";
        for idx in [3 as usize, 4, 5] {
            let test = digs[idx]
                .iter()
                .filter(|k| !known_segments.contains(*k))
                .map(|k| *k)
                .collect::<Vec<&str>>();
            if test.len() == 1 {
                top_right_segment = test[0];
            }
        }
        known_segments.insert(&top_right_segment);

        // Find the bottom right by checking against 1
        let bottom_right_segment: &str = digs[0]
            .iter()
            .filter(|k| !known_segments.contains(*k))
            .map(|k| *k)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(&bottom_right_segment);

        // Top left segment is whatever's left
        let top_left_segment: &str = all_segments
            .iter()
            .filter(|k| !known_segments.contains(*k))
            .map(|k| *k)
            .collect::<Vec<&str>>()[0];
        known_segments.insert(top_left_segment);

        // Now we can set mappings and then numberize
        mappings.insert(top_segment, 0);
        mappings.insert(top_left_segment, 1);
        mappings.insert(top_right_segment, 2);
        mappings.insert(middle_segment, 3);
        mappings.insert(bottom_left_segment, 4);
        mappings.insert(bottom_right_segment, 5);
        mappings.insert(bottom_segment, 6);

        // Numberize the right side
        let mut real_num: u32 = 0;
        for idx in 0..4 {
            let num = to_number(&mappings, right[idx]);
            real_num += num as u32 * u32::pow(10, 3 - idx as u32);
        }
        rv += real_num;
    }

    rv
}

fn part_two_old(input: &str) -> u32 {
    let mut rv = 0;
    let all_segments: HashSet<&str> = HashSet::from_iter(["a", "b", "c", "d", "e", "f", "g"]);

    for line in input.split("\n") {
        // Set up the full set of unknowns (i.e., all segments map to all
        // possible input wires)
        let mut mappings: HashMap<u8, HashSet<&str>> = HashMap::new();
        for idx in 0..7 {
            mappings.insert(idx, all_segments.clone());
        }

        // parse input
        let parts: Vec<&str> = line.split(" | ").collect();
        let mut left: Vec<&str> = parts[0].split(" ").collect();
        let right: Vec<&str> = parts[1].split(" ").collect();

        // Sort the left by size (does this matter? who knows)
        left.sort_by(|a, b| a.len().cmp(&b.len()));

        // Iterate each on the left and try to deduce the signal lighting
        let mut seen_segments: HashSet<&str> = HashSet::new();
        let mut unseen_segments = all_segments.clone();
        for segment in left {
            // Segments is which input segments are active (a, b, etc)
            let mut segments = digitize(&segment);
            unseen_segments.retain(|k| !segments.contains(*k));

            // Remove input segments we've already seen (since we're going from less
            // information to more information...)
            dbg!(&segment, &segments);
            segments.retain(|k| !seen_segments.contains(*k));

            // Possibility is a digit this could be (0, 1, 2, etc)
            for possibility in ids_to_possibles(&segment) {
                // Lit segments are output lights we know are active (0, 1, 2, etc)
                let mut lit_segments = digit_to_segments(possibility);

                // Iterate each possibly lit segment and either intersect it with our
                // seen segments if we know that it's lit, or the unseen segments if we
                // know that it's dark
                for test_segment in 0..7 {
                    if lit_segments.contains(&test_segment) {
                        // Lit
                        mappings.entry(test_segment).and_modify(|m| {
                            let hs_test = HashSet::from_iter(m.intersection(&segments).map(|m| *m));
                            if hs_test.len() >= 1 {
                                *m = hs_test;
                            }
                        });
                    } else {
                        // Unlit
                        mappings.entry(test_segment).and_modify(|m| {
                            let hs_test =
                                HashSet::from_iter(m.intersection(&unseen_segments).map(|m| *m));
                            if hs_test.len() >= 1 {
                                *m = hs_test;
                            }
                        });
                    }
                }
            }

            // Now we have "seen" these segments, record them forever
            for segment in &segments {
                seen_segments.insert(segment);
            }
            dbg!(&segments, &seen_segments, &unseen_segments, &mappings);
        }
    }

    rv
}

fn main() {
    let input = include_str!("day8.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
