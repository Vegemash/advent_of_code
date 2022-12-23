// use itertools::Itertools;
use std::collections::HashSet;

pub fn process_part_1(input: String) -> u32 {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| priority(match_on_line(line)))
        .sum()
}
pub fn process_part_2(input: String) -> u32 {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| priority(match_on_lines(lines.iter().map(|x| *x).collect())))
        .sum()
}

fn match_on_lines(lines: Vec<&str>) -> char {
    let mut common_chars: HashSet<char> = HashSet::from_iter(lines[0].chars());
    for line in lines.iter() {
        let other: HashSet<char> = HashSet::from_iter(line.chars());
        common_chars = HashSet::from_iter(common_chars.intersection(&other).cloned());
    }
    *common_chars.iter().last().unwrap()
}

fn match_on_line(line: &str) -> char {
    let first = line[0..line.len() / 2].to_string();
    let mut first_set: HashSet<char> = HashSet::new();
    for ch in first.chars() {
        first_set.insert(ch);
    }
    let second = line[line.len() / 2..].to_string();
    let mut second_set: HashSet<char> = HashSet::new();
    for ch in second.chars() {
        second_set.insert(ch);
    }
    println!("{}   {}", first, second);
    *first_set
        .intersection(&second_set)
        .into_iter()
        .last()
        .unwrap()
}

fn priority(ch: char) -> u32 {
    match ch as u32 {
        97..=122 => ch as u32 - 96,
        65..=90 => ch as u32 - 38,
        _ => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process_part_1(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                .to_string(),
        );
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                .to_string(),
        );
        assert_eq!(result, 70);
    }
}
