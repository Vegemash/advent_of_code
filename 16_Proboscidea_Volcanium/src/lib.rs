use aoc_tools::trim_prefix;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Valve<'a> {
    id: &'a str,
    flow_rate: u32,
    tunnels_to: Vec<&'a str>,
}
impl<'a> From<&'a str> for Valve<'a> {
    fn from(value: &'a str) -> Self {
        let (id, flow_rate_str, tunnels_to);

        let mut remainder = trim_prefix("Valve ", value);

        (id, remainder) = remainder.split_once(" ").unwrap();

        remainder = trim_prefix("has flow rate=", remainder);
        (flow_rate_str, remainder) = remainder.split_once(";").unwrap();

        remainder = trim_prefix(" tunnels lead to valves ", remainder);
        remainder = trim_prefix(" tunnel leads to valve ", remainder);
        tunnels_to = remainder.split(", ").collect();

        Valve {
            id,
            flow_rate: flow_rate_str.parse().unwrap(),
            tunnels_to,
        }
    }
}

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
    fn test_valve_parse() {
        let result: Valve = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".into();
        assert_eq!(
            result,
            Valve {
                id: "AA",
                flow_rate: 0,
                tunnels_to: vec!["DD", "II", "BB"]
            }
        );
    }

    #[test]
    fn test_multiple_valve_parse() {
        let result: Vec<Valve> = include_str!("../data/test_input")
            .split_terminator("\n")
            .map(|v| v.into())
            .collect();
        for valve in result.iter() {
            for tunnel in valve.tunnels_to.iter() {
                assert!(!tunnel.contains(" "))
            }
        }
    }
    #[test]
    #[ignore]
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
