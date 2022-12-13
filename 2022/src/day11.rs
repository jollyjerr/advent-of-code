use std::collections::VecDeque;
use std::path::Path;

enum Monkey {
    Test0 {
        items: VecDeque<i32>,
        inspection_count: i32,
    },
    Test1 {
        items: VecDeque<i32>,
        inspection_count: i32,
    },
    Test2 {
        items: VecDeque<i32>,
        inspection_count: i32,
    },
    Test3 {
        items: VecDeque<i32>,
        inspection_count: i32,
    },
}

impl Monkey {
    pub fn get_items(&self) -> &VecDeque<i32> {
        match self {
            Monkey::Test0 { items, .. } => items,
            Monkey::Test1 { items, .. } => items,
            Monkey::Test2 { items, .. } => items,
            Monkey::Test3 { items, .. } => items,
        }
    }

    pub fn inspect(&self, item: i32, monkeys: Vec<Monkey>) {
        match *self {
            Monkey::Test0 {
                inspection_count, ..
            } => {
                inspection_count += 1;
                let worry_level = (((item * 19) / 3) as f64).floor() as i32;
                if worry_level % 23 == 0 {
                    monkeys[2].get_items().push_back(worry_level);
                } else {
                    monkeys[3].get_items().push_back(worry_level);
                }
            }
            Monkey::Test1 {
                inspection_count, ..
            } => {
                inspection_count += 1;
                let worry_level = (((item + 6) / 3) as f64).floor() as i32;
                if worry_level % 19 == 0 {
                    monkeys[2].get_items().push_back(worry_level);
                } else {
                    monkeys[0].get_items().push_back(worry_level);
                }
            }
            Monkey::Test2 {
                inspection_count, ..
            } => {
                inspection_count += 1;
                let worry_level = (((item * item) / 3) as f64).floor() as i32;
                if worry_level % 13 == 0 {
                    monkeys[1].get_items().push_back(worry_level);
                } else {
                    monkeys[3].get_items().push_back(worry_level);
                }
            }
            Monkey::Test3 {
                inspection_count, ..
            } => {
                inspection_count += 1;
                let worry_level = (((item + 3) / 3) as f64).floor() as i32;
                if worry_level % 17 == 0 {
                    monkeys[0].get_items().push_back(worry_level);
                } else {
                    monkeys[1].get_items().push_back(worry_level);
                }
            }
        }
    }
}

pub fn day11<P: AsRef<Path>>(_file_path: P) -> (i32, i32) {
    (0, 0)
}

fn test_case_monkeys() -> [Monkey; 4] {
    [
        Monkey::Test0 {
            items: VecDeque::from([79, 98]),
            inspection_count: 0,
        },
        Monkey::Test1 {
            items: VecDeque::from([54, 65, 75, 74]),
            inspection_count: 0,
        },
        Monkey::Test2 {
            items: VecDeque::from([79, 60, 97]),
            inspection_count: 0,
        },
        Monkey::Test3 {
            items: VecDeque::from([74]),
            inspection_count: 0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::day11::day11;

    #[test]
    fn test_case() {
        assert_eq!(day11("src/test_data/day11.txt"), (13140, 0));
    }
}
