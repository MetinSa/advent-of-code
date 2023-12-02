fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    input
        .lines()
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
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(process(input), 142);
    }
}
