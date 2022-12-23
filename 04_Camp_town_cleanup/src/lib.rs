pub fn process_part_1(input: &str) -> usize {
    input
        .split_terminator("\n")
        .filter(|line| {
            let assignments: Vec<usize> = line
                .split(&[',', '-'])
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let a = assignments[0];
            let b = assignments[1];
            let c = assignments[2];
            let d = assignments[3];
            (a >= c && b <= d) || (c >= a && d <= b)
        })
        .count()
}

pub fn process_part_2(input: &str) -> usize {
    input
        .split_terminator("\n")
        .filter(|line| {
            let assignments: Vec<usize> = line
                .split(&[',', '-'])
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let a = assignments[0];
            let b = assignments[1];
            let c = assignments[2];
            let d = assignments[3];

            let res = (a..=b).filter(|x| (c..=d).contains(x)).count() > 0
                || (c..=d).filter(|x| (a..=b).contains(x)).count() > 0;

            res
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
        );
        assert_eq!(result, 2);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
        );
        assert_eq!(result, 4);
    }
}
