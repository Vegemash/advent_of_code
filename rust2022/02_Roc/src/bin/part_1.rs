use aoc_02::process_part1;
use std::fs::read_to_string;

fn main() {
    println!("{}", process_part1(read_to_string("data/input").unwrap()))
}
