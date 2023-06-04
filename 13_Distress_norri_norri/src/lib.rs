use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum PacketPart {
    List(Vec<PacketPart>),
    Integer(u32),
}

fn compare_packets(a: PacketPart, b: PacketPart) -> Ordering {
    match (a, b) {
        (PacketPart::Integer(a_int), PacketPart::Integer(b_int)) => a_int.cmp(&b_int),
        (PacketPart::List(a_list), PacketPart::List(b_list)) =>,
        _ => todo!(),
        
    }
}

fn parse_packet(input: &str) -> PacketPart {
    PacketPart::List(parse_packet_parts(&input[1..input.len() - 1]))
}

fn parse_packet_parts(input: &str) -> Vec<PacketPart> {
    if input == "" {
        return vec![];
    }

    let mut items: Vec<PacketPart> = vec![];
    println!("parsing packet {}", input);

    let mut index = 0;
    let mut char_buff: String = "".to_string();
    loop {
        match input.chars().nth(index) {
            Some('[') => {
                let matching_brace = index + get_matching_index(&input[index..]);
                items.push(PacketPart::List(parse_packet_parts(
                    &input[index + 1..matching_brace],
                )));
                index = matching_brace;
            }
            Some(',') | Some(']') => {
                items.push(PacketPart::Integer(char_buff.parse::<u32>().unwrap()));
                char_buff = "".to_string();
            }
            Some(ch) => char_buff += &ch.to_string(),
            None => panic!(),
        };
        index += 1;
        if index == input.len() {
            if char_buff.len() > 0 {
                items.push(PacketPart::Integer(char_buff.parse::<u32>().unwrap()));
            }
            break;
        }
    }
    return items;
}

fn get_matching_index(input: &str) -> usize {
    let mut depth: u32 = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            '[' => depth += 1,
            _ => (),
        }
    }
    dbg!(input);
    panic!("Could not find matching brace")
}

pub fn process_part_1(input: &str) -> u32 {
    !todo!()
}

pub fn process_part_2(input: &str) -> String {
    !todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(PacketPart::List(vec![]), "[]")]
    #[case(PacketPart::List(vec![PacketPart::List(vec![])]), "[[]]")]
    #[case(
        PacketPart::List(
            vec![
                PacketPart::Integer(3),
                PacketPart::Integer(4)
            ]
        ),
        "[3,4]"
    )]
    #[case(
        PacketPart::List(
            vec![
                PacketPart::Integer(3),
                PacketPart::Integer(4),
                PacketPart::List(vec![]),
            ]
        ),
        "[3,4,[]]"
    )]
    fn test_parse_packet(#[case] packet: PacketPart, #[case] input: &str) {
        println!("{}", input);
        assert_eq!(parse_packet(input), packet);
    }

    #[rstest]
    #[case(PacketPart::Integer(2), PacketPart::Integer(6), Ordering::Less)]
    fn test_compare_packets(#[case] a: PacketPart, #[case] b: PacketPart, #[case] ord: Ordering) {
        assert_eq!(compare_packets(a, b), ord)
    }
    #[test]
    #[ignore]
    fn process_part_1_works() {
        let result = process_part_1(include_str!("../data/test_input"));
        assert_eq!(result, 13);
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
