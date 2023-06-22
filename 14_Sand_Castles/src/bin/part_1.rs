use std::fs::read_to_string;

use aoc_14::process_part_1;

fn main() {
    println!("{}", process_part_1(&read_to_string("data/input").unwrap()));
}
