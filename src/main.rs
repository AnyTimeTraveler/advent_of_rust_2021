use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input/02").unwrap();

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Puzzle: {}", day02::puzzle_b(&input));
}

mod day01;
mod day02;