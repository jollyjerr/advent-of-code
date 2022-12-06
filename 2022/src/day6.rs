use crate::common::read_lines;
use std::{
    collections::{hash_map::HashMap, VecDeque},
    path::Path,
};

pub fn day6<P: AsRef<Path>>(file_path: P) -> (usize, usize) {
    let lines = read_lines(file_path);

    let result = (
        find_first_marker(&lines, 4).unwrap(),
        find_first_marker(&lines, 14).unwrap(),
    );

    println!("{:?}", result);
    result
}

fn find_first_marker(lines: &Vec<String>, marker_length: i32) -> Option<usize> {
    let mut seen: HashMap<char, usize> = HashMap::new();
    let mut window: VecDeque<char> = VecDeque::new();
    for line in lines {
        for (i, char) in line.chars().into_iter().enumerate() {
            match seen.get(&char) {
                Some(count) => {
                    seen.insert(char, count + 1);
                }
                None => {
                    seen.insert(char, 1);
                }
            }
            window.push_back(char);

            if window.len() >= marker_length.try_into().unwrap() {
                if seen.values().all(|v| *v == 1 as usize) {
                    return Some(i + 1);
                } else {
                    let to_remove = window.pop_front().unwrap();
                    match seen.get(&to_remove) {
                        None => panic!(),
                        Some(count) => {
                            if count > &1 {
                                seen.insert(to_remove, count - 1);
                            } else {
                                seen.remove(&to_remove);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day6::day6;

    #[test]
    fn test_case() {
        assert_eq!(day6("src/test_data/day6.txt"), (11, 26));
    }
}
