use crate::common::read_lines;
use std::path::Path;

const WORD_TO_NUM: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn day1<P: AsRef<Path>>(file_path: P, part_two: bool) -> u32 {
    let mut results: Vec<u32> = Vec::new();

    for line in read_lines(file_path) {
        if part_two {
            let first = find_first_num(0..line.len(), &line.as_str());
            let last = find_first_num((0..line.len()).rev(), &line.as_str());
            results.push(10 * first + last);
        } else {
            let ints: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();

            results.push(
                (ints.first().unwrap().to_string() + &ints.last().unwrap().to_string())
                    .parse::<u32>()
                    .unwrap(),
            );
        }
    }

    results.iter().sum()
}

fn find_first_num(mut it: impl Iterator<Item = usize>, line: &str) -> u32 {
    it.find_map(|i| compare_slice(&line[i..])).unwrap()
}

fn compare_slice(slice: &str) -> Option<u32> {
    WORD_TO_NUM
        .iter()
        .enumerate()
        .find(|(_, pattern)| slice.starts_with(*pattern))
        .map(|(i, _)| i as u32 + 1)
        .or_else(|| slice.chars().next().unwrap().to_digit(10))
}

#[cfg(test)]
mod tests {
    use crate::day1::day1;

    #[test]
    fn test_one() {
        assert_eq!(day1("src/data/day1a.txt", false), 142)
    }

    #[test]
    fn test_two() {
        assert_eq!(day1("src/data/day1b.txt", false), 53651)
    }

    #[test]
    fn test_three() {
        assert_eq!(day1("src/data/day1c.txt", true), 281)
    }

    #[test]
    fn test_four() {
        assert_eq!(day1("src/data/day1b.txt", true), 53894)
    }
}
