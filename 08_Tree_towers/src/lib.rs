use grid::Grid;

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}
pub fn process_part_1(input: &str) -> i32 {
    println!("cols: {}", input.lines().collect::<Vec<&str>>().len());
    let mut forrest = get_grid(input);

    for row_num in 0..forrest.rows() - 1 {
        let mut highest = -1;
        for tree in forrest.iter_row_mut(row_num) {
            if tree.height > highest {
                tree.visible = true;
                highest = tree.height;
            }
        }
        highest = -1;
        for tree in forrest.iter_row_mut(row_num).rev() {
            if tree.height > highest {
                tree.visible = true;
                highest = tree.height;
            }
        }
    }
    for col_num in 0..forrest.cols() {
        let mut highest = -1;
        for tree in forrest.iter_col_mut(col_num) {
            if tree.height > highest {
                tree.visible = true;
                highest = tree.height;
            }
        }
        highest = -1;
        for tree in forrest.iter_col_mut(col_num).rev() {
            if tree.height > highest {
                tree.visible = true;
                highest = tree.height;
            }
        }
    }

    forrest.flatten().iter().filter(|t| t.visible).count() as i32
}

pub fn process_part_2(input: &str) -> usize {
    let forrest = get_grid(input);
    let mut best = 0;
    for i in 1..forrest.cols() - 1 {
        for j in 1..forrest.rows() - 1 {
            let current_tree = forrest.get(i, j).unwrap().height;

            let mut dir_scores: Vec<i32> = Vec::new();
            for dir in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                let mut offset = 1;
                // count down
                while let Some(tree) = forrest.get(
                    (i as i32 + (offset * dir.0)) as usize,
                    (j as i32 + (offset * dir.1)) as usize,
                ) {
                    offset += 1;
                    if tree.height >= current_tree {
                        break;
                    }
                }
                dir_scores.push(offset - 1);
            }

            let score = dir_scores.iter().product();
            if score > best {
                best = score
            }
        }
    }
    best.try_into().unwrap()
}

fn get_grid(input: &str) -> Grid<Tree> {
    Grid::from_vec(
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
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
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
    fn process_part_2_works() {
        let result = process_part_2(
            "30373
        25512
        65332
        33549
        35390",
        );
        assert_eq!(result, 8);
    }
}
