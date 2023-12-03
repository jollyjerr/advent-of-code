use crate::common::read_lines;
use std::path::Path;

pub fn day3<P: AsRef<Path>>(file_path: P, part_two: bool) -> u32 {
    let graph: Vec<Vec<char>> = read_lines(file_path)
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut sum: u32 = 0;

    for (ridx, row) in graph.iter().enumerate() {
        for (cidx, col) in row.iter().enumerate() {
            if !col.is_digit(10) && col != &'.' {
                let mut results: Vec<u32> = Vec::new();

                vec![
                    (ridx - 1, cidx + 1),
                    (ridx - 1, cidx),
                    (ridx - 1, cidx - 1),
                    (ridx, cidx - 1),
                    (ridx + 1, cidx - 1),
                    (ridx + 1, cidx),
                    (ridx + 1, cidx + 1),
                    (ridx, cidx + 1),
                ]
                .iter()
                .for_each(|tup| {
                    results.push(extract_full_number(tup.0, tup.1, &graph).unwrap_or(0))
                });

                // I hate this but I timebox these problems
                results.sort();
                results.dedup();
                sum += results.iter().sum::<u32>();
            }
        }
    }

    sum
}

fn extract_full_number(ridx: usize, cidx: usize, graph: &Vec<Vec<char>>) -> Option<u32> {
    if graph[ridx][cidx].is_digit(10) {
        let mut digits: Vec<char> = vec![];
        let mut j = cidx.to_owned();

        digits.push(graph[ridx][j]);

        while j > 0
            && match graph.get(ridx) {
                Some(val) => match val.get(j - 1) {
                    Some(result) => result.is_digit(10),
                    None => false,
                },
                None => false,
            }
        {
            digits.insert(0, graph[ridx][j - 1]);
            j = j - 1;
        }

        j = cidx;

        while match graph.get(ridx) {
            Some(val) => match val.get(j + 1) {
                Some(result) => result.is_digit(10),
                None => false,
            },
            None => false,
        } {
            digits.push(graph[ridx][j + 1]);
            j = j + 1;
        }

        let full: u32 = digits
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();

        println!("{:?}: {:?}", ridx, full);

        return Some(full);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::day3;

    #[test]
    fn test_one() {
        assert_eq!(day3("src/data/day3a.txt", false), 4361);
    }

    #[test]
    fn test_two() {
        assert_eq!(day3("src/data/day3b.txt", false), 528819);
    }
}
