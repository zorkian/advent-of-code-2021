fn do_something(input: &str) -> u32 {
    let lines: Vec<u32> = input
        .split("\n")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut incrs = 0;
    let mut last_i = 0;
    for i in lines {
        if last_i != 0 && i > last_i {
            incrs += 1;
        }
        last_i = i;
    }
    return incrs;
}

fn main() {
    let input = include_str!("day1.txt");
    let answer = do_something(input);
    println!("ANSWER: {}", answer)
}

mod tests {
    use super::*;

    #[test]
    fn test_do_something() {
        assert_eq!(do_something(""), 0);
        assert_eq!(do_something(""), 0);
    }
}
