fn do_something(input: &str) -> u32 {
    let lines: Vec<u32> = input
        .split("\n")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut v1 = 0;
    let mut v2 = 0;
    let mut v3 = 0;

    let mut incrs = 0;

    for i in lines {
        if v1 == 0 {
            v1 = i;
        } else if v2 == 0 {
            v2 = i;
        } else if v3 == 0 {
            v3 = i;
        } else {
            // We are on our fourth number at least, so compare
            if v2 + v3 + i > v1 + v2 + v3 {
                incrs += 1;
            }
            v1 = v2;
            v2 = v3;
            v3 = i;
        }
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
