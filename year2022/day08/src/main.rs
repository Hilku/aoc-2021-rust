fn main() {
    let input = include_str!("day8input.txt");
    let mut lines = input.lines();
    let mut trees: Vec<Vec<i32>> = Vec::new();
    let mut vec_index = 0;
    while let Some(line) = lines.next() {
        trees.push(Vec::new());
        for c in line.chars() {
            trees[vec_index].push(c.to_digit(10).unwrap().try_into().unwrap());
        }

        vec_index += 1;
    }

    let nr_of_tree_rows = trees.len();
    let nr_of_tree_columns = trees[0].len();
    let mut visible_trees = 0;

    for row in 0..nr_of_tree_rows {
        for column in 0..nr_of_tree_columns {
            if row == 0
                || column == 0
                || row == nr_of_tree_rows - 1
                || column == nr_of_tree_columns - 1
            {
                visible_trees += 1;
            } else {
                let tree_height = trees[row][column];

                //check up
                let mut was_visible: bool = false;
                for check_rows in 0..row {
                    if trees[check_rows][column] >= tree_height {
                        break;
                    } else if check_rows == row - 1 {
                        was_visible = true;
                    }
                }

                for check_rows in row + 1..nr_of_tree_rows {
                    if trees[check_rows][column] >= tree_height {
                        break;
                    } else if check_rows == nr_of_tree_rows - 1 {
                        was_visible = true;
                    }
                }

                for check_column in 0..column {
                    if trees[row][check_column] >= tree_height {
                        break;
                    } else if check_column == column - 1 {
                        was_visible = true;
                    }
                }

                for check_column in column + 1..nr_of_tree_columns {
                    if trees[row][check_column] >= tree_height {
                        break;
                    } else if check_column == nr_of_tree_columns - 1 {
                        was_visible = true;
                    }
                }

                if was_visible {
                    visible_trees += 1;
                }
            }
        }
    }

    println!("visible: {}", visible_trees);

    let mut highest_scenic_core = 0;
    for row in 0..nr_of_tree_rows {
        for column in 0..nr_of_tree_columns {
            let mut scenic_score = 1;
            if row == 0
                || column == 0
                || row == nr_of_tree_rows - 1
                || column == nr_of_tree_columns - 1
            {
                //visible_trees += 1; not consider edges for now
            } else {
                let tree_height = trees[row][column];

                let mut nr_of_smaller_trees = 0;
                for check_rows in (0..row).rev() {
                    nr_of_smaller_trees += 1;
                    if trees[check_rows][column] >= tree_height {
                        break;
                    }
                }
                scenic_score *= nr_of_smaller_trees;
                nr_of_smaller_trees = 0;

                for check_rows in row + 1..nr_of_tree_rows {
                    nr_of_smaller_trees += 1;
                    if trees[check_rows][column] >= tree_height {
                        break;
                    }
                }

                scenic_score *= nr_of_smaller_trees;
                nr_of_smaller_trees = 0;

                for check_column in (0..column).rev() {
                    nr_of_smaller_trees += 1;
                    if trees[row][check_column] >= tree_height {
                        break;
                    }
                }

                scenic_score *= nr_of_smaller_trees;
                nr_of_smaller_trees = 0;

                for check_column in column + 1..nr_of_tree_columns {
                    nr_of_smaller_trees += 1;
                    if trees[row][check_column] >= tree_height {
                        break;
                    }
                }
                scenic_score *= nr_of_smaller_trees;

                if scenic_score > highest_scenic_core {
                    highest_scenic_core = scenic_score;
                }
            }
        }
    }

    println!("part2:{}", highest_scenic_core);
}
