#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

pub(crate) fn read_file(day :&str) -> String {
    let mut file = File::open(format!("input/{}", day)).unwrap();

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

mod day00;
mod day01;
mod day02;
