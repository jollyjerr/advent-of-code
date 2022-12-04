use crate::common::read_lines;
use std::path::Path;

pub fn day4<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for pair in read_lines(file_path) {
        let assignments: Vec<&str> = pair.split(",").collect();
        let first_set: Vec<&str> = assignments[0].split("-").collect();
        let second_set: Vec<&str> = assignments[1].split("-").collect();
        let low_one: i32 = first_set[0].parse().unwrap();
        let high_one: i32 = first_set[1].parse().unwrap();
        let low_two: i32 = second_set[0].parse().unwrap();
        let high_two: i32 = second_set[1].parse().unwrap();

        if (low_one <= low_two && high_one >= high_two)
            || (low_two <= low_one && high_two >= high_one)
        {
            part_one_total += 1;
        }

        let first_range: Vec<i32> = (low_one..=high_one).collect();
        let second_range: Vec<i32> = (low_two..=high_two).collect();

        // lol this sucks
        if first_range.contains(&low_two)
            || first_range.contains(&high_two)
            || second_range.contains(&low_one)
            || second_range.contains(&high_one)
        {
            part_two_total += 1;
        }
    }

    let result = (part_one_total, part_two_total);
    println!("{:?}", result);

    result
}

#[cfg(test)]
mod tests {
    use crate::day4::day4;

    #[test]
    fn test_case() {
        assert_eq!(day4("src/test_data/day4.txt"), (2, 4));
    }
}
