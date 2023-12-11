use std::fs::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut tops: [u64; 3] = [0, 0, 0];
    let mut current: u64 = 0;
    let mut count: u64 = 0;
    let mut elf_count: u64 = 1;
    for line in read_lines("./data/input").unwrap() {
        count += 1;
        match line {
            Ok(l) => {
                if l.len() == 0 {
                    elf_count += 1;
                    tops = update_tops(tops, current);
                    current = 0;
                } else {
                    current += l.parse::<u64>().unwrap();
                }
            }
            Err(_) => break,
        }
    }
    tops = update_tops(tops, current);
    println!(
        "line count: {}, elf count: {}, max: {}",
        count,
        elf_count,
        tops.iter().sum::<u64>()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_tops(mut tops: [u64; 3], current: u64) -> [u64; 3] {
    if current < tops[0] && current < tops[1] && current < tops[2] {
        return tops;
    }
    let mut least = 0;
    if tops[1] < tops[least] {
        least = 1
    }
    if tops[2] < tops[least] {
        least = 2
    }
    tops[least] = current;
    tops
}
