use crate::common::read_lines;
use std::path::Path;

pub fn day2<P: AsRef<Path>>(file_path: P, part_two: bool) -> u32 {
    if part_two {
        return solve_part_two(read_lines(file_path));
    }

    let mut possible: Vec<u32> = Vec::new();

    for (index, line) in read_lines(file_path).iter().enumerate() {
        let rounds = get_rounds(line);

        let mut valid: bool = true;
        for round in rounds {
            if get_val(&round, "red") > 12
                || get_val(&round, "green") > 13
                || get_val(&round, "blue") > 14
            {
                valid = false;
            }
        }

        if valid {
            possible.push((index + 1).try_into().unwrap());
        }
    }

    possible.iter().sum()
}

fn solve_part_two(lines: Vec<String>) -> u32 {
    let mut powers: Vec<u32> = Vec::new();

    for line in lines {
        let rounds = get_rounds(&line);
        let mut maxes: [u32; 3] = [0, 0, 0];

        for round in rounds {
            let red = get_val(&round, "red");
            let green = get_val(&round, "green");
            let blue = get_val(&round, "blue");

            if red > maxes[0] {
                maxes[0] = red;
            }
            if green > maxes[1] {
                maxes[1] = green;
            }
            if blue > maxes[2] {
                maxes[2] = blue;
            }
        }

        powers.push(maxes[0] * maxes[1] * maxes[2]);
    }

    powers.iter().sum()
}

fn get_rounds(line: &str) -> Vec<Vec<&str>> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split("; ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|round| round.split(", ").collect())
        .collect()
}

fn get_val(round: &Vec<&str>, term: &str) -> u32 {
    match round.iter().find(|x| x.contains(term)) {
        Some(term) => term.split(" ").nth(0).unwrap().parse().unwrap(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::day2;

    #[test]
    fn test_one() {
        assert_eq!(day2("src/data/day2a.txt", false), 8);
    }

    #[test]
    fn test_two() {
        assert_eq!(day2("src/data/day2b.txt", false), 2505);
    }

    #[test]
    fn test_three() {
        assert_eq!(day2("src/data/day2a.txt", true), 2286);
    }

    #[test]
    fn test_four() {
        assert_eq!(day2("src/data/day2b.txt", true), 70265);
    }
}
