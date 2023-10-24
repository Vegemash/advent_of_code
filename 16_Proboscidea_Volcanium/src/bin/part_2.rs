use std::fs::read_to_string;

use aoc_16::process_part_2;

fn main() {
    println!("{}", process_part_2(&read_to_string("data/input").unwrap()));
}
