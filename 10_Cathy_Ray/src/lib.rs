use std::fs::read_to_string;

#[derive(Debug)]
enum Action {
    Noop,
    Addx(i32),
}

pub fn process_part_1(input: &str) -> i32 {
    let mut x = 1;
    let mut counter = 0;
    let mut sumation = 0;
    let mut instructions: Vec<Action> = Vec::new();
    for line in input.to_string().split_terminator("\n") {
        instructions.append(&mut parse_line(line));
    }

    for instruction in instructions {
        counter += 1;
        if counter % 40 == 20 {
            sumation += counter * x;
        }
        match instruction {
            Action::Addx(ax) => x += ax,
            Action::Noop => (),
        }
    }
    sumation
}

fn parse_line(input: &str) -> Vec<Action> {
    if input == "noop" {
        return vec![Action::Noop];
    }
    let (_, x) = input.split_once(" ").unwrap();
    vec![Action::Noop, Action::Addx(x.parse::<i32>().unwrap())]
}

pub fn process_part_2(input: &str) -> String {
    let mut x: i32 = 1;
    let mut counter: i32 = 0;
    let mut instructions: Vec<Action> = Vec::new();
    let mut screen = "".to_string();
    for line in input.to_string().split_terminator("\n") {
        instructions.append(&mut parse_line(line));
    }

    for instruction in instructions {
        let lit = (x - (counter % 40)).abs() < 2;
        if  lit {
            screen.push_str("#");
        } else {
            screen.push_str(" ");
        }
        match instruction {
            Action::Addx(ax) => x += ax,
            Action::Noop => (),
        }

        if counter % 40 == 39 {
            screen.push_str("\n");
        }
        counter += 1;
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn process_part_1_works() {
        let result = process_part_1(&read_to_string("data/test_input").unwrap());
        assert_eq!(result, 13140);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(&read_to_string("data/test_input").unwrap());
        assert_eq!(
            result,
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
"
        );
    }
}
