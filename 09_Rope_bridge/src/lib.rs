use std::collections::HashSet;
use std::ops::Sub;

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for Pos {
    fn eq(self: &Self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

pub fn process_part_1(input: &str) -> usize {
    let mut positions: HashSet<Pos> = HashSet::new();

    let moves = input
        .split_terminator("\n")
        .map(parse_line)
        .collect::<Vec<(Pos, i32)>>();
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    positions.insert(tail);
    for (Pos { x, y }, count) in moves.iter() {
        for _ in 0..*count as usize {
            head.x += x;
            head.y += y;
            if (head.x - tail.x).abs() > 1 {
                tail.x += x;
                tail.x = head.x;
            }
            if (head.y - tail.y).abs() > 1 {
                tail.y += y;
                tail.x = head.x;
            }
            positions.insert(tail);
        }
    }

    positions.iter().count()
}

fn parse_line(line: &str) -> (Pos, i32) {
    let (dir_str, count_str) = line.split_once(" ").unwrap();
    let count = count_str.parse::<i32>().unwrap();
    match dir_str {
        "D" => (Pos { x: 0, y: 1 }, count),
        "U" => (Pos { x: 0, y: -1 }, count),
        "L" => (Pos { x: -1, y: 0 }, count),
        "R" => (Pos { x: 1, y: 0 }, count),
        _ => panic!(),
    }
}

pub fn process_part_2(input: &str) -> usize {
    let mut positions: HashSet<Pos> = HashSet::new();

    let moves = input
        .split_terminator("\n")
        .map(parse_line)
        .collect::<Vec<(Pos, i32)>>();
    let mut rope = [Pos { x: 0, y: 0 }; 10];
    positions.insert(rope[9]);
    let mut all_positions: HashSet<Pos> = HashSet::new();
    for (Pos { x, y }, count) in moves.iter() {
        for _ in 0..*count as usize {
            rope[0].x += x;
            rope[0].y += y;
            for i in 0..9 {
                let diff = rope[i] - rope[i + 1];

                if (diff.x).abs() > 1 {
                    rope[i + 1].x = rope[i].x - diff.x.signum();
                    rope[i + 1].y = rope[i].y;
                }
                if (diff.y).abs() > 1 {
                    rope[i + 1].y = rope[i].y - diff.y.signum();
                    rope[i + 1].x = rope[i].x;
                }
                all_positions.insert(rope[i]);
                all_positions.insert(rope[i + 1]);
            }
            positions.insert(rope[9]);
        }
        println!("({x}, {y}) x {count}");
    }

    print_board(&all_positions, &positions, &rope);
    positions.iter().count()
}

fn print_board(all_positions: &HashSet<Pos>, positions: &HashSet<Pos>, rope: &[Pos; 10]) -> () {
    for j in all_positions
        .iter()
        .chain(rope.iter())
        .map(|p| p.y)
        .min()
        .unwrap()
        ..all_positions
            .iter()
            .chain(rope.iter())
            .map(|p| p.y)
            .max()
            .unwrap()
            + 1
    {
        for k in all_positions
            .iter()
            .chain(rope.iter())
            .map(|p| p.x)
            .min()
            .unwrap()
            ..all_positions
                .iter()
                .chain(rope.iter())
                .map(|p| p.x)
                .max()
                .unwrap()
                + 1
        {
            if rope.contains(&Pos { x: k, y: j }) {
                for (z, knot) in rope.iter().enumerate() {
                    if knot == &(Pos { x: k, y: j }) {
                        print!("{z}");
                        break;
                    }
                }
            } else if positions.contains(&Pos { x: k, y: j }) {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(result, 13);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(result, 36);
    }
}
