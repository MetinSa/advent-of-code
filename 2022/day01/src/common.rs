use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn sum_chunks(input: &str) -> Vec<i32> {
    let mut sum = 0;
    let mut sums = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums
}
