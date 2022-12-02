use crate::common::read_lines;
use std::path::Path;

pub fn day2<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for line in read_lines(file_path) {
        let plays: Vec<&str> = line.split(" ").collect();

        part_one_total += get_round_score(plays[0], plays[1], false);
        part_two_total += get_round_score(plays[0], plays[1], true);
    }

    println!("one: {}", part_one_total);
    println!("two: {}", part_two_total);

    (part_one_total, part_two_total)
}

fn get_round_score(them: &str, you: &str, part_two: bool) -> i32 {
    let your_hand: &str;
    if part_two {
        your_hand = get_hand_reverse(them, you);
    } else {
        your_hand = get_hand_match(you);
    }

    return get_hand_score(your_hand) + get_pair_score(them, your_hand);
}

fn get_hand_score(hand: &str) -> i32 {
    match hand {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!(),
    }
}

fn get_hand_match(hand: &str) -> &str {
    match hand {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!(),
    }
}

fn get_hand_reverse<'a>(them: &'a str, you: &'a str) -> &'a str {
    match them {
        "A" => match you {
            "X" => "C",
            "Y" => "A",
            "Z" => "B",
            _ => panic!(),
        },
        "B" => match you {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => panic!(),
        },
        "C" => match you {
            "X" => "B",
            "Y" => "C",
            "Z" => "A",
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn get_pair_score<'a>(them: &'a str, you: &'a str) -> i32 {
    match them {
        "A" => match you {
            "A" => 3,
            "B" => 6,
            "C" => 0,
            _ => panic!(),
        },
        "B" => match you {
            "A" => 0,
            "B" => 3,
            "C" => 6,
            _ => panic!(),
        },
        "C" => match you {
            "A" => 6,
            "B" => 0,
            "C" => 3,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::day2;

    #[test]
    fn test_case() {
        assert_eq!(day2("src/test_data/day2.txt"), (15, 12));
    }
}
