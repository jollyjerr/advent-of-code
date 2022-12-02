use std::path::Path;
use crate::common::read_lines;

pub fn day1<P: AsRef<Path>>(file_path: P) -> (i32, i32)  {
    let mut counters: Vec<i32> = Vec::new();
    let mut counter: i32 = 0;
    for line in read_lines(file_path) {
        if line == "" {
            counters.push(counter);
            counter = 0
        } else {
            let new_int: i32 = line.parse().unwrap();
            counter += new_int
        }
    }

    counters.sort_by(|a, b| b.cmp(a));
    let first = counters[0];
    let total = counters[0] + counters[1] + counters[2];

    println!("One: {}", first);
    println!("Total: {}", total);

    (first, total)
}

#[cfg(test)]
mod tests {
    use crate::day1;

#[test]
    fn test_case_1() {
        assert_eq!(day1("src/test_data/day1.txt"), (24000, 41000));
    }
}
