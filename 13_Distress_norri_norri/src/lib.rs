use std::cmp::Ordering;
use std::fmt::Display;
use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
enum PacketPart {
    List(Vec<PacketPart>),
    Integer(u32),
}

impl Display for PacketPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketPart::Integer(int) => write!(f, "{}", int),
            PacketPart::List(list) => {
                write!(
                    f,
                    "[{}]",
                    list.iter()
                        .enumerate()
                        .map(|(i, s)| format!("{}{}", if i != 0 { "," } else { "" }, s))
                        .collect::<String>()
                )
            }
        }
    }
}

impl Ord for PacketPart {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketPart::Integer(a_int), PacketPart::Integer(b_int)) => a_int.cmp(&b_int),
            (PacketPart::List(a_list), PacketPart::List(b_list)) => {
                for (a_part, b_part) in zip(a_list, b_list).into_iter() {
                    let ord = a_part.cmp(&b_part);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                return a_list.len().cmp(&b_list.len());
            }
            (PacketPart::List(a_list), PacketPart::Integer(b_int)) => {
                a_list.cmp(&vec![PacketPart::Integer(b_int.to_owned())])
            }
            (PacketPart::Integer(a_int), PacketPart::List(b_list)) => {
                vec![PacketPart::Integer(a_int.to_owned())].cmp(&b_list)
            }
        }
    }
}

impl PartialOrd for PacketPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
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
                if char_buff.len() > 0 {
                    items.push(PacketPart::Integer(char_buff.parse::<u32>().unwrap()));
                }
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
    input
        .lines()
        .filter(|line| line != &"")
        .map(|line| parse_packet(line))
        .collect::<Vec<PacketPart>>()
        .chunks(2)
        .enumerate()
        .into_iter()
        .map(|(i, packets)| {
            if packets[0].cmp(&packets[1]) == Ordering::Less {
                println!("adding index {}", i + 1);
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>() as u32
}

pub fn process_part_2(input: &str) -> String {
    let mut packets = input
        .lines()
        .filter(|line| line != &"")
        .map(|line| parse_packet(line))
        .collect::<Vec<PacketPart>>();
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));
    packets.sort();
    for packet in packets.iter() {
        println!("{}", packet);
    }
    let mut two = 0;
    let mut six = 0;
    for (i, packet) in packets.iter().enumerate() {
        let repr = format!("{}", packet);
        if repr == "[[2]]" {
            two = i + 1
        }
        if repr == "[[6]]" {
            six = i + 1
        }
    }
    (two * six).to_string()
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
        assert_eq!(a.cmp(&b), ord)
    }

    #[rstest]
    #[case("[9]", "[[8,7,6]]", Ordering::Greater)]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less)]
    #[case("[[[]]]", "[[]]", Ordering::Greater)]
    #[case(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Ordering::Greater
    )]

    fn test_parse_and_compare_packet(
        #[case] a_packet: &str,
        #[case] b_packet: &str,
        #[case] ord: Ordering,
    ) {
        assert_eq!(parse_packet(a_packet).cmp(&parse_packet(b_packet)), ord)
    }
    #[test]
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
