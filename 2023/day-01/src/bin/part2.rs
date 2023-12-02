fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let number_map = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            for (number, replacement) in number_map.iter() {
                line = line.replace(number, replacement);
            }
            line
        })
        .map(|line| {
            let mut iterator = line.chars().filter_map(|character| character.to_digit(10));
            let first = iterator.next().unwrap();
            let last = match iterator.last() {
                Some(number) => number,
                None => first,
            };
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!(process(input), 281);
    }
}
