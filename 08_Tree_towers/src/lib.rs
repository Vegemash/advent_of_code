use grid::Grid;

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}
pub fn process_part_1(input: &str) -> i32 {
    println!("cols: {}", input.lines().collect::<Vec<&str>>().len());
    let mut forrest = Grid::from_vec(
        input
            .chars()
            .filter_map(|char| match char.to_string().parse::<i8>() {
                Ok(n) => Some(Tree {
                    height: n,
                    visible: false,
                }),
                Err(_) => None,
            })
            .collect::<Vec<Tree>>(),
        input.lines().collect::<Vec<&str>>().len(),
    );

    for row_num in 0..forrest.rows() - 1 {
        let mut highest = -1;
        for tree in forrest.iter_row_mut(row_num) {
            if tree.height > highest{
                tree.visible = true;
                highest = tree.height;
            }
        }
        highest = -1;
        for tree in forrest.iter_row_mut(row_num).rev() {
            if tree.height > highest{
                tree.visible = true;
                highest = tree.height;
            }
        }
    }
    for col_num in 0..forrest.cols() {
        let mut highest = -1;
        for tree in forrest.iter_col_mut(col_num) {
            if tree.height > highest{
                tree.visible = true;
                highest = tree.height;
            }
        }
        highest = -1;
        for tree in forrest.iter_col_mut(col_num).rev() {
            if tree.height > highest{
                tree.visible = true;
                highest = tree.height;
            }
        }
    }

    forrest.flatten().iter().filter(|t| t.visible).count() as i32
}

pub fn process_part_2(input: &str) -> i8 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(
            "30373
        25512
        65332
        33549
        35390",
        );
        assert_eq!(result, 21);
    }
    #[test]
    #[ignore]
    fn process_part_2_works() {
        let result = process_part_2(
            "
",
        );
        assert_eq!(result, 0);
    }
}
