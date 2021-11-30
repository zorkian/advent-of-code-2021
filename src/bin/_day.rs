fn do_something(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("_day.txt");
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
