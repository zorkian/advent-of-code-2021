fn part_one(input: &str) -> u32 {
    let mut bits: Vec<Vec<usize>> = Vec::new();

    for line in input.split("\n") {
        let mut bit: Vec<usize> = Vec::new();
        for num in line.split("") {
            if num == "0" || num == "1" {
                bit.push(num.parse::<usize>().unwrap());
            }
        }
        bits.push(bit);
    }

    let cols = bits[0].len();
    let rows = bits.len();

    let mut gamma = 0;
    let mut epsilon = 0;
    for col in 0..cols {
        let mut count = 0;
        for row in 0..rows {
            count += bits[row][col];
        }

        if count > (rows - count) {
            gamma += 1 << (cols - col - 1);
        } else {
            epsilon += 1 << (cols - col - 1);
        }
    }

    gamma * epsilon
}

fn filter_bits(inps: &Vec<Vec<usize>>, compare: &Vec<usize>) -> Vec<Vec<usize>> {
    if compare.len() == 0 {
        return inps.clone();
    }

    let mut rv: Vec<Vec<usize>> = Vec::new();

    for test in inps {
        let mut keep = true;
        for i in 0..compare.len() {
            if test[i] != compare[i] {
                keep = false;
            }
        }
        if keep {
            rv.push(test.clone());
        }
    }

    rv
}

fn most_common_bit(inps: &Vec<Vec<usize>>, pos: usize, tie_return: usize) -> usize {
    let mut count = 0;
    let len = inps.len();
    for row in 0..inps.len() {
        count += inps[row][pos];
    }
    if count > (len - count) {
        1
    } else if count == (len - count) {
        tie_return
    } else {
        0
    }
}

fn part_two(input: &str) -> u32 {
    let mut bits: Vec<Vec<usize>> = Vec::new();

    for line in input.split("\n") {
        let mut bit: Vec<usize> = Vec::new();
        for num in line.split("") {
            if num == "0" || num == "1" {
                bit.push(num.parse::<usize>().unwrap());
            }
        }
        bits.push(bit);
    }

    let cols = bits[0].len();

    let mut oxy = 0;
    let mut co2 = 0;

    // find oxygen
    let mut filter: Vec<usize> = Vec::new();
    let mut filtered = bits.clone();
    for col in 0..cols {
        let common_bit = most_common_bit(&filtered, col, 1);
        filter.push(common_bit);
        filtered = filter_bits(&bits, &filter);
        if filtered.len() == 1 {
            for col in 0..cols {
                if filtered[0][col] == 1 {
                    oxy += 1 << (cols - col - 1);
                }
            }
            break;
        }
    }

    // find co2
    let mut filter: Vec<usize> = Vec::new();
    let mut filtered = bits.clone();
    for col in 0..cols {
        let common_bit = most_common_bit(&filtered, col, 1);
        let least_common_bit = if common_bit == 1 { 0 } else { 1 };
        filter.push(least_common_bit);
        filtered = filter_bits(&bits, &filter);
        if filtered.len() == 1 {
            for col in 0..cols {
                if filtered[0][col] == 1 {
                    co2 += 1 << (cols - col - 1);
                }
            }
            break;
        }
    }

    oxy * co2
}

fn main() {
    let input = include_str!("day3.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
