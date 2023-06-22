use colored::Colorize;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

pub fn process_part_1(input: &str) -> usize {
    let mut map = produce_hash_set(
        &&input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Vec<Vec<Coords>>>(),
    );
    let mut sand = HashSet::new();

    let max_x = map.iter().map(|c| c.x).max().unwrap();
    let min_x = map.iter().map(|c| c.x).min().unwrap();
    let max_y = map.iter().map(|c| c.y).max().unwrap();

    let mut sand_count = 0;
    let mut done = false;
    loop {
        sand_count += 1;
        let mut sand_pos = Coords { x: 500, y: 0 };
        loop {
            if !map.contains(&Coords {
                x: sand_pos.x,
                y: sand_pos.y + 1,
            }) {
                sand_pos = Coords {
                    x: sand_pos.x,
                    y: sand_pos.y + 1,
                };
            } else if !map.contains(&Coords {
                x: sand_pos.x - 1,
                y: sand_pos.y + 1,
            }) {
                sand_pos = Coords {
                    x: sand_pos.x - 1,
                    y: sand_pos.y + 1,
                };
            } else if !map.contains(&Coords {
                x: sand_pos.x + 1,
                y: sand_pos.y + 1,
            }) {
                sand_pos = Coords {
                    x: sand_pos.x + 1,
                    y: sand_pos.y + 1,
                };
            } else {
                map.insert(sand_pos);
                sand.insert(sand_pos);
                break;
            }
            if sand_pos.x > max_x || sand_pos.y > max_y || sand_pos.x < min_x {
                done = true;
                break;
            }
        }
        if sand_count % 100000 == 0 {
            print!("{}[2J", 27 as char);
            println!("=====");
            for y in 0..=max_y {
                for x in min_x..=max_x {
                    if sand.contains(&Coords { x, y }) {
                        print!("{}", "s".yellow());
                    } else if map.contains(&Coords { x, y }) {
                        print!("{}", "#".purple());
                    } else {
                        print!("{}", ".".blue());
                    }
                }
                print!("\n");
            }
            println!("=====");
        }
        if done {
            break;
        }
    }
    sand_count - 1
}

pub fn process_part_2(input: &str) -> String {
    input.to_string()
}

fn parse_line(input: &str) -> Vec<Coords> {
    input
        .split(" -> ")
        .map(|s| {
            let (x, y) = s.split_once(",").unwrap();
            Coords {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<Coords>>()
}

fn reduce_scan_map(input: &Vec<Vec<Coords>>) -> Vec<Vec<Coords>> {
    let min_x = input.iter().flatten().map(|c| c.x).min().unwrap();
    let min_y = input.iter().flatten().map(|c| c.y).min().unwrap();
    let mut reduced = input.clone();
    for line in reduced.iter_mut() {
        for coord in line.iter_mut() {
            coord.x -= min_x;
            coord.y -= min_y;
        }
    }
    reduced
}

fn produce_hash_set(input: &Vec<Vec<Coords>>) -> HashSet<Coords> {
    let mut hash_set = HashSet::new();
    for line in input.iter() {
        for (a, b) in line.iter().tuple_windows() {
            if a.x != b.x {
                for i in min(a.x, b.x)..=max(a.x, b.x) {
                    hash_set.insert(Coords { x: i, y: a.y });
                }
            }
            if a.y != b.y {
                for i in min(a.y, b.y)..=max(a.y, b.y) {
                    hash_set.insert(Coords { x: a.x, y: i });
                }
            }
        }
    }
    hash_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("498,4 -> 498,6 -> 496,6"),
            vec![
                Coords { x: 498, y: 4 },
                Coords { x: 498, y: 6 },
                Coords { x: 496, y: 6 },
            ]
        )
    }

    #[test]
    fn test_reduce_hashmap_coords() {
        assert_eq!(
            reduce_scan_map(&vec![
                vec![
                    Coords { x: 498, y: 4 },
                    Coords { x: 498, y: 6 },
                    Coords { x: 496, y: 6 },
                ],
                vec![
                    Coords { x: 498, y: 4 },
                    Coords { x: 498, y: 6 },
                    Coords { x: 496, y: 60 },
                ]
            ]),
            vec![
                vec![
                    Coords { x: 2, y: 0 },
                    Coords { x: 2, y: 2 },
                    Coords { x: 0, y: 2 },
                ],
                vec![
                    Coords { x: 2, y: 0 },
                    Coords { x: 2, y: 2 },
                    Coords { x: 0, y: 56 },
                ]
            ]
        )
    }
    #[test]
    fn process_part_1_works() {
        let result = process_part_1(include_str!("../data/test_input"));
        assert_eq!(result, 24);
    }
    #[test]
    #[ignore]
    fn process_part_2_works() {
        let result = process_part_2(
            "
",
        );
        assert_eq!(result, "MCD");
    }
}
