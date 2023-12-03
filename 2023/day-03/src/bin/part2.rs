fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Number {
    chars: Vec<char>,
    indices: Vec<usize>,
}

#[derive(Debug)]
struct Symbol {
    character: char,
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
                        character,
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

    let mut score = 0;
    let symbols: Vec<Symbol> = symbols
        .into_iter()
        .filter(|symbol| symbol.character == '*')
        .collect();
    for symbol in symbols.iter() {
        let mut adjacent_numbers: Vec<&Number> = vec![];
        let neighbor_indices = vec![
            symbol.index - 1,
            symbol.index + 1,
            symbol.index - n_col,
            symbol.index + n_col,
            symbol.index - n_col - 1,
            symbol.index - n_col + 1,
            symbol.index + n_col - 1,
            symbol.index + n_col + 1,
        ];
        for number in &numbers {
            if neighbor_indices
                .iter()
                .any(|idx| number.indices.contains(idx))
            {
                adjacent_numbers.push(number);
            }
        }
        if adjacent_numbers.len() == 2 {
            score += adjacent_numbers
                .into_iter()
                .map(|number| number.to_digit())
                .product::<u32>();
        }
    }
    score
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
        assert_eq!(process(input), 467835);
    }
}
