pub(crate) fn puzzle_a(input: &str) -> i64 {
    0
}

pub(crate) fn puzzle_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::{read_big_input_file, read_input_file};

    use super::*;

    const INPUT: &str = "";
    const RESULT_A: i64 = 0;
    const RESULT_B: i64 = 0;

    fn get_day() -> &'static str {
        let (_day, nr) = file!().split_at(7);
        let (nr, _ending) = nr.split_at(2);
        nr
    }

    #[test]
    fn test_a() {
        assert_eq!(puzzle_a(INPUT), RESULT_A)
    }

    #[test]
    fn actual_puzzle_a() {
        println!("Puzzle {} a: {}", get_day(), puzzle_a(&read_input_file(get_day())));
    }

    #[test]
    fn actual_puzzle_a_big_input() {
        let big_input = read_big_input_file(get_day());
        let start = Instant::now();
        let result = puzzle_a(&big_input);
        let end = Instant::now();
        let duration = end - start;
        println!("Puzzle {} a BIG INPUT: {} | {} ms", get_day(), result, duration.as_millis());
    }

    #[test]
    fn test_b() {
        assert_eq!(puzzle_b(INPUT), RESULT_B)
    }

    #[test]
    fn actual_puzzle_b() {
        println!("Puzzle {} b: {}", get_day(), puzzle_b(&read_input_file(get_day())));
    }

    #[test]
    fn actual_puzzle_b_big_input() {
        let big_input = read_big_input_file(get_day());
        let start = Instant::now();
        let result = puzzle_b(&big_input);
        let end = Instant::now();
        let duration = end - start;
        println!("Puzzle {} b BIG INPUT: {} | {} ms", get_day(), result, duration.as_millis());
    }
}
