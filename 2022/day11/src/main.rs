use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fs;

fn main() {
    const N_ROUNDS: usize = 10000;

    let input = fs::read_to_string("input.txt").unwrap();

    let mut monkies = parse_initial_monkey_config(&input);

    let modulus: i64 = monkies.iter().map(|monkey| monkey.divide_by).product();

    for _ in 0..N_ROUNDS {
        for i in 0..monkies.len() {
            while !monkies[i].items.is_empty() {
                let item = monkies[i].items.pop_front().unwrap();
                let argument = match monkies[i].operation.1.borrow() {
                    "old" => item,
                    _ => monkies[i].operation.1.parse().unwrap(),
                };

                let mut updated_item = match monkies[i].operation.0.borrow() {
                    "+" => item + argument,
                    "-" => item - argument,
                    "*" => item * argument,
                    other => {
                        panic!("Unknown operator type {}", other)
                    }
                };
                updated_item %= modulus;

                // Part 1)
                // let new_worry_level = (updated_item as f64/3f64) as i64;

                // Part 2)
                let new_worry_level = updated_item;

                let throw_to: usize = if new_worry_level % monkies[i].divide_by == 0 {
                    monkies[i].throw_to.0
                } else {
                    monkies[i].throw_to.1
                };
                monkies[throw_to].items.push_back(updated_item);
                monkies[i].inspected_n_items += 1;
            }
        }
    }
    monkies.sort_by_key(|x| x.inspected_n_items);
    monkies.reverse();
    println!(
        "{}",
        monkies[0].inspected_n_items * monkies[1].inspected_n_items
    )
}

struct Monkey {
    items: VecDeque<i64>,
    operation: (String, String),
    divide_by: i64,
    throw_to: (usize, usize),
    inspected_n_items: i64,
}

fn parse_initial_monkey_config(input: &str) -> Vec<Monkey> {
    let mut monkies: Vec<Monkey> = Vec::new();
    let mut lines = input.lines();
    loop {
        let line = match lines.next() {
            Some(value) => value,
            None => break,
        };
        if line.is_empty() {
            continue;
        }
        let start_items: VecDeque<i64> = lines
            .next()
            .unwrap()
            .trim()
            .replace("Starting items: ", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let operation_str: Vec<String> = lines
            .next()
            .unwrap()
            .replace("  Operation: new = old ", "")
            .split(" ")
            .map(|x| x.to_string())
            .collect();

        let (operator, argument) = (&operation_str[0], &operation_str[1]);

        let divide_by: i64 = lines
            .next()
            .unwrap()
            .replace("  Test: divisible by ", "")
            .parse()
            .unwrap();

        let throw_to_true: usize = lines
            .next()
            .unwrap()
            .replace("    If true: throw to monkey ", "")
            .parse()
            .unwrap();

        let throw_to_false: usize = lines
            .next()
            .unwrap()
            .replace("    If false: throw to monkey ", "")
            .parse()
            .unwrap();

        monkies.push(Monkey {
            items: start_items,
            operation: (operator.to_string(), argument.to_string()),
            divide_by: divide_by,
            throw_to: (throw_to_true, throw_to_false),
            inspected_n_items: 0,
        });
    }
    monkies
}
