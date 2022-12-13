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

        println!("directions {:?}", directions);

        let direction = directions[0];
        let steps = directions[1].parse::<i32>().unwrap();

        for _step in 0..steps {
            let mut h_position = positions[0];
            move_head(&mut h_position, direction);
            positions[0] = h_position;

            for t_id in 1..positions.len() {
                let h_position = positions[t_id - 1];
                let mut t_position = positions[t_id];

                t_position = move_tail(&h_position, &t_position);

                if t_id == (number_of_knots - 1).try_into().unwrap() {
                    if !unique_t_positions.contains_key(&t_position) {
                        unique_t_position_count += 1;
                    }
                    unique_t_positions.insert(t_position.to_owned(), true);
                }

                positions[t_id] = t_position;
            }
        }

        println!("results {:?}", positions);
    }

    unique_t_position_count
}

fn move_head(h_position: &mut [i32; 2], direction: &str) {
    match direction {
        "L" => {
            h_position[0] -= 1;
        }
        "U" => {
            h_position[1] += 1;
        }
        "R" => {
            h_position[0] += 1;
        }
        "D" => {
            h_position[1] -= 1;
        }
        _ => unreachable!(),
    }
}

fn move_tail(h: &[i32; 2], t: &[i32; 2]) -> [i32; 2] {
    let difx = h[0].max(t[0]) - h[0].min(t[0]);
    let dify = h[1].max(t[1]) - h[1].min(t[1]);

    println!("h {:?} t {:?} difx {:?} dify {:?}", h, t, difx, dify);

    if difx > 1 {
        if dify == 1 {
            return [get_one_closer(h[0], t[0]), h[1]];
        }
        return [get_one_closer(h[0], t[0]), t[1]];
    } else if dify > 1 {
        if difx == 1 {
            return [h[0], get_one_closer(h[1], t[1])];
        }
        return [t[0], get_one_closer(h[1], t[1])];
    }

    *t
}

fn get_one_closer(h: i32, t: i32) -> i32 {
    if h > t {
        return t + 1;
    }
    t - 1
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
        assert_eq!(day9("src/test_data/day9-2.txt"), (88, 37));
    }
}
