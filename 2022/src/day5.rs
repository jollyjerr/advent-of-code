use crate::common::read_lines;
use std::collections::VecDeque;
use std::path::Path;

pub fn day5<P: AsRef<Path>>(file_path: P) -> (String, String) {
    let (mut stacks, instructions) = create_stacks_and_instructions(&read_lines(&file_path));

    for (_, instruction) in instructions.iter().enumerate() {
        let mut moving: VecDeque<String> = VecDeque::new();
        for _count in 0..instruction[0] {
            moving.push_back(stacks[instruction[1] - 1].pop_back().unwrap());
        }
        while let Some(item) = moving.pop_front() {
            stacks[instruction[2] - 1].push_back(item);
        }
    }

    let mut part_one: String = String::new();
    for mut stack in stacks {
        part_one.push_str(stack.pop_back().unwrap().as_str());
    }

    let (mut stacks, instructions) = create_stacks_and_instructions(&read_lines(file_path));

    for (_, instruction) in instructions.iter().enumerate() {
        let mut moving: VecDeque<String> = VecDeque::new();
        for _count in 0..instruction[0] {
            moving.push_back(stacks[instruction[1] - 1].pop_back().unwrap());
        }
        while let Some(item) = moving.pop_back() {
            stacks[instruction[2] - 1].push_back(item);
        }
    }

    let mut part_two: String = String::new();
    for mut stack in stacks {
        part_two.push_str(stack.pop_back().unwrap().as_str());
    }

    let result = (part_one, part_two);

    println!("{:?}", result);

    result
}

fn create_stacks_and_instructions(lines: &Vec<String>) -> (Vec<VecDeque<String>>, Vec<[usize; 3]>) {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut stop_point = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.find("1").is_some() {
            for char in line.chars() {
                if char.to_string().parse::<i32>().is_ok() {
                    let new_stack = VecDeque::new();
                    stacks.push(new_stack);
                }
            }

            stop_point = i;
            break;
        }
    }

    let mut instructions: Vec<[usize; 3]> = Vec::new();
    let mut stack_num;
    for (i, line) in lines.iter().enumerate() {
        if i > stop_point && line.len() > 1 {
            let count: usize = line.split::<&str>("move ").collect::<Vec<&str>>()[1]
                .split::<&str>(" from")
                .collect::<Vec<&str>>()[0]
                .parse()
                .unwrap();

            let start: usize = line.split::<&str>("from ").collect::<Vec<&str>>()[1]
                .split::<&str>(" to")
                .collect::<Vec<&str>>()[0]
                .parse()
                .unwrap();

            let finish: usize = line.split::<&str>("to ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();

            let instruction: [usize; 3] = [count, start, finish];

            instructions.push(instruction);
        } else {
            stack_num = 0;
            for (j, char) in line.chars().into_iter().enumerate() {
                if j > 0 && j % 4 == 0 {
                    stack_num += 1;
                }
                if char.is_alphabetic() {
                    stacks[stack_num].push_front(char.to_string());
                }
            }
        }
    }

    (stacks, instructions)
}

#[cfg(test)]
mod tests {
    use crate::day5::day5;

    #[test]
    fn test_case() {
        assert_eq!(
            day5("src/test_data/day5.txt"),
            (String::from("CMZ"), String::from("MCD"))
        );
    }
}
