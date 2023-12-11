pub fn process_part_1(input: &str) -> String {
    input.to_string()
}

pub fn process_part_2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(include_str!("../data/test_input"));
        assert_eq!(result, "CMZ");
    }
    #[test]
    #[ignore]
    fn process_part_2_works() {
        let result = process_part_2(include_str!("../data/test_input"));
        assert_eq!(result, "CMZ");
    }
}
