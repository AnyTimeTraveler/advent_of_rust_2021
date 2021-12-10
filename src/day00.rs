pub(crate) fn puzzle_a(input: &str) -> i32 {
    0
}

pub(crate) fn puzzle_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use crate::read_input_file;

    use super::*;

    const INPUT: &str = "";
    const RESULT_A: i32 = 0;
    const RESULT_B: i32 = 0;

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
    fn test_b() {
        assert_eq!(puzzle_b(INPUT), RESULT_B)
    }

    #[test]
    fn actual_puzzle_b() {
        println!("Puzzle {} b: {}", get_day(), puzzle_b(&read_input_file(get_day())));
    }
}
