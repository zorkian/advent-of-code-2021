fn part_one(input: &str) -> u32 {
    let mut rv = 0;

    for line in input.split("\n") {
        let mut chars: Vec<&str> = Vec::new();
        for char in line.split("") {
            match char {
                "{" | "[" | "<" | "(" => {
                    chars.push(char);
                }
                _ => {
                    if let Some(test_char) = chars.pop() {
                        match (test_char.to_owned() + char).as_str() {
                            "<>" | "[]" | "{}" | "()" => continue,
                            _ => match char {
                                ")" => rv += 3,
                                "]" => rv += 57,
                                "}" => rv += 1197,
                                ">" => rv += 25137,
                                _ => {}
                            },
                        }
                        break;
                    }
                }
            }
        }
        dbg!(rv);
    }

    rv
}

fn part_two(input: &str) -> u64 {
    let mut scores: Vec<u64> = Vec::new();

    for line in input.split("\n") {
        let mut chars: Vec<&str> = Vec::new();
        let mut corrupt = true;

        for char in line.split("") {
            match char {
                "{" | "[" | "<" | "(" => {
                    chars.push(char);
                }
                _ => {
                    if let Some(test_char) = chars.pop() {
                        match (test_char.to_owned() + char).as_str() {
                            "<>" | "[]" | "{}" | "()" => continue,
                            _ => match char {
                                "" => {
                                    corrupt = false;
                                    chars.push(test_char);
                                }
                                _ => {}
                            },
                        }
                        break;
                    }
                }
            }
        }

        if corrupt {
            continue;
        }

        let mut score: u64 = 0;
        while let Some(char) = chars.pop() {
            score *= 5;

            match char {
                "(" => score += 1,
                "[" => score += 2,
                "{" => score += 3,
                "<" => score += 4,
                _ => panic!("oh no"),
            }
        }
        scores.push(score);
    }

    scores.sort_by(|a, b| a.cmp(b));

    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("day10.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
