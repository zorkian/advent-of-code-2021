use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
    z: i64,
    distances: HashSet<u64>,
}

impl Beacon {
    fn new(x: i64, y: i64, z: i64) -> Beacon {
        Beacon {
            x,
            y,
            z,
            distances: HashSet::new(),
        }
    }

    fn distance_to(&self, target: &Beacon) -> u64 {
        ((((self.x - target.x).pow(2) + (self.y - target.y).pow(2) + (self.z - target.z).pow(2))
            as f64)
            .sqrt()) as u64
    }

    fn new_coords(&self, facing: u32, rotation: u32) -> Beacon {
        let mut rv = Beacon::new(self.x, self.y, self.z);

        // -x = left, +x = right
        // -y = up, +y = down
        // -z = inwards, +z = outwards
        // Facings: 0 = default, 1 = right, 2 = behind, 3 = left, 4 = up, 5 = down
        // Rotations: 0 = default, 1 = 90 deg, 2 = 180 deg, 3 = 270 deg

        match facing {
            0 => {}
            1 => {
                // Y axis 90
                let nx = rv.z; // z 0 - x 1
                let nz = -rv.x; // z 1 + x 0
                rv.x = nx;
                rv.z = nz;
            }
            2 => {
                // Y axis 180
                let nx = -rv.x; // z -1 - x 0
                let nz = -rv.z; // z 0 + x -1
                rv.x = nx;
                rv.z = nz;
            }
            3 => {
                // Y axis 270
                let nx = -rv.z; // z 0 - x -1
                let nz = rv.x; // z -1 + x 0
                rv.x = nx;
                rv.z = nz;
            }
            4 => {
                // Z axis rotation 270
                let ny = rv.z;
                let nz = -rv.y;
                rv.y = ny;
                rv.z = nz;
            }
            5 => {
                // Z axis rotation 90
                let ny = -rv.z;
                let nz = rv.y;
                rv.y = ny;
                rv.z = nz;
            }
            _ => panic!("invalid facing"),
        };

        //       0, 90, 180, 270
        // cos = 1,  0,  -1,   0
        // sin = 0,  1,   0,  -1

        let deg90 = std::f64::consts::PI / 2f64;

        match rotation {
            0 => {}
            1 => {
                let nx = rv.y;
                let ny = -rv.x;
                rv.x = nx;
                rv.y = ny;
            }
            2 => {
                let nx = -rv.x;
                let ny = -rv.y;
                rv.x = nx;
                rv.y = ny;
            }
            3 => {
                let nx = -rv.y;
                let ny = rv.x;
                rv.x = nx;
                rv.y = ny;
            }
            _ => panic!("invalid rotation"),
        }

        rv
    }
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    offset_x: i64,
    offset_y: i64,
    offset_z: i64,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(id: u32) -> Scanner {
        Scanner {
            id,
            offset_x: 0,
            offset_y: 0,
            offset_z: 0,
            beacons: Vec::new(),
        }
    }

    fn calculate_distances(&mut self) {
        // Create full mesh of distances from every beacon to every beacon
        // println!("calc dist {}", self.id);
        for test_idx in 0..self.beacons.len() {
            for target_idx in 0..self.beacons.len() {
                if test_idx == target_idx {
                    continue;
                }
                let distance = self.beacons[test_idx].distance_to(&self.beacons[target_idx]);
                self.beacons[test_idx].distances.insert(distance);
            }
        }
    }

    fn num_overlaps(&self, target: &Scanner) -> u32 {
        // Count how many of our beacons share a lot of overlaps with beacons in the
        // target scanner we're considering
        let mut rv = 0;
        for beacon in &self.beacons {
            for target_beacon in &target.beacons {
                let isect_count = beacon
                    .distances
                    .intersection(&target_beacon.distances)
                    .count();
                if isect_count >= 11 {
                    rv += 1;
                }
            }
        }
        rv
    }

    fn reorient(&self, target: &Scanner) -> Scanner {
        // We have a scanner, we need to identify a beacon that matches a different
        // beacon, and then calculate the rotation necessary to get target to be
        // back to the scanner's rotation
        let mut pairs: Vec<(&Beacon, &Beacon)> = Vec::new();
        for beacon in &self.beacons {
            for target_beacon in &target.beacons {
                // See if these are the same beacon (have at least 11 neighbors that
                // have the exact distance)
                let isect_count = beacon
                    .distances
                    .intersection(&target_beacon.distances)
                    .count();
                if isect_count >= 11 {
                    // These beacons are the same, we can use them to calculate first
                    // an orientation change and second
                    pairs.push((target_beacon, beacon));
                }
            }
        }

        // If we have no matching

        // Now we have a bunch of pairs of beacons from scanner A to scanner B,
        // let's try to calculate a rotation that will make all of the pairs
        // align up
        let test_pair = pairs.pop().unwrap();
        for facing in 0..6 {
            for rotation in 0..4 {
                // Let's rotate our test beacon by this and then use that to calculate
                // where the scanner would be, relative to the first scanner
                let test_beacon = test_pair.1.new_coords(facing, rotation);
                let sx = test_pair.0.x - test_beacon.x;
                let sy = test_pair.0.y - test_beacon.y;
                let sz = test_pair.0.z - test_beacon.z;
                // dbg!(facing, rotation, sx, sy, sz);

                let mut validated = true;
                for pair in &pairs {
                    let mut validate_beacon = pair.1.new_coords(facing, rotation);
                    validate_beacon.x += sx;
                    validate_beacon.y += sy;
                    validate_beacon.z += sz;

                    if pair.0.x != validate_beacon.x
                        || pair.0.y != validate_beacon.y
                        || pair.0.z != validate_beacon.z
                    {
                        // This does not validate, continue
                        validated = false;
                        break;
                    }
                }

                // If we've validated, we can create a new scanner, with fully rotated beacons
                // with this new facing, and return it
                if validated {
                    println!(
                        "VALIDATED id {}, facing {}, rotation {}, offsets {} {} {}",
                        self.id, facing, rotation, sx, sy, sz
                    );
                    let mut scanner = Scanner::new(self.id);
                    scanner.offset_x = sx;
                    scanner.offset_y = sy;
                    scanner.offset_z = sz;
                    scanner.beacons = Vec::from_iter(self.beacons.iter().map(|b| {
                        // Now rotate each of the beacons to its final home, and then apply
                        // the camera offset, so all beacons are now global
                        let mut new_beacon = b.new_coords(facing, rotation);
                        new_beacon.x += sx;
                        new_beacon.y += sy;
                        new_beacon.z += sz;
                        new_beacon
                    }));
                    scanner.calculate_distances();
                    return scanner;
                }
            }
        }

        panic!("no validation found");
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut rv = Vec::new();
    let mut scanner = Scanner::new(0);

    for line in input.split("\n") {
        if line.starts_with("---") {
            if scanner.beacons.len() > 0 {
                scanner.calculate_distances();
                rv.push(scanner);
                scanner = Scanner::new(0);
            }
            scanner.id = line.split(" ").collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap();
        } else if line.contains(",") {
            let coords: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
            scanner
                .beacons
                .push(Beacon::new(coords[0], coords[1], coords[2]));
        }
    }
    scanner.calculate_distances();
    rv.push(scanner);

    let mut mappings: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Calculate the overlapping scanners and determine mappings
    for test_idx in 0..rv.len() {
        for target_idx in test_idx..rv.len() {
            if test_idx == target_idx {
                continue;
            }

            let num_overlaps = rv[test_idx].num_overlaps(&rv[target_idx]);
            if num_overlaps < 10 {
                // Not enough confidence to allow it
                continue;
            }

            println!("{} .. {} => {}", test_idx, target_idx, num_overlaps);
            mappings
                .entry(test_idx)
                .or_insert(HashSet::new())
                .insert(target_idx);
            mappings
                .entry(target_idx)
                .or_insert(HashSet::new())
                .insert(test_idx);
        }
    }

    // Now we need to try to reorient everybody by chaining the mappings since we now
    // know which scanners have overlap with which
    let mut reoriented: HashSet<usize> = HashSet::from([0]);
    let mut still_need: HashSet<usize> = HashSet::from_iter(1..rv.len());
    while !still_need.is_empty() {
        let test: Vec<usize> = still_need.iter().map(|i| *i).collect();
        for test_idx in test {
            // If we can map this to anything that is already reoriented, do it
            for target_idx in mappings.get(&test_idx).unwrap() {
                if reoriented.contains(target_idx) {
                    // Reorient and mark it
                    rv[test_idx] = rv[test_idx].reorient(&rv[*target_idx]);
                    reoriented.insert(test_idx);
                    still_need.remove(&test_idx);
                }
            }
        }
    }

    rv
}

fn part_one(input: &str) -> u32 {
    let input = parse_input(input);

    // Now iterate all scanners, all beacons are now in their proper orientation and have
    // been collapsed to camera 0's coordinate system
    let mut beacon_coords = HashSet::new();
    for scanner in input {
        for beacon in scanner.beacons {
            beacon_coords.insert((0, beacon.x, beacon.y, beacon.z));
        }
    }

    let mut tmp: Vec<(u32, i64, i64, i64)> = beacon_coords.iter().map(|i| *i).collect();
    tmp.sort();
    for l in &tmp {
        println!("{},{},{},{}", l.0, l.1, l.2, l.3);
    }

    beacon_coords.len() as u32
}

fn part_two(input: &str) -> u32 {
    let input = parse_input(input);

    let mut rv = 0;
    for test_idx in 0..input.len() {
        for target_idx in 0..input.len() {
            if test_idx == target_idx {
                continue;
            }
            let distance = (input[test_idx].offset_x - input[target_idx].offset_x)
                + (input[test_idx].offset_y - input[target_idx].offset_y)
                + (input[test_idx].offset_z - input[target_idx].offset_z);
            rv = max(rv, distance);
        }
    }

    rv as u32
}

fn main() {
    let input = include_str!("day19.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::Beacon;

    #[test]
    fn it_works() {
        let beacon = Beacon::new(10, 5, 15);

        // -x = left, +x = right
        // -y = up, +y = down
        // -z = inwards, +z = outwards

        assert_eq!(beacon.new_coords(0, 0), Beacon::new(10, 5, 15));
        assert_eq!(beacon.new_coords(0, 1), Beacon::new(5, -10, 15));
        assert_eq!(beacon.new_coords(0, 2), Beacon::new(-10, -5, 15));
        assert_eq!(beacon.new_coords(0, 3), Beacon::new(-5, 10, 15));
        // Right
        assert_eq!(beacon.new_coords(1, 0), Beacon::new(15, 5, 10));
        // Behind
        assert_eq!(beacon.new_coords(2, 0), Beacon::new(-10, 5, -15));
        // Left
        assert_eq!(beacon.new_coords(3, 0), Beacon::new(15, 5, -10));
        // Up
        assert_eq!(beacon.new_coords(4, 0), Beacon::new(10, 15, -5));
        // Down
        assert_eq!(beacon.new_coords(5, 0), Beacon::new(10, 15, 5));
    }
}
