use crate::common::read_lines;
use std::collections::HashMap;
use std::path::Path;

pub fn day5<P: AsRef<Path>>(file_path: P, part_two: bool) -> u32 {
    let mut data = pull_data_from_lines(read_lines(file_path)).to_owned();
    let seeds = data.first().unwrap().first().unwrap().to_owned();
    data.remove(0);

    println!("seeds {:?}", seeds);
    println!("data {:?}", data);

    let mut maps: [HashMap<u32, u32>; 7] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    let mut map_fill_idx = 0;

    for vals in data {
        for line in vals {
            for modifier in 0..line[2] {
                maps[map_fill_idx].insert(line[1] + modifier, line[0] + modifier);
            }
        }
        map_fill_idx += 1;
    }

    let mut out: u32 = 99999999;

    for seed in seeds {
        let mut current: u32 = seed;

        for map in &maps {
            current = *map.get(&current).unwrap_or(&current);
        }

        if current < out {
            out = current;
        }
    }

    out
}

fn pull_data_from_lines(lines: Vec<String>) -> Vec<Vec<Vec<u32>>> {
    let mut data: Vec<Vec<Vec<u32>>> = vec![];

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

fn line_of_numbers(line: String) -> Vec<u32> {
    line.split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|c| c.parse::<u32>().unwrap())
        .collect()
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
