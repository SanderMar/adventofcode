use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Item = usize;
type Monkeys = Vec<Monkey>;

pub struct Monkey {
    items: VecDeque<Item>,
    operation: (String, String),
    test_value: usize,
    receiver_true: usize,
    receiver_false: usize,
}
impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: ("+".to_owned(), "old".to_owned()),
            test_value: 1,
            receiver_true: 0,
            receiver_false: 0,
        }
    }

    fn get_value(input: &str, old: Item) -> Item {
        input.parse::<Item>().unwrap_or(old)
    }

    fn inspect_item(&self, item: Item) -> Item {
        let mut val: Item = 0;
        if self.operation.0 == "+" {
            val = item + Monkey::get_value(&self.operation.1, item);
        } else if self.operation.0 == "*" {
            val = item * Monkey::get_value(&self.operation.1, item);
        }

        val / 3
    }

    fn throw_item(&mut self) -> (usize, Item) {
        let item: Item = self.items.pop_front().unwrap_or(0);
        let worry: Item = self.inspect_item(item);
        if worry % self.test_value == 0 {
            return (self.receiver_true, worry);
        } else {
            return (self.receiver_false, worry);
        }
    }
}

fn throw_items(monkeys: &mut Monkeys, m_i: usize) {
    while !monkeys[m_i].items.is_empty() {
        let (receiver, item) = monkeys[m_i].throw_item();
        monkeys[receiver].items.push_back(item);
    }
}

#[aoc_generator(day11)]
pub fn gen_day11(input: &str) -> Monkeys {
    let mut monkeys = Monkeys::new();

    input.split("\n\n")
        .for_each(|monkey_str| {monkeys.push(construct_monkey(monkey_str))});

    monkeys
}

pub fn construct_monkey(monkey_str: &str) -> Monkey {
    let mut monkey = Monkey::new();
    let splitted = monkey_str.split("\n");

    for (i, str) in splitted.enumerate() {
        if i == 0 {
            continue;
        }
        let vals: &str = str.split(":").collect::<Vec<&str>>()[1];
        let vals = vals.trim();
        if i == 1 {
            vals.split(",")
                .for_each(|val| monkey.items.push_back(val.trim().parse::<Item>().unwrap()));
        } else if i == 2 {
            let operations: Vec<&str> = vals.split(" ").collect();
            let len_op = operations.len();
            monkey.operation = (
                operations[len_op - 2].to_owned(),
                operations[len_op - 1].to_owned(),
            );
        } else if i == 3 {
            let operations: Vec<&str> = vals.split(" ").collect();
            monkey.test_value = operations.last().unwrap().parse().unwrap();
        } else if i == 4 {
            let operations: Vec<&str> = vals.split(" ").collect();
            monkey.receiver_true = operations.last().unwrap().parse().unwrap();
        } else if i == 5 {
            let operations: Vec<&str> = vals.split(" ").collect();
            monkey.receiver_false = operations.last().unwrap().parse().unwrap();
        }
    }
    monkey
}

#[cfg(test)]
mod monkey_business {
    use super::*;

    #[test]
    fn get_value() {
        let val1 = Monkey::get_value("20", 10);
        let val2 = Monkey::get_value("old", 10);

        assert_eq!(val1, 20);
        assert_eq!(val2, 10);
    }
    #[test]
    fn inspect_item_plus() {
        let monkey = Monkey {
            items: VecDeque::new(),
            operation: ("+".to_owned(), "10".to_owned()),
            test_value: 20,
            receiver_true: 0,
            receiver_false: 0,
        };
        assert_eq!(monkey.inspect_item(2), 4);
    }
    #[test]
    fn inspect_item_times() {
        let monkey = Monkey {
            items: VecDeque::new(),
            operation: ("*".to_owned(), "10".to_owned()),
            test_value: 20,
            receiver_true: 0,
            receiver_false: 0,
        };
        assert_eq!(monkey.inspect_item(2), 6);
    }
}
