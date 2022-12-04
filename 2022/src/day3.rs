use crate::common::read_lines;
use std::path::Path;

pub fn day3<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    let rucksacks = read_lines(&file_path);
    let compartments: Vec<(&str, &str)> = rucksacks
        .iter()
        .map(|rucksack| (rucksack.split_at(rucksack.len() / 2)))
        .collect();

    for (i, (compartment_one, compartment_two)) in compartments.iter().enumerate() {
        for item in compartment_two.chars() {
            if compartment_one.find(item).is_some() {
                part_one_total += ALPHABET.find(item).unwrap() + 1;
                break;
            }
        }

        if i % 3 == 0 {
            for item in rucksacks[i].chars() {
                if rucksacks[i + 1].find(item).is_some() && rucksacks[i + 2].find(item).is_some() {
                    part_two_total += ALPHABET.find(item).unwrap() + 1;
                    break;
                }
            }
        }
    }

    let result = (
        part_one_total.try_into().unwrap(),
        part_two_total.try_into().unwrap(),
    );
    println!("{:?}", result);

    result
}

#[cfg(test)]
mod tests {
    use crate::day3::day3;

    #[test]
    fn test_case() {
        assert_eq!(day3("src/test_data/day3.txt"), (157, 70));
    }
}
