use std::collections::VecDeque;

pub fn process_part_1(input: &str) -> String {
    let (setup, actions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(setup);

    for (count, from, dest) in parse_moves(actions) {
        for _ in 0..count {
            match stacks[from].pop_front() {
                Some(krate) => stacks[dest].push_front(krate),
                None => (),
            }
        }
    }

    let res = stacks
        .iter()
        .filter_map(|stack| stack.front())
        .collect::<String>();
    res
}

pub fn process_part_2(input: &str) -> String {
    let (setup, actions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(setup);

    for (count, from, dest) in parse_moves(actions) {
        let mut hand = VecDeque::new();
        for _ in 0..count {
            match stacks[from].pop_front() {
                Some(krate) => hand.push_front(krate),
                None => (),
            }
        }
        for _ in 0..count {
            match hand.pop_front() {
                Some(krate) => stacks[dest].push_front(krate),
                None => (),
            }
        }
    }

    let res = stacks
        .iter()
        .filter_map(|stack| stack.front())
        .collect::<String>();
    res
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let nums = line
                .split(" ")
                .filter_map(|n| match n.parse::<usize>() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                })
                .take(3)
                .collect::<Vec<usize>>();
            (nums[0], nums[1] - 1, nums[2] - 1)
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let buckets = (lines[0].len() + 1) / 4;
    for _ in 0..buckets {
        stacks.push(VecDeque::new());
    }
    for line in lines[0..lines.len() - 1].into_iter() {
        for (i, c) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .collect::<Vec<(usize, char)>>()
        {
            stacks[i / 4].push_back(c);
        }
    }

    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_setup() {
        let setup = parse_stacks(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        );
        assert_eq!(setup, vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']])
    }

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
        );
        assert_eq!(result, "CMZ");
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
        );
        assert_eq!(result, "MCD");
    }
}
