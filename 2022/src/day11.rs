use std::collections::VecDeque;
use std::path::Path;

#[derive(Debug)]
struct MonkeyState {
    items: VecDeque<i32>,
    inspection_count: i32,
}
impl MonkeyState {
    fn get_next_item(&mut self) -> Option<i32> {
        let item = self.items.pop_front();
        if item.is_some() {
            self.inspection_count += 1;
        }

        item
    }
    fn add_item(&mut self, item: i32) -> () {
        self.items.push_back(item);
    }
}

trait MonkeyBuisness {
    // returns (monkey_index, item_value)
    fn inspect(&mut self) -> Option<(usize, i32)>;
}

#[derive(Debug)]
struct TestMonkey0 {
    state: MonkeyState,
}
impl MonkeyBuisness for TestMonkey0 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item * 19, 23, 2, 3))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct TestMonkey1 {
    state: MonkeyState,
}
impl MonkeyBuisness for TestMonkey1 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 6, 19, 2, 0))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct TestMonkey2 {
    state: MonkeyState,
}
impl MonkeyBuisness for TestMonkey2 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item * item, 13, 1, 3))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct TestMonkey3 {
    state: MonkeyState,
}
impl MonkeyBuisness for TestMonkey3 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 3, 17, 0, 1))
        } else {
            None
        }
    }
}

struct Monkey0 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey0 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item * 5, 3, 7, 4))
        } else {
            None
        }
    }
}
struct Monkey1 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey1 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 6, 17, 3, 0))
        } else {
            None
        }
    }
}

struct Monkey2 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey2 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 5, 2, 3, 1))
        } else {
            None
        }
    }
}

struct Monkey3 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey3 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 2, 19, 7, 0))
        } else {
            None
        }
    }
}

struct Monkey4 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey4 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item * 7, 11, 5, 6))
        } else {
            None
        }
    }
}

struct Monkey5 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey5 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 7, 5, 2, 1))
        } else {
            None
        }
    }
}

struct Monkey6 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey6 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item + 1, 13, 5, 2))
        } else {
            None
        }
    }
}

struct Monkey7 {
    state: MonkeyState,
}
impl MonkeyBuisness for Monkey7 {
    fn inspect(&mut self) -> Option<(usize, i32)> {
        if let Some(item) = self.state.get_next_item() {
            Some(get_value_and_target(item * item, 7, 4, 6))
        } else {
            None
        }
    }
}

fn get_value_and_target(
    result_of_monkey_calculation: i32,
    test_divisibility_with: i32,
    true_index: usize,
    false_index: usize,
) -> (usize, i32) {
    let new_value = adjust_value_for_relief(result_of_monkey_calculation);
    if new_value % test_divisibility_with == 0 {
        (true_index, new_value)
    } else {
        (false_index, new_value)
    }
}

fn adjust_value_for_relief(item: i32) -> i32 {
    ((item / 3) as f64).floor() as i32
}

fn test_monkeys() -> (TestMonkey0, TestMonkey1, TestMonkey2, TestMonkey3) {
    (
        TestMonkey0 {
            state: MonkeyState {
                items: VecDeque::from([79, 98]),
                inspection_count: 0,
            },
        },
        TestMonkey1 {
            state: MonkeyState {
                items: VecDeque::from([54, 65, 75, 74]),
                inspection_count: 0,
            },
        },
        TestMonkey2 {
            state: MonkeyState {
                items: VecDeque::from([79, 60, 97]),
                inspection_count: 0,
            },
        },
        TestMonkey3 {
            state: MonkeyState {
                items: VecDeque::from([74]),
                inspection_count: 0,
            },
        },
    )
}

fn monkeys() -> (
    Monkey0,
    Monkey1,
    Monkey2,
    Monkey3,
    Monkey4,
    Monkey5,
    Monkey6,
    Monkey7,
) {
    (
        Monkey0 {
            state: MonkeyState {
                items: VecDeque::from([66, 71, 94]),
                inspection_count: 0,
            },
        },
        Monkey1 {
            state: MonkeyState {
                items: VecDeque::from([70]),
                inspection_count: 0,
            },
        },
        Monkey2 {
            state: MonkeyState {
                items: VecDeque::from([62, 68, 56, 65, 94, 78]),
                inspection_count: 0,
            },
        },
        Monkey3 {
            state: MonkeyState {
                items: VecDeque::from([89, 94, 94, 67]),
                inspection_count: 0,
            },
        },
        Monkey4 {
            state: MonkeyState {
                items: VecDeque::from([71, 61, 73, 65, 98, 98, 63]),
                inspection_count: 0,
            },
        },
        Monkey5 {
            state: MonkeyState {
                items: VecDeque::from([55, 62, 68, 61, 60]),
                inspection_count: 0,
            },
        },
        Monkey6 {
            state: MonkeyState {
                items: VecDeque::from([93, 91, 69, 64, 72, 89, 50, 71]),
                inspection_count: 0,
            },
        },
        Monkey7 {
            state: MonkeyState {
                items: VecDeque::from([76, 50]),
                inspection_count: 0,
            },
        },
    )
}

fn test_round(monkeys: &mut (TestMonkey0, TestMonkey1, TestMonkey2, TestMonkey3)) {
    while let Some((target, value)) = monkeys.0.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.1.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.2.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.3.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            _ => unreachable!(),
        }
    }
}

fn round(monkeys: &mut (Monkey0, Monkey1, Monkey2, Monkey3, Monkey4, Monkey5, Monkey6, Monkey7)) {
    while let Some((target, value)) = monkeys.0.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.1.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.2.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.3.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.4.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.5.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.6.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
    while let Some((target, value)) = monkeys.7.inspect() {
        match target {
            0 => monkeys.0.state.add_item(value),
            1 => monkeys.1.state.add_item(value),
            2 => monkeys.2.state.add_item(value),
            3 => monkeys.3.state.add_item(value),
            4 => monkeys.4.state.add_item(value),
            5 => monkeys.5.state.add_item(value),
            6 => monkeys.6.state.add_item(value),
            7 => monkeys.7.state.add_item(value),
            _ => unreachable!(),
        }
    }
}

pub fn day11<P: AsRef<Path>>(_file_path: P) -> (i32, i32) {
    let mut test_monkeys = test_monkeys();

    for _ in 0..20 {
        test_round(&mut test_monkeys);
    }

    let mut test_counts = [
        test_monkeys.0.state.inspection_count,
        test_monkeys.1.state.inspection_count,
        test_monkeys.2.state.inspection_count,
        test_monkeys.3.state.inspection_count,
    ];
    test_counts.sort_by(|a, b| b.cmp(a));

    let mut monkeys = monkeys();

    for _ in 0..20 {
        round(&mut monkeys);
    }

    let mut counts = [
        monkeys.0.state.inspection_count,
        monkeys.1.state.inspection_count,
        monkeys.2.state.inspection_count,
        monkeys.3.state.inspection_count,
        monkeys.4.state.inspection_count,
        monkeys.5.state.inspection_count,
        monkeys.6.state.inspection_count,
        monkeys.7.state.inspection_count,
    ];
    counts.sort_by(|a, b| b.cmp(a));

    (test_counts[0] * test_counts[1], counts[0] * counts[1])
}

#[cfg(test)]
mod tests {
    use crate::day11::day11;

    #[test]
    fn test_case() {
        assert_eq!(day11("src/test_data/day11.txt"), (10605, 0));
    }
}
