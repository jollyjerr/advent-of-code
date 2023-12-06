use crate::common::read_lines;
use std::path::Path;

pub fn day5<P: AsRef<Path>>(file_path: P, part_two: bool) -> u128 {
    let mut data = pull_data_from_lines(read_lines(file_path)).to_owned();
    let seeds = data.first().unwrap().first().unwrap().to_owned();
    data.remove(0);

    let mut out: u128 = 999999999999999999;

    for seed in seeds {
        let mut carry = seed.to_owned();

        for map in &data {
            for round in map {
                match get_mapping(carry, &round) {
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

        if carry < out {
            out = carry;
        }
    }

    out
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

fn get_mapping(val: u128, rules: &Vec<u128>) -> Option<u128> {
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
        assert_eq!(day5("src/data/day5b.txt", false), 35);
    }
}
