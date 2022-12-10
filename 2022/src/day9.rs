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

    println!("{}", number_of_knots);

    for line in lines {
        println!("step {:?}", line);
        let directions = line.split(" ").collect::<Vec<&str>>().as_slice().to_owned();

        let direction = directions[0];
        let steps = directions[1].parse::<i32>().unwrap();

        for knot in 0..steps {
            if knot == 0 {
                let mut h_position = positions[knot as usize];
                move_head(&mut h_position, direction);
            } else {
                let mut h_position = positions[knot as usize + 1];
                let mut t_position = positions[knot as usize];

                move_head(&mut h_position, direction);
                t_position = move_tail(&h_position, &t_position, knot % number_of_knots == 0);

                if knot == (number_of_knots - 2) {
                    if !unique_t_positions.contains_key(&t_position) {
                        unique_t_position_count += 1;
                    }
                    unique_t_positions.insert(t_position.to_owned(), true);
                }

                positions[knot as usize + 1] = h_position;
                positions[knot as usize] = t_position;
            }
        }

        println!("result {:?}", positions);
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

fn move_tail(h: &[i32; 2], t: &[i32; 2], use_diag: bool) -> [i32; 2] {
    let difx = h[0].max(t[0]) - h[0].min(t[0]);
    let dify = h[1].max(t[1]) - h[1].min(t[1]);

    if difx > 1 && dify > 1 && use_diag {
        return [get_one_closer(h[0], t[0]), get_one_closer(h[1], t[1])];
    } else if difx > 1 {
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
        assert_eq!(day9("src/test_data/day9.txt"), (13, 1))
    }
}
