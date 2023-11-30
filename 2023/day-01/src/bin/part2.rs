fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    "todo!()".to_string()
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("");
        assert_eq!(result, "todo!()".to_string());
    }
}