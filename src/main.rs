use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input/01").unwrap();

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Puzzle 01 a: {}", day01::puzzle_a(&input));
    println!("Puzzle 01 b: {}", day01::puzzle_b(&input));
}

mod day01;