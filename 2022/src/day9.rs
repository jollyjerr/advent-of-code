use crate::common::read_lines;
use std::{collections::HashMap, path::Path};

pub fn day9<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let lines = read_lines(file_path);

    (track_knots(&lines, 2), track_knots(&lines, 10))
}

fn track_knots(lines: &Vec<String>, number_of_knots: i32) -> i32 {
    let mut unique_t_position_count = 0;
    let mut unique_t_positions: HashMap<[i32; 2], bool> = HashMap::new();
    let mut positions = vec![[0, 0]; number_of_knots as usize];

    for line in lines {
        let directions = line.split(" ").collect::<Vec<&str>>().as_slice().to_owned();

        // println!("direction: {:?}", directions);

        let direction = directions[0];
        let steps = directions[1].parse::<i32>().unwrap();

        for _step in 0..steps {
            positions[0] = move_head(positions[0], direction);

            for t_id in 1..positions.len() {
                if !touching(&positions[t_id - 1], &positions[t_id]) {
                    positions[t_id] = move_tail(&positions[t_id - 1], &positions[t_id]);
                }

                if t_id == (number_of_knots - 1).try_into().unwrap() {
                    if !unique_t_positions.contains_key(&positions[t_id]) {
                        unique_t_position_count += 1;
                    }
                    unique_t_positions.insert(positions[t_id].to_owned(), true);
                }
            }
        }

        // println!("results {:?}", positions);
    }

    unique_t_position_count
}

fn move_head(h_position: [i32; 2], direction: &str) -> [i32; 2] {
    match direction {
        "L" => {
            return [h_position[0] - 1, h_position[1]];
        }
        "U" => {
            return [h_position[0], h_position[1] + 1];
        }
        "R" => {
            return [h_position[0] + 1, h_position[1]];
        }
        "D" => {
            return [h_position[0], h_position[1] - 1];
        }
        _ => unreachable!(),
    }
}

fn touching(h: &[i32; 2], t: &[i32; 2]) -> bool {
    return (h[0] - t[0]).abs() <= 1 && (h[1] - t[1]).abs() <= 1;
}

fn move_tail(h: &[i32; 2], t: &[i32; 2]) -> [i32; 2] {
    let move_x = if h[0] == t[0] {
        0
    } else {
        (h[0] - t[0]) / ((h[0] - t[0]).abs())
    };
    let move_y = if h[1] == t[1] {
        0
    } else {
        (h[1] - t[1]) / ((h[1] - t[1]).abs())
    };

    [t[0] + move_x, t[1] + move_y]
}

#[cfg(test)]
mod tests {
    use crate::day9::day9;

    #[test]
    fn test_case() {
        assert_eq!(day9("src/test_data/day9.txt"), (13, 1));
    }

    #[test]
    fn larger_test_case() {
        assert_eq!(day9("src/test_data/day9-2.txt"), (88, 36));
    }
}
