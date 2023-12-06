use crate::common::read_lines;
use std::path::Path;

pub fn day6<P: AsRef<Path>>(file_path: P, part_two: bool) -> u64 {
    let mut data: Vec<Vec<u64>> = vec![];

    for line in read_lines(file_path) {
        if part_two {
            data.push(vec![line
                .split(":")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>()
                .join("")
                .to_string()
                .parse::<u64>()
                .unwrap()])
        } else {
            data.push(
                line.split(" ")
                    .map(|i| i.parse::<u64>())
                    .filter(|x| x.is_ok())
                    .map(|i| i.unwrap())
                    .collect::<Vec<u64>>(),
            );
        }
    }

    let mut results = vec![];

    for (round_idx, &time) in data[0].iter().enumerate() {
        let mut count = 0;
        for press_time in 0..=time {
            let remaining_time = time - press_time;
            let distance = press_time * remaining_time;
            if distance > data[1][round_idx] {
                count += 1;
            }
        }
        results.push(count);
    }

    results.iter().fold(1, |acc, item| acc * item)
}

#[cfg(test)]
mod tests {
    use crate::day6::day6;

    #[test]
    fn test_one() {
        assert_eq!(day6("src/data/day6a.txt", false), 288);
    }

    #[test]
    fn test_two() {
        assert_eq!(day6("src/data/day6b.txt", false), 303600);
    }

    #[test]
    fn test_three() {
        assert_eq!(day6("src/data/day6a.txt", true), 71503);
    }

    #[test]
    fn test_four() {
        assert_eq!(day6("src/data/day6b.txt", true), 23654842);
    }
}
