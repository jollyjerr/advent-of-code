use crate::common::read_lines;
use std::cmp::Ordering;
use std::path::Path;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
}

pub fn day7<P: AsRef<Path>>(file_path: P, part_two: bool) -> usize {
    let mut hands: Vec<Hand> = read_lines(file_path)
        .iter()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            Hand {
                cards: parts[0].to_owned(),
                bid: parts[1].to_owned().parse().unwrap(),
            }
        })
        .collect();

    hands.sort_by(|a, b| camel_cards(a, b, part_two));

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid))
}

fn camel_cards<'a, 'b>(first: &'a Hand, second: &'b Hand, part_two: bool) -> Ordering {
    let a = hand_ranking(&first.cards, part_two);
    let b = hand_ranking(&second.cards, part_two);

    if a == b {
        for (first, second) in first.cards.chars().zip(second.cards.chars()) {
            if char_ranking(first, part_two) == char_ranking(second, part_two) {
                continue;
            } else {
                return char_ranking(first, part_two).cmp(&char_ranking(second, part_two));
            }
        }
        unreachable!("duplicate hands")
    } else {
        a.cmp(&b)
    }
}

fn hand_ranking(hand: &String, part_two: bool) -> u32 {
    let mut counts: [usize; 13] = [0; 13];

    for ch in hand.chars() {
        counts[char_ranking(ch, part_two)] += 1;
    }

    if part_two && hand.contains('J') {
        let num_js = counts[0];

        counts[0] = 0;

        let highest = counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, val)| val)
            .map(|(index, _)| index)
            .unwrap();

        counts[highest] += num_js;
    }

    counts.sort();

    match &counts[8..] {
        [1, 1, 1, 1, 1] => 0, // high card
        [0, 1, 1, 1, 2] => 1, // one pair
        [0, 0, 1, 2, 2] => 2, // two pair
        [0, 0, 1, 1, 3] => 3, // three of a kind
        [0, 0, 0, 2, 3] => 4, // full house
        [0, 0, 0, 1, 4] => 5, // four of a kind
        [0, 0, 0, 0, 5] => 6, // five of a kind
        _ => unreachable!(),
    }
}

fn char_ranking(ch: char, part_two: bool) -> usize {
    if part_two {
        return match ch {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        };
    }
    match ch {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::day7;

    #[test]
    fn test_one() {
        assert_eq!(day7("src/data/day7a.txt", false), 6440);
    }

    #[test]
    fn test_two() {
        assert_eq!(day7("src/data/day7b.txt", false), 249638405);
    }

    #[test]
    fn test_three() {
        assert_eq!(day7("src/data/day7a.txt", true), 5905);
    }

    #[test]
    fn test_four() {
        assert_eq!(day7("src/data/day7b.txt", true), 249776650);
    }
}
