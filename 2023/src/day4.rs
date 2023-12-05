use crate::common::read_lines;
use std::path::Path;

pub fn day4<P: AsRef<Path>>(file_path: P, part_two: bool) -> u32 {
    if part_two {
        return solve_part_two(read_lines(file_path));
    }

    let mut out: u32 = 0;

    for line in read_lines(file_path) {
        let data = line_to_game(&line);

        let mut round: u32 = 0;

        data[1].iter().for_each(|item| {
            if data[0].contains(&item) {
                if round > 0 {
                    round = round * 2;
                } else {
                    round = 1;
                }
            }
        });

        out += round;
    }

    out
}

fn solve_part_two(lines: Vec<String>) -> u32 {
    let cards = lines
        .iter()
        .map(line_to_game)
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut counts = cards.iter().map(|_| 1).collect::<Vec<u32>>();

    cards.iter().enumerate().for_each(|(idx, card)| {
        let mut current = idx + 1;

        card[1].iter().for_each(|item| {
            if card[0].contains(item) {
                counts[current] += counts[idx];
                current += 1;
            }
        });
    });

    counts.iter().sum()
}

fn line_to_game(line: &String) -> Vec<Vec<u32>> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            l.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .filter(|i| i.parse::<u32>().is_ok())
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use crate::day4::day4;

    #[test]
    fn test_one() {
        assert_eq!(day4("src/data/day4a.txt", false), 13);
    }

    #[test]
    fn test_two() {
        assert_eq!(day4("src/data/day4b.txt", false), 23750);
    }

    #[test]
    fn test_three() {
        assert_eq!(day4("src/data/day4a.txt", true), 30);
    }

    #[test]
    fn test_four() {
        assert_eq!(day4("src/data/day4b.txt", true), 13261850);
    }
}
