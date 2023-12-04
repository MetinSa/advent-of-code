use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

impl Card {
    fn matches(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }
}

fn process(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().fold(vec![], |mut acc, line| {
        let numbers: HashSet<usize> = line
            .split("|")
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let winning_numbers: HashSet<usize> = line
            .split("|")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        acc.push(Card {
            numbers,
            winning_numbers,
        });
        acc
    });

    let total_cards: BTreeMap<usize, usize> = (1..=cards.len()).map(|x| (x, 1)).collect();
    
    cards
        .iter()
        .map(|x| x.matches())
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
        .fold(total_cards, |mut acc, (idx, card_id)| {
            for i in (idx + 2)..(idx + 2 + card_id) {
                let copies = *acc.get(&(idx + 1)).unwrap();
                acc.entry(i).and_modify(|value| *value += copies);
            }
            acc
        })
        .values()
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 30);
    }
}
