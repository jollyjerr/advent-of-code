use std::{collections::hash_map::HashMap, iter::Cycle, str::Chars};

pub fn day8(lines: Vec<String>, is_part_two: bool) -> usize {
    let instructions = lines[0].trim().chars();

    let mut instructions_cycle = instructions.clone().cycle();
    let mut graph: HashMap<String, [String; 2]> = HashMap::new();

    for line in &lines[2..] {
        let parts: Vec<&str> = line.split(" = ").collect();

        graph.insert(
            parts[0].to_owned(),
            [
                parts[1]
                    .split("(")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .nth(0)
                    .unwrap()
                    .to_owned(),
                parts[1]
                    .split(")")
                    .nth(0)
                    .unwrap()
                    .split(", ")
                    .nth(1)
                    .unwrap()
                    .to_owned(),
            ],
        );
    }

    match is_part_two {
        false => part_one(
            String::from("AAA"),
            |word| word == "ZZZ",
            graph,
            &mut instructions_cycle,
        ),
        true => part_two(graph, &mut instructions_cycle),
    }
}

fn part_one(
    start: String,
    is_correct: fn(word: &String) -> bool,
    graph: HashMap<String, [String; 2]>,
    instructions_cycle: &mut Cycle<Chars<'_>>,
) -> usize {
    let mut current: String = start.to_owned();
    let mut count = 0;

    while !is_correct(&current) {
        let direction = instructions_cycle.next().unwrap();

        current = graph.get(&current).unwrap()[match direction {
            'L' => 0,
            _ => 1,
        }]
        .to_owned();

        count += 1;
    }

    count
}

fn part_two(
    graph: HashMap<String, [String; 2]>,
    instructions_cycle: &mut Cycle<Chars<'_>>,
) -> usize {
    let mut nodes: Vec<String> = vec![];

    for (key, _) in &graph {
        if key.chars().nth(2).unwrap() == 'A' {
            nodes.push(key.clone());
        }
    }

    least_common_multiple(
        nodes
            .iter()
            .map(|n| {
                part_one(
                    n.to_owned(),
                    |word| word.chars().nth(2).unwrap() == 'Z',
                    graph.to_owned(),
                    instructions_cycle,
                )
            })
            .collect::<Vec<usize>>(),
    )
}

fn least_common_multiple(nums: Vec<usize>) -> usize {
    nums.into_iter()
        .fold(1, |acc, num| acc * num / greatest_common_divisor(acc, num))
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::common::read_lines;
    use crate::day8::day8;

    #[test]
    fn test_one() {
        assert_eq!(day8(read_lines("src/data/day8a.txt"), false), 6);
    }

    #[test]
    fn test_two() {
        assert_eq!(day8(read_lines("src/data/day8b.txt"), false), 12643);
    }

    #[test]
    fn test_three() {
        assert_eq!(day8(read_lines("src/data/day8c.txt"), true), 6);
    }

    #[test]
    fn test_four() {
        assert_eq!(day8(read_lines("src/data/day8b.txt"), true), 13133452426987);
    }
}
