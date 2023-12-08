use std::collections::hash_map::HashMap;

pub fn day8(lines: Vec<String>, part_two: bool) -> usize {
    let instructions = lines[0].trim().chars();

    let mut instructions_cycle = instructions.clone().cycle();
    let mut graph: HashMap<String, [String; 2]> = HashMap::new();
    let mut current: String = String::from("AAA");

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

    let mut count = 0;

    while current != "ZZZ" {
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
        assert_eq!(day8(read_lines("src/data/day8b.txt"), false), 8);
    }
}
