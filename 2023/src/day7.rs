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

    hands.sort_by(camel_cards);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid))
}

fn camel_cards<'a, 'b>(first: &'a Hand, second: &'b Hand) -> Ordering {
    let a = hand_ranking(&first.cards);
    let b = hand_ranking(&second.cards);

    if a == b {
        for (first, second) in first.cards.chars().zip(second.cards.chars()) {
            if char_ranking(first) == char_ranking(second) {
                continue;
            } else {
                return char_ranking(first).cmp(&char_ranking(second));
            }
        }
        unreachable!("duplicate hands")
    } else {
        a.cmp(&b)
    }
}

fn hand_ranking(hand: &String) -> u32 {
    let mut counts: [usize; 13] = [0; 13];

    for ch in hand.chars() {
        counts[char_ranking(ch)] += 1;
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

fn char_ranking(ch: char) -> usize {
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
    //
    // #[test]
    // fn test_three() {
    //     assert_eq!(day3("src/data/day3a.txt", true), 467835);
    // }
    //
    // #[test]
    // fn test_four() {
    //     assert_eq!(day3("src/data/day3b.txt", true), 80403602);
    // }
}
