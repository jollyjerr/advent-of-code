use std::collections::VecDeque;
use std::path::Path;

enum MonkeySign {
    Add(),
    Multiply(),
    Square(),
}

struct Monkey {
    items: VecDeque<i64>,
    inspection_count: i64,
    sign: MonkeySign,
    value: i64,
    divisibility: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn get_next_item(&mut self) -> Option<i64> {
        let item = self.items.pop_front();
        if item.is_some() {
            self.inspection_count += 1;
        }

        item
    }
    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
    fn inspect(&mut self, with_relief: bool) -> Option<(usize, i64)> {
        if let Some(item) = self.get_next_item() {
            let mut new_item_value = match self.sign {
                MonkeySign::Add() => item + self.value,
                MonkeySign::Multiply() => item * self.value,
                MonkeySign::Square() => item * item,
            };

            if with_relief {
                new_item_value = adjust_value_for_relief(new_item_value)
            }

            if new_item_value % self.divisibility == 0 {
                Some((self.true_monkey, new_item_value))
            } else {
                Some((self.false_monkey, new_item_value))
            }
        } else {
            None
        }
    }
}

fn adjust_value_for_relief(item: i64) -> i64 {
    ((item / 3) as f64).floor() as i64
}

fn round(monkeys: &mut Vec<Monkey>, with_relief: bool) {
    for i in 0..monkeys.len() {
        while let Some((target, value)) = monkeys[i].inspect(with_relief) {
            monkeys[target].add_item(value);
        }
    }
}

fn get_test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            inspection_count: 0,
            items: VecDeque::from([79, 98]),
            sign: MonkeySign::Multiply(),
            value: 19,
            divisibility: 23,
            true_monkey: 2,
            false_monkey: 3,
        },
        Monkey {
            inspection_count: 0,
            items: VecDeque::from([54, 65, 75, 74]),
            sign: MonkeySign::Add(),
            value: 6,
            divisibility: 19,
            true_monkey: 2,
            false_monkey: 0,
        },
        Monkey {
            inspection_count: 0,
            items: VecDeque::from([79, 60, 97]),
            sign: MonkeySign::Square(),
            value: 0,
            divisibility: 13,
            true_monkey: 1,
            false_monkey: 3,
        },
        Monkey {
            inspection_count: 0,
            items: VecDeque::from([74]),
            sign: MonkeySign::Add(),
            value: 3,
            divisibility: 17,
            true_monkey: 0,
            false_monkey: 1,
        },
    ]
}

pub fn day11<P: AsRef<Path>>(_file_path: P) -> (i64, i64) {
    let mut test_monkeys = get_test_monkeys();
    for _ in 0..20 {
        round(&mut test_monkeys, true);
    }
    test_monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let mut big_test_monkeys = get_test_monkeys();
    for _ in 0..1000 {
        round(&mut big_test_monkeys, false);
    }
    big_test_monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    (
        test_monkeys[0].inspection_count * test_monkeys[1].inspection_count,
        big_test_monkeys[0].inspection_count * big_test_monkeys[1].inspection_count,
    )
}

#[cfg(test)]
mod tests {
    use crate::day11::day11;

    #[test]
    fn test_case() {
        assert_eq!(day11("src/test_data/day11.txt"), (10605, 2713310158));
    }
}
