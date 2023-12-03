use std::{collections::HashSet, vec};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Number {
    chars: Vec<char>,
    indices: Vec<usize>,
}

struct Symbol {
    index: usize,
}

impl Number {
    fn to_digit(&self) -> u32 {
        self.chars.iter().collect::<String>().parse().unwrap()
    }
}

fn process(input: &str) -> u32 {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let n_col = input.lines().next().unwrap().len();

    for (row_idx, line) in input.lines().enumerate() {
        let mut number = Number {
            chars: vec![],
            indices: vec![],
        };
        for (col_idx, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                number.chars.push(character);
                number.indices.push(row_idx * n_col + col_idx);
            } else {
                if character != '.' {
                    symbols.push(Symbol {
                        index: row_idx * n_col + col_idx,
                    });
                }
                if !number.chars.is_empty() {
                    numbers.push(number);
                }
                number = Number {
                    chars: vec![],
                    indices: vec![],
                };
            }
        }
        if !number.chars.is_empty() {
            numbers.push(number);
        }

    }
    
    let mut adjacent_numbers: Vec<&Number> = vec![];
    for number in &numbers {
        let mut unique_neighbor_indices: HashSet<usize> = HashSet::new();
        for idx in &number.indices {
            let neighbor_indices: Vec<i32> = vec![
                *idx as i32 - 1,
                *idx as i32 + 1,
                *idx as i32 - n_col as i32,
                *idx as i32 + n_col as i32,
                *idx as i32 - n_col as i32 - 1,
                *idx as i32 - n_col as i32 + 1,
                *idx as i32 + n_col as i32 - 1,
                *idx as i32 + n_col as i32 + 1,
            ];
            neighbor_indices.into_iter().for_each(|idx| {
                if idx >= 0 {
                    unique_neighbor_indices.insert(idx as usize);
                }
            });
        }
        if symbols
            .iter()
            .any(|symbol| unique_neighbor_indices.contains(&symbol.index))
        {
            adjacent_numbers.push(number);
        }
    }
    adjacent_numbers
        .iter()
        .map(|number| number.to_digit())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 4361);
    }
}
