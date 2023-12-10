fn day9(lines: Vec<String>, is_part_two: bool) -> isize {
    lines
        .iter()
        .map(|line| {
            let mut nums = get_values(line);

            if is_part_two {
                nums.reverse();
            }

            let mut layers: Vec<Vec<isize>> = vec![nums];

            while !layers.last().unwrap().iter().all(|&v| v == 0) {
                layers.push(derive(layers.last().unwrap()));
            }

            let mut out = 0;

            for (i, _layer) in layers.iter().enumerate().rev() {
                if i > 0 {
                    let compare = layers[i - 1].last().unwrap();
                    out += compare;
                }
            }

            out
        })
        .sum()
}

fn get_values(line: &String) -> Vec<isize> {
    line.split(" ")
        .into_iter()
        .map(|v| v.parse().unwrap())
        .collect()
}

fn derive(line: &Vec<isize>) -> Vec<isize> {
    let mut out = vec![];

    for (i, &val) in line.iter().enumerate() {
        line.get(i + 1).and_then(|v| Some(out.push(v - val)));
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::common::read_lines;
    use crate::day9::day9;

    #[test]
    fn test_one() {
        assert_eq!(day9(read_lines("src/data/day9a.txt"), false), 114);
    }

    #[test]
    fn test_two() {
        assert_eq!(day9(read_lines("src/data/day9b.txt"), false), 1666172641);
    }

    #[test]
    fn test_three() {
        assert_eq!(day9(read_lines("src/data/day9a.txt"), true), 2);
    }

    #[test]
    fn test_four() {
        assert_eq!(day9(read_lines("src/data/day9b.txt"), true), 933);
    }
}
