use std::fs::read_to_string;

use aoc_04::process_part_1;

fn main() {
    let a: usize = 2;
    let b: usize = 6;
    let c: usize = 4;
    let d: usize = 8;
    let res = (a..=b).filter(|x| (c..=d).contains(x)).count() > 0
        || (c..=d).filter(|x| (a..=b).contains(x)).count() > 0;

    println!("{}", res);
    println!("{}", process_part_1(&read_to_string("data/input").unwrap()));
}
