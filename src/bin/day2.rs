fn part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut horiz = 0;
    let mut vert = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let num = parts[1].parse::<u32>().unwrap();
        match parts[0] {
            "forward" => horiz += num,
            "up" => vert -= num,
            "down" => vert += num,
            _ => {}
        }
    }

    return horiz * vert;
}

fn part_two(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut horiz = 0;
    let mut vert = 0;
    let mut aim = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let num = parts[1].parse::<u32>().unwrap();
        match parts[0] {
            "forward" => {
                horiz += num;
                vert += aim * num;
            }
            "up" => aim -= num,
            "down" => aim += num,
            _ => {}
        }
    }

    return horiz * vert;
}

fn main() {
    let input = include_str!("day2.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
