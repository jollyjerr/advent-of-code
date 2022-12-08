use crate::common::read_lines;
use std::path::Path;

pub fn day8<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let trees = get_grid_from_lines(read_lines(file_path));
    let mut visibility_record = create_visibility_grid(&trees, false);
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
            if tree > &row_max[i] && tree > &col_max[j] {
                row_max[i] = *tree;
                col_max[j] = *tree;

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
            if tree > &row_max[i] && tree > &col_max[j] {
                row_max[i] = *tree;
                col_max[j] = *tree;

                if visibility_record[i][j] == false {
                    visibility_record[i][j] = true;
                    total_visible += 1;
                }
            }
        }
    }

    println!("{:?}", visibility_record);

    (total_visible, 0)
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

fn create_visibility_grid<TRet>(grid: &Vec<Vec<i32>>, fill: TRet) -> Vec<Vec<TRet>>
where
    TRet: Copy,
{
    grid.iter()
        .map(|line| line.iter().map(|_tree| fill).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day8::day8;

    #[test]
    fn test_case() {
        assert_eq!(day8("src/test_data/day8.txt"), (21, 0))
    }
}
