use colored::Colorize;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Add, AddAssign};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Coords {
    x: i32,
    y: i32,
}

const DOWN: Coords = Coords { x: 0, y: 1 };
const LEFT: Coords = Coords { x: -1, y: 0 };
const RIGHT: Coords = Coords { x: 1, y: 0 };
const START_POS: Coords = Coords { x: 500, y: 0 };

impl Add for Coords {
    type Output = Coords;
    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

pub fn process_part_1(input: &str) -> usize {
    let (mut map, mut sand) = parse_grid(input);

    let max_x = map.iter().map(|c| c.x).max().unwrap();
    let min_x = map.iter().map(|c| c.x).min().unwrap();
    let max_y = map.iter().map(|c| c.y).max().unwrap();
    let bounds = (Coords { x: min_x, y: 0 }, Coords { x: max_x, y: max_y });

    let mut sand_count = 0;
    loop {
        sand_count += 1;
        if sand_count % 25 == 0 {
            draw_map(&mut map, &mut sand);
            // sleep(Duration::new(0, 400));
        }
        if step_unlimited(&mut map, &mut sand, bounds) {
            break;
        }
    }
    sand_count - 1
}

fn exceeds_bounds(pos: Coords, bounds: (Coords, Coords)) -> bool {
    if pos.x < bounds.0.x || pos.x > bounds.1.x || pos.y < bounds.0.y || pos.y > bounds.1.y {
        return true;
    }
    false
}

fn step_unlimited(
    map: &mut HashSet<Coords>,
    sand: &mut HashSet<Coords>,
    bounds: (Coords, Coords),
) -> bool {
    let mut sand_pos = START_POS.clone();
    let mut done = false;
    'filled: loop {
        let mut moved = false;
        'moove: for &pos in [
            sand_pos + DOWN,
            sand_pos + DOWN + LEFT,
            sand_pos + DOWN + RIGHT,
        ]
        .iter()
        {
            if exceeds_bounds(sand_pos, bounds) {
                done = true;
                break 'filled;
            }
            if !map.contains(&(pos)) {
                moved = true;
                sand_pos = pos;
                break 'moove;
            }
        }
        if !moved {
            map.insert(sand_pos);
            sand.insert(sand_pos);
            break 'filled;
        }
    }
    done
}

fn get_floor(map: &HashSet<Coords>, sand: &HashSet<Coords>) -> i32 {
    map.iter()
        // exclude sand from map calculation
        .filter(|x| !sand.contains(x))
        .map(|c| c.y)
        .max()
        .unwrap()
        + 2
}

pub fn step_limited(map: &mut HashSet<Coords>, sand: &mut HashSet<Coords>) -> bool {
    let mut grain_pos = START_POS.clone();
    let floor = get_floor(map, sand);

    'filled: loop {
        let mut moved = false;
        'moove: for &pos in [
            grain_pos + DOWN,
            grain_pos + DOWN + LEFT,
            grain_pos + DOWN + RIGHT,
        ]
        .iter()
        {
            if !map.contains(&(pos)) && grain_pos.y + 1 < floor {
                moved = true;
                grain_pos = pos;
                break 'moove;
            }
        }
        if !moved {
            map.insert(grain_pos);
            sand.insert(grain_pos);
            if grain_pos == START_POS {
                return true;
            }
            break 'filled;
        }
    }
    false
}

fn draw_map(map: &mut HashSet<Coords>, sand: &mut HashSet<Coords>) {
    print!("{}", get_repr(map, sand));
}

pub fn get_grid_repr(map: &HashSet<Coords>, sand: &HashSet<Coords>) -> Vec<Vec<char>> {
    let max_x = map.iter().map(|c| c.x).max().unwrap();
    let min_x = map.iter().map(|c| c.x).min().unwrap();
    let max_y = map.iter().filter(|x| !sand.contains(&x)).map(|c| c.y).max().unwrap() + 2;
    let mut repr = vec![];
    for y in 0..=max_y {
        repr.push(vec![]);
        for x in min_x..=max_x {
            let ch;
            if sand.contains(&Coords { x, y }) {
                ch = 's';
            } else if map.contains(&Coords { x, y }) || y == max_y {
                ch = '#';
            } else {
                ch = '.';
            }
            repr[y as usize].push(ch);
        }
    }
    repr
}

pub fn get_repr(map: &HashSet<Coords>, sand: &HashSet<Coords>) -> String {
    let mut repr = format!("{}[2J=====\n", 27 as char);
    repr = format!("{repr}=====\n");
    let grid_repr = get_grid_repr(map, sand);
    for y in 0..grid_repr.len() {
        for x in 0..grid_repr[y].len() {
            repr = format!("{repr}{}", grid_repr[y][x]);
        }
        repr = format!("{repr}\n");
    }
    repr
}

pub fn get_grid() -> (HashSet<Coords>, HashSet<Coords>) {
    parse_grid(include_str!("../data/input"))
}

fn parse_grid(input: &str) -> (HashSet<Coords>, HashSet<Coords>) {
    let map = produce_hash_set(
        &&input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Vec<Vec<Coords>>>(),
    );
    let sand = HashSet::new();
    (map, sand)
}

pub fn process_part_2(input: &str) -> usize {
    let (mut map, mut sand) = parse_grid(input);

    let mut sand_count = 0;
    loop {
        sand_count += 1;
        if step_limited(&mut map, &mut sand) {
            break;
        }
        // if sand_count % 2500 == 0 {
        //     draw_map(&mut map, &mut sand);
        //     sleep(Duration::new(0, 400));
        // }
    }
    sand_count
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

fn produce_hash_set(input: &Vec<Vec<Coords>>) -> HashSet<Coords> {
    let mut hash_set = HashSet::new();
    for line in input.iter() {
        for (point_a, point_b) in line.iter().tuple_windows() {
            // Horizontal line
            if point_a.x != point_b.x {
                for i in min(point_a.x, point_b.x)..=max(point_a.x, point_b.x) {
                    hash_set.insert(Coords { x: i, y: point_a.y });
                }
            }
            // Vertical line
            if point_a.y != point_b.y {
                for i in min(point_a.y, point_b.y)..=max(point_a.y, point_b.y) {
                    hash_set.insert(Coords { x: point_a.x, y: i });
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
    fn process_part_1_works() {
        let result = process_part_1(include_str!("../data/test_input"));
        assert_eq!(result, 24);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(include_str!("../data/test_input"));
        assert_eq!(result, 93);
    }
}
