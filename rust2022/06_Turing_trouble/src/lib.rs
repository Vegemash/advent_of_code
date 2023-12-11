use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub fn process_part_1(input: &str) -> usize {
    find_marker(input, 4)
}
pub fn process_part_2(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, marker_size: usize) -> usize {
    let mut window: VecDeque<char> = VecDeque::new();
    // fill out the window
    for ch in input[0..marker_size - 1].chars() {
        window.push_back(ch)
    }
    for (i, ch) in input.chars().enumerate() {
        window.push_back(ch);
        println!("{:?}", window);
        if check_window(&window) {
            return i + 1;
        }
        window.pop_front();
    }
    return 0;
}

fn check_window<T>(window: &VecDeque<T>) -> bool
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    HashSet::<T>::from_iter(window.clone()).len() == window.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        assert_eq!(process_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(process_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn process_part_2_works() {
        assert_eq!(process_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(process_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
