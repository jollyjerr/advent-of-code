use crate::common::read_lines;
use std::path::Path;

pub fn day10<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let mut signal_strength = 0;

    let cycle_checks = [20, 60, 100, 140, 180, 220];

    let mut x = 1;
    let mut clock = 0;
    for line in read_lines(file_path) {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["addx", count] => {
                clock += 1;
                if cycle_checks.contains(&clock) {
                    signal_strength += clock * x;
                }

                clock += 1;
                if cycle_checks.contains(&clock) {
                    signal_strength += clock * x;
                }
                x += count.parse::<i32>().unwrap();
            }
            _ => {
                clock += 1;
                if cycle_checks.contains(&clock) {
                    signal_strength += clock * x;
                }
            }
        }
    }

    (signal_strength, 0)
}

#[cfg(test)]
mod tests {
    use crate::day10::day10;

    #[test]
    fn test_case() {
        assert_eq!(day10("src/test_data/day10.txt"), (13140, 0));
    }
}
