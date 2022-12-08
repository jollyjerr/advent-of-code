use crate::common::read_lines;
use std::path::Path;

pub fn day7<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let mut directory_sizes: Vec<i32> = Vec::new();
    let mut current_directory: Vec<i32> = Vec::new();

    for line in read_lines(file_path) {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => {
                directory_sizes.push(current_directory.pop().unwrap());
            }
            ["$", "cd", _dir] => {
                current_directory.push(0);
            }
            [info, _file_name] => {
                if info.chars().nth(0).unwrap().is_numeric() {
                    current_directory
                        .iter_mut()
                        .for_each(|value| *value += info.parse::<i32>().unwrap())
                }
            }
            _ => panic!(),
        }
    }
    directory_sizes.extend(&current_directory);

    let total_size = current_directory[0];

    (
        directory_sizes.iter().filter(|&size| size <= &100000).sum(),
        *directory_sizes
            .iter()
            .filter(|&size| 70000000 - (total_size - size) >= 30000000)
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::day7::day7;

    #[test]
    fn test_case() {
        assert_eq!(day7("src/test_data/day7.txt"), (95437, 24933642))
    }
}
