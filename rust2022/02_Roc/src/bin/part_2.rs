use aoc_02::process_part2;
use std::fs::read_to_string;

fn main() {
    println!("{}", process_part2(read_to_string("data/input").unwrap()))
}
