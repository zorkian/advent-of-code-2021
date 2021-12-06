fn run_and_simulate(input: &str, simulate_for: u64) -> u64 {
    let fish: Vec<u64> = input
        .split(",")
        .map(|f| f.parse::<u64>().unwrap())
        .collect();

    let mut days: Vec<u64> = vec![0; 9];
    for i in fish {
        days[i as usize] += 1;
    }

    let mut day: u64 = 0;
    loop {
        // Which index is about to birth new fish
        let birth_idx = day % 9;

        // Which index new fish will go in
        let reset_idx = (day + 7) % 9;

        // Any fish that are on the birth index get added to
        // the reset index, this is the only thing we have to do ??
        days[reset_idx as usize] += days[birth_idx as usize];

        // Increase simulation counter and exit if we're at breakpoint
        day += 1;
        if day == simulate_for {
            break;
        }
    }

    let mut counter = 0;
    for day in days {
        counter += day;
    }
    counter
}

fn part_one(input: &str) -> u64 {
    run_and_simulate(input, 80)
}

fn part_two(input: &str) -> u64 {
    run_and_simulate(input, 256)
}

fn main() {
    let input = include_str!("day6.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
