use crate::common::read_lines;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

pub fn day5<P: AsRef<Path>>(file_path: P, part_two: bool) -> u128 {
    let mut data = pull_data_from_lines(read_lines(file_path)).to_owned();
    let seeds = data.first().unwrap().first().unwrap().to_owned();
    data.remove(0);

    let mut results: Vec<u128> = vec![];

    if part_two {
        let (tx, rx) = mpsc::channel();

        for (idx, seed) in seeds.iter().enumerate() {
            if idx % 2 != 0 {
                continue;
            }

            for val in *seed..(seed + seeds[idx + 1]) {
                let vaal = val.to_owned();
                let deeta = data.to_owned();
                let txx = tx.to_owned();

                thread::spawn(move || {
                    txx.send(get_mapping_for_seed(vaal, &deeta)).unwrap();
                });

                results.push(rx.recv().unwrap());
            }
        }
    } else {
        for seed in seeds {
            results.push(get_mapping_for_seed(seed, &data));
        }
    }

    *results.iter().min().unwrap()
}

fn pull_data_from_lines(lines: Vec<String>) -> Vec<Vec<Vec<u128>>> {
    let mut data: Vec<Vec<Vec<u128>>> = vec![];

    let mut outer_vec_idx = 0;

    for line in lines {
        if line.contains("seeds:") {
            data.push(vec![line_of_numbers(
                line.split(": ").nth(1).unwrap().to_string(),
            )])
        } else if line.contains(":") {
            data.push(Vec::new());
            outer_vec_idx += 1;
        } else if line.len() > 1 {
            data[outer_vec_idx].push(line_of_numbers(line))
        }
    }

    data
}

fn line_of_numbers(line: String) -> Vec<u128> {
    line.split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|c| c.parse::<u128>().unwrap())
        .collect()
}

fn get_mapping_for_seed(seed: u128, data: &Vec<Vec<Vec<u128>>>) -> u128 {
    let mut carry = seed.to_owned();

    for map in data {
        for round in map {
            match get_mapping_for_line(carry, &round) {
                Some(value) => {
                    carry = value;
                    break;
                }
                None => {
                    // noop
                }
            }
        }
    }

    carry
}

fn get_mapping_for_line(val: u128, rules: &Vec<u128>) -> Option<u128> {
    let low = rules[1];
    let high = rules[1] + rules[2];

    if val >= low && val < high {
        return Some(rules[0] + (val - rules[1]));
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day5::day5;

    #[test]
    fn test_one() {
        assert_eq!(day5("src/data/day5a.txt", false), 35);
    }

    #[test]
    fn test_two() {
        assert_eq!(day5("src/data/day5b.txt", false), 462648396);
    }

    #[test]
    fn test_three() {
        assert_eq!(day5("src/data/day5a.txt", true), 46);
    }

    // #[test]
    // fn test_four() {
    //     assert_eq!(day5("src/data/day5b.txt", true), 46);
    // }
}
