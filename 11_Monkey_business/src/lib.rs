#[allow(dead_code, unused_variables)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Old,
    Num(u128),
    Add,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<u128>,
    operation: Vec<Token>,
    test: u128,
    targets: [u128; 2],
}

fn parse_int_list(input: &str) -> Vec<u128> {
    input
        .split(",")
        .into_iter()
        .map(|x| x.trim().parse::<u128>().unwrap())
        .collect()
}

fn parse_operation(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for x in input.split(" ").into_iter() {
        tokens.push(match x {
            "old" => Token::Old,
            "+" => Token::Add,
            "*" => Token::Mul,
            x => Token::Num(x.parse::<u128>().unwrap()),
        })
    }
    tokens
}

fn run_operation(old: u128, tokens: Vec<Token>) -> u128 {
    let mut remaining_tokens = tokens.to_vec();
    while remaining_tokens.len() >= 3 {
        let evaluated_value = apply_operation(
            old,
            [
                remaining_tokens[0],
                remaining_tokens[1],
                remaining_tokens[2],
            ],
        );
        if remaining_tokens.len() > 3 {
            let mut tmp = vec![Token::Num(evaluated_value)];
            tmp.append(&mut remaining_tokens[3..].to_vec());

            remaining_tokens = tmp;
        } else {
            return evaluated_value;
        }
    }
    panic!()
}

fn apply_operation(old: u128, tokens: [Token; 3]) -> u128 {
    let a = match tokens[0] {
        Token::Old => old,
        Token::Num(x) => x,
        _ => panic!(),
    };
    let b = match tokens[2] {
        Token::Old => old,
        Token::Num(x) => x,
        _ => panic!(),
    };

    match tokens[1] {
        Token::Add => a + b,
        Token::Mul => a * b,
        Token::Div => a / b,
        _ => panic!(),
    }
}

fn num_from_end(input: &str) -> u128 {
    input
        .split(" ")
        .into_iter()
        .last()
        .unwrap()
        .parse::<u128>()
        .unwrap()
}

fn parse_monkey(input: &str) -> Monkey {
    let lines = input.lines().collect::<Vec<&str>>();
    let test = num_from_end(lines[3]);
    Monkey {
        items: parse_int_list(lines[1].trim().trim_start_matches("Starting items: ")),
        operation: parse_operation(lines[2].trim().trim_start_matches("Operation: new = ")),
        test,
        targets: [num_from_end(lines[4]), num_from_end(lines[5])],
    }
}

pub fn process_part_1(input: &str) -> u128 {
    process_monkey_business(input, 20, vec![Token::Div, Token::Num(3)])
}
pub fn process_part_2(input: &str) -> u128 {
    process_monkey_business(input, 10_000, vec![])
}
fn process_monkey_business(input: &str, rounds: usize, mitigation: Vec<Token>) -> u128 {
    let mut monkies: Vec<Monkey> = input.split("\n\n").into_iter().map(parse_monkey).collect();
    let modulo: u128 = monkies.iter().map(|x| x.test).product();
    // counters for monkeys
    let mut inspection_counts: Vec<u128> = Vec::new();
    for _ in 0..monkies.len() {
        inspection_counts.push(0);
    }
    // loop through rounds
    for _round in 0..rounds {
        // loop through monkeys
        for monkey_index in 0..monkies.len() {
            // loop through items
            for item in monkies[monkey_index].items.clone().iter() {
                // Update counter
                inspection_counts[monkey_index] += 1;
                // apply operation
                let mut op = monkies[monkey_index].operation.clone();
                op.append(&mut mitigation.clone());
                let altered_item = run_operation(*item, op) % modulo;

                // run the test
                let to_index = match altered_item % monkies[monkey_index].test == 0 {
                    true => monkies[monkey_index].targets[0],
                    false => monkies[monkey_index].targets[1],
                };
                // send item to next monkey
                monkies[to_index as usize].items.push(altered_item);
            }
            monkies[monkey_index].items = vec![];
        }
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use std::fs::read_to_string;

    use super::*;

    #[rstest]
    #[case(3, [Token::Old,Token::Add,Token::Num(4)], 7)]
    #[case(3, [Token::Old,Token::Mul,Token::Num(4)], 12)]
    #[case(12, [Token::Old,Token::Div,Token::Num(4)], 3)]
    #[case(12, [Token::Old,Token::Div,Token::Num(5)], 2)]
    fn apply_operation_works(#[case] old: u128, #[case] tokens: [Token; 3], #[case] expected: u128) {
        assert_eq!(apply_operation(old, tokens), expected);
    }

    #[rstest]
    #[case(3, vec![Token::Old,Token::Add,Token::Num(4)], 7)]
    #[case(3, vec![Token::Old,Token::Add,Token::Num(4), Token::Mul, Token::Num(3)], 21)]
    fn run_operation_works(#[case] old: u128, #[case] tokens: Vec<Token>, #[case] expected: u128) {
        assert_eq!(run_operation(old, tokens), expected);
    }

    #[test]
    fn parse_operation_works() {
        assert_eq!(
            parse_operation("old + 4 * old"),
            vec![
                Token::Old,
                Token::Add,
                Token::Num(4),
                Token::Mul,
                Token::Old,
            ],
        );
    }
    #[test]
    fn parse_int_list_works() {
        assert_eq!(
            parse_int_list("98, 97, 98, 55, 56, 72"),
            vec![98, 97, 98, 55, 56, 72]
        );
    }
    #[test]
    fn parse_monkey_works() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        assert_eq!(
            parse_monkey(input),
            Monkey {
                items: vec![79, 98],
                operation: vec![Token::Old, Token::Mul, Token::Num(19),],
                test: 23,
                targets: [2, 3]
            }
        );
    }
    #[test]
    fn process_part_1_works() {
        let result = process_part_1(&read_to_string("data/test_input").unwrap());
        assert_eq!(result, 10605);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(&read_to_string("data/test_input").unwrap());
        assert_eq!(result, 2713310158);
    }
}
