use crate::common::read_lines;
use std::path::Path;

pub fn day8<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let trees = get_grid_from_lines(read_lines(file_path));
    (part_one(&trees), part_two(&trees))
}

fn part_one(trees: &Vec<Vec<i32>>) -> i32 {
    let mut visibility_record = vec![vec![false; trees[0].len()]; trees.len()];
    let mut total_visible: i32 = 0;

    // Search left -> right and top -> bottom and include sides
    let mut row_max: Vec<i32> = vec![-1; trees[0].len()];
    let mut col_max: Vec<i32> = vec![-1; trees.len()];
    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            if i == 0 || j == 0 || i == row.len() - 1 || j == trees.len() - 1 {
                visibility_record[i][j] = true;
                total_visible += 1;
            }

            let mut seen = false;
            if tree > &row_max[i] {
                seen = true;
                row_max[i] = *tree;
            }
            if tree > &col_max[j] {
                seen = true;
                col_max[j] = *tree;
            }

            if seen {
                if visibility_record[i][j] == false {
                    visibility_record[i][j] = true;
                    total_visible += 1;
                }
            }
        }
    }

    // Search right -> left and bottom -> top without sides this time
    let mut row_max: Vec<i32> = vec![-1; trees[0].len()];
    let mut col_max: Vec<i32> = vec![-1; trees.len()];
    for (i, row) in trees.iter().enumerate().rev() {
        for (j, tree) in row.iter().enumerate().rev() {
            let mut seen = false;
            if tree > &row_max[i] {
                seen = true;
                row_max[i] = *tree;
            }
            if tree > &col_max[j] {
                seen = true;
                col_max[j] = *tree;
            }

            if seen {
                if visibility_record[i][j] == false {
                    visibility_record[i][j] = true;
                    total_visible += 1;
                }
            }
        }
    }

    total_visible
}

fn part_two(trees: &Vec<Vec<i32>>) -> i32 {
    let mut top_scenic_score = 0;

    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let left = score_left(trees, i, j, tree);
            let up = score_up(trees, i, j, tree);
            let right = score_right(trees, i, j, tree);
            let down = score_down(trees, i, j, tree);
            let score = left * up * right * down;

            // println!(
            //     "tree {} left {} up {} right {} down {} total {}",
            //     tree, left, up, right, down, score
            // );

            if score > top_scenic_score {
                top_scenic_score = score;
            }
        }
    }

    top_scenic_score
}

fn score_left(trees: &Vec<Vec<i32>>, i: usize, j: usize, height: &i32) -> i32 {
    let mut score = 0;

    for check in trees[i][0..j].iter().rev() {
        if check < height {
            score += 1;
        } else {
            score += 1;
            break;
        }
    }

    score
}
fn score_up(trees: &Vec<Vec<i32>>, i: usize, j: usize, height: &i32) -> i32 {
    let mut score = 0;

    for row in trees[0..i].iter().rev() {
        if row[j] < *height {
            score += 1
        } else {
            score += 1;
            break;
        }
    }

    score
}
fn score_right(trees: &Vec<Vec<i32>>, i: usize, j: usize, height: &i32) -> i32 {
    let mut score = 0;

    // TODO: look into why slice does not work here rustc --explain E0277
    for (idx, check) in trees[i].iter().enumerate() {
        if idx > j {
            if check < height {
                score += 1;
            } else {
                score += 1;
                break;
            }
        }
    }

    score
}
fn score_down(trees: &Vec<Vec<i32>>, i: usize, j: usize, height: &i32) -> i32 {
    let mut score = 0;

    for (idx, row) in trees.iter().enumerate() {
        if idx > i {
            if row[j] < *height {
                score += 1;
            } else {
                score += 1;
                break;
            }
        }
    }

    score
}

fn get_grid_from_lines(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.split("")
                .filter(|char| char.len() > 0)
                .map(|char| char.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day8::day8;

    #[test]
    fn test_case() {
        assert_eq!(day8("src/test_data/day8.txt"), (21, 8))
    }
}
