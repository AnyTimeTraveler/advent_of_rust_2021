fn word_to_binary(word: &str) -> u8 {
    let mut out = 0;
    for b in word.bytes() {
        let index = b - b'a';
        out |= 1 << index;
    }
    out
}

fn process_line(line: &str) -> (u8, u8, u8, u8) {
    let (all_digits, value_digits) = line.split_once('|').unwrap();
    let words: Vec<u8> = all_digits.split(' ').filter(|i| !i.is_empty()).map(word_to_binary).collect();
    let one = *words.iter().find(|word| word.count_ones() == 2).unwrap();
    let four = *words.iter().find(|word| word.count_ones() == 4).unwrap();
    let seven = *words.iter().find(|word| word.count_ones() == 3).unwrap();

    let six_and_nine_and_zero: Vec<u8> = words.iter().filter(|word| word.count_ones() == 6).cloned().collect();
    assert_eq!(six_and_nine_and_zero.len(), 3);

    let zero = *six_and_nine_and_zero.iter().find(|digit| (**digit & four & !one).count_ones() == 1).unwrap();

    let six_and_nine: Vec<u8> = six_and_nine_and_zero.iter().filter(|digit| **digit != zero).cloned().collect();
    assert_eq!(six_and_nine.len(), 2);

    let six;
    let nine;
    if six_and_nine[0] & one == one {
        six = six_and_nine[1];
        nine = six_and_nine[0];
    } else {
        six = six_and_nine[0];
        nine = six_and_nine[1];
    }

    let two_and_three_and_five: Vec<u8> = words.iter().filter(|word| word.count_ones() == 5).cloned().collect();
    assert_eq!(two_and_three_and_five.len(), 3);

    let three = *two_and_three_and_five.iter().find(|digit| **digit & one == one).unwrap();

    let two_and_five: Vec<u8> = two_and_three_and_five.iter().filter(|digit| **digit != three).cloned().collect();
    assert_eq!(two_and_five.len(), 2);

    let one_top = one & !six;
    let two;
    let five;
    if two_and_five[0] & one_top == one_top {
        two = two_and_five[0];
        five = two_and_five[1];
    } else {
        two = two_and_five[1];
        five = two_and_five[0];
    }

    let eight = six | one_top;

    let mut map = [0; 128];
    map[zero as usize] = 0;
    map[one as usize] = 1;
    map[two as usize] = 2;
    map[three as usize] = 3;
    map[four as usize] = 4;
    map[five as usize] = 5;
    map[six as usize] = 6;
    map[seven as usize] = 7;
    map[eight as usize] = 8;
    map[nine as usize] = 9;

    let digits: Vec<u8> = value_digits.split(' ').filter(|i| !i.is_empty()).map(word_to_binary).collect();

    (
        map[digits[0] as usize],
        map[digits[1] as usize],
        map[digits[2] as usize],
        map[digits[3] as usize],
    )
}

pub(crate) fn puzzle_a(input: &str) -> i32 {
    let lines: Vec<(u8, u8, u8, u8)> = input.trim().split('\n').map(process_line).collect();
    let mut counter = 0;

    for t in lines {
        counter += count_certain_digits(t);
    }

    counter
}

fn count_certain_digits(t: (u8, u8, u8, u8)) -> i32 {
    let mut counter = 0;
    let (a, b, c, d) = t;
    for digit in [a, b, c, d] {
        if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
            counter += 1;
        }
    }
    counter
}

pub(crate) fn puzzle_b(input: &str) -> i64 {
    let lines: Vec<(u8, u8, u8, u8)> = input.trim().split('\n').map(process_line).collect();
    let mut counter = 0;

    for (a, b, c, d) in lines {
        let (a, b, c, d) = (a as i64, b as i64, c as i64, d as i64);
        counter += a * 1000;
        counter += b * 100;
        counter += c * 10;
        counter += d;
    }

    counter
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::{read_big_input_file, read_input_file};

    use super::*;

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    const INPUT_LONGER: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    const RESULT_LINES: [i32; 10] = [
        2,
        3,
        3,
        1,
        3,
        4,
        3,
        1,
        4,
        2
    ];
    const RESULT_LINES_B:[i64; 10] = [
        8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315,
    ];
    const RESULT_A: i32 = 2;
    const RESULT_B: i64 = 8394;
    const RESULT_LONGER_A: i32 = 26;
    const RESULT_LONGER_B: i64 = 61229;

    fn get_day() -> &'static str {
        let (_day, nr) = file!().split_at(7);
        let (nr, _ending) = nr.split_at(2);
        nr
    }

    #[test]
    fn test_word_to_binary() {
        assert_eq!(word_to_binary("a"), 1);
        assert_eq!(word_to_binary("b"), 2);
        assert_eq!(word_to_binary("c"), 4);
        assert_eq!(word_to_binary("d"), 8);
        assert_eq!(word_to_binary("e"), 16);
        assert_eq!(word_to_binary("f"), 32);
        assert_eq!(word_to_binary("g"), 64);
        assert_eq!(word_to_binary("abcdefg"), 127);
        assert_eq!(word_to_binary("gbcadef"), 127);
    }

    #[test]
    fn test_process_line() {
        assert_eq!(process_line(INPUT), (8, 3, 9, 4));
    }

    #[test]
    fn test_a() {
        assert_eq!(puzzle_a(INPUT), RESULT_A)
    }

    #[test]
    fn test_longer() {
        assert_eq!(puzzle_a(INPUT_LONGER), RESULT_LONGER_A)
    }

    #[test]
    fn test_longer_lines_a() {
        let lines: Vec<&str> = INPUT_LONGER.split("\n").collect();
        for (line, result) in lines.iter().zip(RESULT_LINES) {
            let t = process_line(line);
            println!("{} = {} {:?}", line, result, t);
            assert_eq!(count_certain_digits(t), result);
        }
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
    fn test_longer_lines_b() {
        assert_eq!(puzzle_b(INPUT_LONGER), RESULT_LONGER_B);
    }

    #[test]
    fn test_longer_lines_b_linewise() {
        let lines: Vec<&str> = INPUT_LONGER.split("\n").collect();
        for (line, result) in lines.iter().zip(RESULT_LINES_B) {
            let count = puzzle_b(line);
            println!("{} : {} == {}", line, result, count);
            assert_eq!(count, result);
        }
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
